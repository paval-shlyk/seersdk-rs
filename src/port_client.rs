use bytes::BytesMut;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, Notify};
use tracing::{debug, error};

use crate::error::{RbkError, RbkResult};
use crate::protocol::{RbkDecoder, encode_request};

/// Client for a specific RBK port
pub(crate) struct RbkPortClient {
    host: String,
    port: u16,
    state: Arc<Mutex<ClientState>>,
}

struct ClientState {
    connection: Option<Connection>,
    flow_no_counter: u16,
    response_map: HashMap<u16, String>,
    notify: Arc<Notify>,
    disposed: bool,
}

struct Connection {
    stream: TcpStream,
    read_task: tokio::task::JoinHandle<()>,
}

impl RbkPortClient {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            state: Arc::new(Mutex::new(ClientState {
                connection: None,
                flow_no_counter: 0,
                response_map: HashMap::new(),
                notify: Arc::new(Notify::new()),
                disposed: false,
            })),
        }
    }

    pub async fn request(
        &self,
        api_no: u16,
        req_str: &str,
        timeout: Duration,
    ) -> RbkResult<String> {
        // Try up to 3 times with automatic reconnection
        const MAX_RETRIES: usize = 3;
        let mut last_error = None;

        for attempt in 0..MAX_RETRIES {
            let result = self.do_request(api_no, req_str, timeout).await;

            match result {
                Ok(response) => return Ok(response),
                Err(e) => {
                    debug!(
                        "Request failed (API {}, attempt {}/{}): {:?}",
                        api_no, attempt + 1, MAX_RETRIES, e
                    );
                    
                    // Reset connection on error
                    self.reset().await;
                    
                    last_error = Some(e);
                    
                    // If not the last attempt, wait briefly before retry
                    if attempt + 1 < MAX_RETRIES {
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                }
            }
        }

        // Return the last error after all retries exhausted
        Err(last_error.unwrap())
    }

    //fixme: not cancel-safe due to the timeout
    async fn do_request(
        &self,
        api_no: u16,
        req_str: &str,
        timeout: Duration,
    ) -> RbkResult<String> {
        let mut state = self.state.lock().await;

        // Ensure connection - will reconnect if disposed or disconnected
        if state.connection.is_none() || state.disposed {
            drop(state);
            self.connect().await?;
            state = self.state.lock().await;
        }

        let flow_no = state.next_flow_no();
        let notify = state.notify.clone();

        // Encode and send request
        let request_bytes = encode_request(api_no, req_str, flow_no);

        if let Some(ref mut conn) = state.connection {
            conn.stream.write_all(&request_bytes).await.map_err(|e| {
                error!("Write error for API {}: {}", api_no, e.kind());
                RbkError::WriteError(e.to_string())
            })?;
        }

        drop(state);

        // Wait for response with timeout
        tokio::time::timeout(timeout, async {
            loop {
                notify.notified().await;
                let mut state = self.state.lock().await;

                // If disposed during wait, return error to trigger reconnection
                if state.disposed {
                    return Err(RbkError::Disposed);
                }

                if let Some(res_str) = state.response_map.remove(&flow_no) {
                    return Ok(res_str);
                }
            }
        })
        .await
        .map_err(|_| RbkError::Timeout)?
    }

    async fn connect(&self) -> RbkResult<()> {
        let addr = format!("{}:{}", self.host, self.port);
        let stream = tokio::time::timeout(
            Duration::from_secs(10),
            TcpStream::connect(&addr),
        )
        .await
        .map_err(|_| RbkError::Timeout)?
        .map_err(|e| RbkError::ConnectionFailed(e.to_string()))?;

        let state_clone = self.state.clone();
        let read_task = tokio::spawn(async move {
            read_loop(state_clone).await;
        });

        let mut state = self.state.lock().await;
        state.connection = Some(Connection { stream, read_task });
        state.disposed = false;

        Ok(())
    }

    async fn reset(&self) {
        let mut state = self.state.lock().await;
        state.response_map.clear();
        state.disposed = true;

        if let Some(mut conn) = state.connection.take() {
            conn.read_task.abort();
            let _ = conn.stream.shutdown().await;
        }

        state.notify.notify_waiters();
    }
}

impl Drop for RbkPortClient {
    fn drop(&mut self) {
        // Note: Drop cannot be async in Rust, and proper cleanup requires async operations.
        // The read task will be automatically aborted when the JoinHandle is dropped.
        // The TCP connection will be closed when the TcpStream is dropped.
        // This provides automatic cleanup, though it's not as graceful as calling reset().
    }
}

impl ClientState {
    fn next_flow_no(&mut self) -> u16 {
        self.flow_no_counter = (self.flow_no_counter + 1) % 512;
        self.flow_no_counter
    }
}

async fn read_loop(state: Arc<Mutex<ClientState>>) {
    let mut decoder = RbkDecoder::new();
    let mut buf = BytesMut::with_capacity(4096);
    let mut read_buf = vec![0u8; 4096];

    loop {
        // Get a mutable reference to the stream
        let mut state_lock = state.lock().await;

        let has_connection = state_lock.connection.is_some();
        if !has_connection {
            break;
        }

        // Take ownership of the stream temporarily
        let mut conn = match state_lock.connection.take() {
            Some(c) => c,
            None => break,
        };
        drop(state_lock);

        // Read from stream without holding the lock
        match conn.stream.read(&mut read_buf).await {
            Ok(0) => {
                // Connection closed
                break;
            }
            Ok(n) => {
                buf.extend_from_slice(&read_buf[..n]);

                // Process all complete frames
                while let Some(frame) = decoder.decode(&mut buf) {
                    let mut state = state.lock().await;
                    state.response_map.insert(frame.flow_no, frame.body);
                    state.notify.notify_waiters();
                }

                // Put the stream back
                let mut state = state.lock().await;
                state.connection = Some(conn);
            }
            Err(e) => {
                error!("Read error: {}", e);
                break;
            }
        }
    }
}

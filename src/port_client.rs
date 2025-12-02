use bytes::BytesMut;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, Notify};
use tokio::time::timeout;
use tracing::{debug, error};

use crate::error::{RbkError, RbkResult};
use crate::frame::RbkResultKind;
use crate::protocol::{encode_request, RbkDecoder};
use crate::RbkRequestResult;

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
        api_no: i32,
        req_str: &str,
        timeout_ms: u64,
    ) -> RbkResult<RbkRequestResult> {
        let result = self.do_request(api_no, req_str, timeout_ms).await;

        // Reset on error
        if let Ok(ref res) = result {
            if res.kind != RbkResultKind::Ok {
                debug!("Request failed, resetting client: {:?}", res.kind);
                self.reset().await;
            }
        }

        result
    }

    async fn do_request(
        &self,
        api_no: i32,
        req_str: &str,
        timeout_ms: u64,
    ) -> RbkResult<RbkRequestResult> {
        let mut state = self.state.lock().await;

        if state.disposed {
            return Ok(RbkRequestResult::new(
                RbkResultKind::Disposed,
                self.host.clone(),
                api_no,
                req_str.to_string(),
            ));
        }

        // Ensure connection
        if state.connection.is_none() {
            drop(state);
            self.connect().await?;
            state = self.state.lock().await;
        }

        let flow_no = state.next_flow_no();
        let notify = state.notify.clone();

        // Validate API number fits in u16
        if api_no < 0 || api_no > 65535 {
            return Ok(RbkRequestResult::new(
                RbkResultKind::BadApiNo,
                self.host.clone(),
                api_no,
                req_str.to_string(),
            )
            .with_error(format!("API number {} out of valid range", api_no)));
        }

        // Encode and send request
        let request_bytes = encode_request(api_no as u16, req_str, flow_no);

        if let Some(ref mut conn) = state.connection {
            if let Err(e) = conn.stream.write_all(&request_bytes).await {
                error!("Write error: {}", e);
                return Ok(RbkRequestResult::new(
                    RbkResultKind::WriteError,
                    self.host.clone(),
                    api_no,
                    req_str.to_string(),
                )
                .with_error(e.to_string()));
            }
        }

        drop(state);

        // Wait for response with timeout
        let timeout_duration = Duration::from_millis(timeout_ms);
        match timeout(timeout_duration, async {
            loop {
                notify.notified().await;
                let mut state = self.state.lock().await;

                if state.disposed {
                    return RbkRequestResult::new(
                        RbkResultKind::Disposed,
                        self.host.clone(),
                        api_no,
                        req_str.to_string(),
                    );
                }

                if let Some(res_str) = state.response_map.remove(&flow_no) {
                    return RbkRequestResult::new(
                        RbkResultKind::Ok,
                        self.host.clone(),
                        api_no,
                        req_str.to_string(),
                    )
                    .with_response(res_str);
                }
            }
        })
        .await
        {
            Ok(result) => Ok(result),
            Err(_) => Ok(RbkRequestResult::new(
                RbkResultKind::Timeout,
                self.host.clone(),
                api_no,
                req_str.to_string(),
            )
            .with_error("Timeout".to_string())),
        }
    }

    async fn connect(&self) -> RbkResult<()> {
        let addr = format!("{}:{}", self.host, self.port);
        let stream = timeout(Duration::from_secs(10), TcpStream::connect(&addr))
            .await
            .map_err(|_| RbkError::Timeout)?
            .map_err(|e| RbkError::ConnectionFailed(e.to_string()))?;

        let state_clone = self.state.clone();
        let read_task = tokio::spawn(async move {
            Self::read_loop(state_clone).await;
        });

        let mut state = self.state.lock().await;
        state.connection = Some(Connection { stream, read_task });
        state.disposed = false;

        Ok(())
    }

    async fn read_loop(state: Arc<Mutex<ClientState>>) {
        let mut decoder = RbkDecoder::new();
        let mut buf = BytesMut::with_capacity(4096);
        let mut read_buf = vec![0u8; 4096];

        loop {
            // Get a mutable reference to the stream
            let mut stream_guard = state.lock().await;
            
            let has_connection = stream_guard.connection.is_some();
            if !has_connection {
                break;
            }
            
            // Take ownership of the stream temporarily
            let mut conn = match stream_guard.connection.take() {
                Some(c) => c,
                None => break,
            };
            drop(stream_guard);

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
                        state.response_map.insert(frame.flow_no, frame.body_str);
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

    pub async fn dispose(&self) {
        self.reset().await;
    }
}

impl ClientState {
    fn next_flow_no(&mut self) -> u16 {
        self.flow_no_counter = (self.flow_no_counter + 1) % 512;
        self.flow_no_counter
    }
}

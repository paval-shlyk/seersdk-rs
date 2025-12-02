use crate::error::{RbkError, RbkResult};
use crate::port_client::RbkPortClient;
use crate::RbkRequestResult;
use std::time::Duration;

// Port constants for different API categories
const STATE_PORT: u16 = 19204;
const CONTROL_PORT: u16 = 19205;
const NAV_PORT: u16 = 19206;
const CONFIG_PORT: u16 = 19207;
const KERNEL_PORT: u16 = 19208;
const MISC_PORT: u16 = 19210;

/// Main RBK client for communicating with robots
///
/// This client manages multiple port clients for different API categories:
/// - State APIs (1000-1999): port 19204
/// - Control APIs (2000-2999): port 19205
/// - Navigation APIs (3000-3999): port 19206
/// - Config APIs (4000-5999): port 19207
/// - Kernel APIs (7000-7999): port 19208
/// - Misc APIs (6000-6998): port 19210
pub struct RbkClient {
    #[allow(dead_code)]
    host: String,
    config_client: RbkPortClient,
    misc_client: RbkPortClient,
    state_client: RbkPortClient,
    control_client: RbkPortClient,
    nav_client: RbkPortClient,
    kernel_client: RbkPortClient,
}

impl RbkClient {
    /// Create a new RBK client for the given host
    ///
    /// # Arguments
    ///
    /// * `host` - The IP address or hostname of the robot
    ///
    /// # Example
    ///
    /// ```no_run
    /// use seersdk_rs::RbkClient;
    ///
    /// let client = RbkClient::new("192.168.8.114");
    /// ```
    pub fn new(host: impl Into<String>) -> Self {
        let host = host.into();
        Self {
            config_client: RbkPortClient::new(host.clone(), CONFIG_PORT),
            misc_client: RbkPortClient::new(host.clone(), MISC_PORT),
            state_client: RbkPortClient::new(host.clone(), STATE_PORT),
            control_client: RbkPortClient::new(host.clone(), CONTROL_PORT),
            nav_client: RbkPortClient::new(host.clone(), NAV_PORT),
            kernel_client: RbkPortClient::new(host.clone(), KERNEL_PORT),
            host,
        }
    }

    /// Send a request to the robot
    ///
    /// # Arguments
    ///
    /// * `api_no` - The API number (determines which port to use)
    /// * `request_str` - The request body as a JSON string
    /// * `timeout` - Timeout duration (defaults to 10 seconds if zero)
    ///
    /// # Returns
    ///
    /// Returns `RbkRequestResult` with the response or error information
    ///
    /// # Example
    ///
    /// ```no_run
    /// use seersdk_rs::{RbkClient, RbkResultKind};
    /// use std::time::Duration;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RbkClient::new("192.168.8.114");
    /// let result = client.request(1007, r#"{"simple": true}"#, Duration::from_secs(10)).await?;
    ///
    /// if result.kind == RbkResultKind::Ok {
    ///     println!("Battery level response: {}", result.res_str);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn request(
        &self,
        api_no: i32,
        request_str: &str,
        timeout: Duration,
    ) -> RbkResult<RbkRequestResult> {
        let timeout = if timeout.is_zero() {
            Duration::from_secs(10)
        } else {
            timeout
        };

        match api_no {
            1000..=1999 => self.state_client.request(api_no, request_str, timeout).await,
            2000..=2999 => self.control_client.request(api_no, request_str, timeout).await,
            3000..=3999 => self.nav_client.request(api_no, request_str, timeout).await,
            4000..=5999 => self.config_client.request(api_no, request_str, timeout).await,
            6000..=6998 => self.misc_client.request(api_no, request_str, timeout).await,
            7000..=7999 => self.kernel_client.request(api_no, request_str, timeout).await,
            _ => Err(RbkError::BadApiNo(api_no)),
        }
    }
}

impl Drop for RbkClient {
    fn drop(&mut self) {
        // Note: Drop cannot be async in Rust, and proper cleanup of TCP connections
        // requires async operations. The connections will be closed when the underlying
        // RbkPortClient instances are dropped, which will abort their read tasks.
        // For graceful shutdown with proper connection cleanup, users should manage
        // the client lifetime explicitly within an async context.
    }
}

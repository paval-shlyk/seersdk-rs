use crate::error::RbkResult;
use crate::frame::RbkResultKind;
use crate::port_client::RbkPortClient;
use crate::RbkRequestResult;

/// Main RBK client for communicating with robots
///
/// This client manages multiple port clients for different API categories:
/// - State APIs (1000-1999): port 19204
/// - Control APIs (2000-2999): port 19205
/// - Navigation APIs (3000-3999): port 19206
/// - Config APIs (4000-5999): port 19207
/// - Misc APIs (6000-6998): port 19210
pub struct RbkClient {
    host: String,
    config_client: RbkPortClient,
    misc_client: RbkPortClient,
    state_client: RbkPortClient,
    control_client: RbkPortClient,
    nav_client: RbkPortClient,
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
            config_client: RbkPortClient::new(host.clone(), 19207),
            misc_client: RbkPortClient::new(host.clone(), 19210),
            state_client: RbkPortClient::new(host.clone(), 19204),
            control_client: RbkPortClient::new(host.clone(), 19205),
            nav_client: RbkPortClient::new(host.clone(), 19206),
            host,
        }
    }

    /// Send a request to the robot
    ///
    /// # Arguments
    ///
    /// * `api_no` - The API number (determines which port to use)
    /// * `request_str` - The request body as a JSON string
    /// * `timeout_ms` - Timeout in milliseconds (use 10000 for 10 seconds)
    ///
    /// # Returns
    ///
    /// Returns `RbkRequestResult` with the response or error information
    ///
    /// # Example
    ///
    /// ```no_run
    /// use seersdk_rs::{RbkClient, RbkResultKind};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RbkClient::new("192.168.8.114");
    /// let result = client.request(1007, r#"{"simple": true}"#, 10000).await?;
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
        timeout_ms: u64,
    ) -> RbkResult<RbkRequestResult> {
        let timeout_ms = if timeout_ms == 0 { 10000 } else { timeout_ms };

        match api_no {
            1000..=1999 => self.state_client.request(api_no, request_str, timeout_ms).await,
            2000..=2999 => self.control_client.request(api_no, request_str, timeout_ms).await,
            3000..=3999 => self.nav_client.request(api_no, request_str, timeout_ms).await,
            4000..=5999 => self.config_client.request(api_no, request_str, timeout_ms).await,
            6000..=6998 => self.misc_client.request(api_no, request_str, timeout_ms).await,
            _ => Ok(RbkRequestResult::new(
                RbkResultKind::BadApiNo,
                self.host.clone(),
                api_no,
                request_str.to_string(),
            )),
        }
    }

    /// Release all connections to the robot
    ///
    /// This should be called when the client is no longer needed
    pub async fn dispose(&self) {
        self.state_client.dispose().await;
        self.control_client.dispose().await;
        self.nav_client.dispose().await;
        self.config_client.dispose().await;
        self.misc_client.dispose().await;
    }
}

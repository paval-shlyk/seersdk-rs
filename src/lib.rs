//! # Seer RBK SDK for Rust
//!
//! A Rust client library for communicating with RBK robots via TCP.
//!
//! ## Example
//!
//! ```no_run
//! use seersdk_rs::{RbkClient, RobotBatteryStatusRequest};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = RbkClient::new("192.168.8.114");
//!     
//!     let request = RobotBatteryStatusRequest::new();
//!     let response = client.request(request, Duration::from_secs(10)).await?;
//!     
//!     println!("Response: {:?}", response);
//!     
//!     Ok(())
//! }
//! ```

mod api;
mod client;
mod error;
mod frame;
mod port_client;
mod protocol;

pub use api::*;
pub use client::RbkClient;
pub use error::{RbkError, RbkResult};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_types_exist() {
        // Test that all major request types are generated and accessible
        let _ = RobotInfoRequest::new();
        let _ = RobotBatteryStatusRequest::new();
        let _ = StopExerciseRequest::new();
        let _ = MoveToTargetRequest::new();
        let _ = SetConfigRequest::new();
        let _ = ShutdownRequest::new();
        let _ = SpeakerRequest::new();
    }

    #[test]
    fn test_request_body_serialization() {
        use crate::api::ToRequestBody;

        // Test request without payload returns empty string
        let request = RobotInfoRequest::new();
        assert_eq!(request.to_request_body().unwrap(), "");

        // Verify all requests have proper API variants
        let api = request.to_api_request();
        assert_eq!(api.api_no(), 1000);
    }

    #[test]
    fn test_response_type_associations() {
        use crate::api::FromResponseBody;

        // Verify response type associations work
        type Response = <RobotInfoRequest as FromResponseBody>::Response;

        // Response should be StatusMessage
        let _: Response = StatusMessage {
            code: ErrorCode::Unavailable,
            message: "test".to_string(),
        };
    }
}

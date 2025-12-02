//! # Seer RBK SDK for Rust
//!
//! A Rust client library for communicating with RBK robots via TCP.
//!
//! ## Example
//!
//! ```no_run
//! use seersdk_rs::{RbkClient, RbkResultKind};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = RbkClient::new("192.168.8.114");
//!     
//!     let result = client.request(1007, r#"{"simple": true}"#, Duration::from_secs(10)).await?;
//!     
//!     if result.kind == RbkResultKind::Ok {
//!         println!("Response: {}", result.res_str);
//!     }
//!     
//!     Ok(())
//! }
//! ```

mod client;
mod error;
mod frame;
mod port_client;
mod protocol;

pub use client::RbkClient;
pub use error::{RbkError, RbkResult};
pub use frame::RbkResultKind;

/// Result of an RBK request
#[derive(Debug, Clone)]
pub struct RbkRequestResult {
    /// Status of the request
    pub kind: RbkResultKind,
    /// IP address of the robot
    pub ip: String,
    /// API number
    pub api_no: i32,
    /// Request string (JSON)
    pub req_str: String,
    /// Response string (JSON)
    pub res_str: String,
    /// Error message if any
    pub err_msg: String,
}

impl RbkRequestResult {
    pub(crate) fn new(
        kind: RbkResultKind,
        ip: String,
        api_no: i32,
        req_str: String,
    ) -> Self {
        Self {
            kind,
            ip,
            api_no,
            req_str,
            res_str: String::new(),
            err_msg: String::new(),
        }
    }

    pub(crate) fn with_response(mut self, res_str: String) -> Self {
        self.res_str = res_str;
        self
    }

    pub(crate) fn with_error(mut self, err_msg: String) -> Self {
        self.err_msg = err_msg;
        self
    }
}

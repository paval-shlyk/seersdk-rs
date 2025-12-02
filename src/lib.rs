//! # Seer RBK SDK for Rust
//!
//! A Rust client library for communicating with RBK robots via TCP.
//!
//! ## Example
//!
//! ```no_run
//! use seersdk_rs::{RbkClient, ApiRequest, StateApi};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = RbkClient::new("192.168.8.114");
//!     
//!     let response = client.request(
//!         ApiRequest::State(StateApi::QueryBattery),
//!         r#"{"simple": true}"#,
//!         Duration::from_secs(10)
//!     ).await?;
//!     
//!     println!("Response: {}", response);
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

pub use api::{ApiRequest, ConfigApi, ControlApi, KernelApi, MiscApi, NavApi, StateApi};
pub use client::RbkClient;
pub use error::{RbkError, RbkResult};

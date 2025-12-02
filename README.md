# Seer RBK SDK for Rust

A Rust client library for communicating with RBK robots via TCP.

## Features

- Async/await support using Tokio
- Type-safe API with enum-based request types
- Automatic connection management
- Multiple port support for different API categories

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
seersdk-rs = "1.0.0"
```

## Usage

```rust
use seersdk_rs::{RbkClient, ApiRequest, StateApi};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client connection to the robot
    let client = RbkClient::new("192.168.8.114");
    
    // Send a request to query the robot's battery level
    // API QueryBattery with parameter {"simple":true}
    let response = client.request(
        ApiRequest::State(StateApi::QueryBattery),
        r#"{"simple": true}"#,
        std::time::Duration::from_secs(10)
    ).await?;
    
    println!("Response: {}", response);
    
    Ok(())
}
```

## API Categories

The RBK protocol uses different ports for different API categories:

- **State APIs** (1000-1999): port 19204 - Robot state queries
- **Control APIs** (2000-2999): port 19205 - Robot control commands
- **Navigation APIs** (3000-3999): port 19206 - Navigation commands
- **Config APIs** (4000-5999): port 19207 - Configuration management
- **Kernel APIs** (7000-7999): port 19208 - Kernel operations
- **Misc APIs** (6000-6998): port 19210 - Miscellaneous operations

## API Request Types

The SDK provides type-safe API requests through the `ApiRequest` enum:

```rust
// State APIs
ApiRequest::State(StateApi::QueryBattery)  // API 1007
ApiRequest::State(StateApi::QueryPose)     // API 1004
ApiRequest::State(StateApi::QuerySpeed)    // API 1005
ApiRequest::State(StateApi::QueryStatus)   // API 1001

// Control APIs
ApiRequest::Control(ControlApi::SetSpeed)  // API 2001
ApiRequest::Control(ControlApi::Stop)      // API 2002

// Navigation APIs
ApiRequest::Nav(NavApi::StartNav)          // API 3001
ApiRequest::Nav(NavApi::StopNav)           // API 3002

// Config APIs
ApiRequest::Config(ConfigApi::GetConfig)   // API 4001
ApiRequest::Config(ConfigApi::SetConfig)   // API 4002

// Custom APIs (for APIs not explicitly defined)
ApiRequest::State(StateApi::Custom(1999))
ApiRequest::Control(ControlApi::Custom(2500))
```

## Examples

See the `examples/` directory for more usage examples:

```bash
cargo run --example battery_query
```

## Note

Most RBK API requests and responses use JSON format.

---

## Legacy Java Client

The original Java client implementation is preserved in the `src/` directory with Maven configuration.

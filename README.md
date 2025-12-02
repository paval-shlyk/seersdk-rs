# Seer RBK SDK for Rust

A Rust client library for communicating with RBK robots via TCP.

## Features

- Async/await support using Tokio
- Type-safe API
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
use seersdk_rs::{RbkClient, RbkResultKind};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client connection to the robot
    let client = RbkClient::new("192.168.8.114");
    
    // Send a request to query the robot's battery level
    // API 1007 queries battery with parameter {"simple":true}
    let result = client.request(1007, r#"{"simple": true}"#, 10000).await?;
    
    if result.kind == RbkResultKind::Ok {
        println!("Response: {}", result.res_str);
    } else {
        println!("Error: {}", result.err_msg);
    }
    
    // Release connection when done
    client.dispose().await;
    
    Ok(())
}
```

## API Categories

The RBK protocol uses different ports for different API categories:

- **State APIs** (1000-1999): port 19204 - Robot state queries
- **Control APIs** (2000-2999): port 19205 - Robot control commands
- **Navigation APIs** (3000-3999): port 19206 - Navigation commands
- **Config APIs** (4000-5999): port 19207 - Configuration management
- **Misc APIs** (6000-6998): port 19210 - Miscellaneous operations

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

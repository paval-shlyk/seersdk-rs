# Seer RBK SDK for Rust

[![CI](https://github.com/paval-shlyk/seersdk-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/paval-shlyk/seersdk-rs/actions/workflows/ci.yml)

A Rust client library for communicating with RBK robots via TCP.

## Features

- Async/await support using Tokio
- Type-safe API with strongly-typed request/response DTOs
- Automatic JSON serialization/deserialization
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
use seersdk_rs::{RbkClient, RobotBatteryStatusRequest};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client connection to the robot
    let client = RbkClient::new("192.168.8.114");
    
    // Create a typed request
    let request = RobotBatteryStatusRequest::new();
    
    // Send the request and get a typed response
    let response = client.request(request, Duration::from_secs(10)).await?;
    
    println!("Response: {:?}", response);
    
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
- **Peripheral APIs** (6000-6998): port 19210 - Peripheralellaneous operations

## API Request Types

The SDK provides type-safe request DTOs for all RBK APIs. Each request type is generated using the `impl_api_request!` macro and implements the `ToRequestBody` and `FromResponseBody` traits.

### State APIs (24 types)

- `RobotInfoRequest` - Query robot information (API 1000)
- `BatteryStatusRequest` - Check battery status (API 1007)
- `RobotLocationRequest` - Query robot location (API 1004)
- `RobotSpeedRequest` - Query robot speed (API 1005)
- And 20 more...

### Control APIs (10 types)

- `StartExerciseRequest` - Start exercising (API 2000)
- `StopExerciseRequest` - Stop exercising (API 2001)
- `RelocateRequest` - Relocate robot (API 2003)
- And 7 more...

### Navigation APIs (8 types)

- `MoveToPointRequest` - Free navigation to a point (API 3050)
- `MoveToTargetRequest` - Fixed path navigation (API 3051)
- `PatrolRequest` - Inspection route (API 3052)
- And 5 more...

### Config APIs (4 types)

- `SwitchModeRequest` - Switch operating mode (API 4000)
- `SetConfigRequest` - Set configuration (API 4001)
- And 2 more...

### Kernel APIs (3 types)

- `ShutdownRequest` - Shutdown robot (API 5000)
- `RebootRequest` - Reboot robot (API 5003)
- `ResetFirmwareRequest` - Reset firmware (API 5005)

### Peripheral APIs

## Examples

See the `examples/` directory for more usage examples:

```bash
cargo run --example battery_query
```

## Documentation

Build the documentation locally:

```bash
cargo doc --open
```

## Note

All RBK API requests and responses use JSON format. The SDK automatically handles serialization and deserialization.

---

## License

This project is licensed under MIT OR Apache-2.0.

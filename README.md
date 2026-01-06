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

### Basic Usage with Real Robot

```rust
use seersdk_rs::{RbkClient, BatteryStatusRequest};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client connection to the robot
    let client = RbkClient::new("192.168.8.114");
    
    // Create a typed request
    let request = BatteryStatusRequest::new();
    
    // Send the request and get a typed response
    let response = client.request(request, Duration::from_secs(10)).await?;
    
    println!("Response: {:?}", response);
    
    Ok(())
}
```

### Development Mode with Mock Server

For development and testing without physical hardware:

1. **Start the mock robot server** (in one terminal):
```bash
cargo run --example mock_robot_server
```

2. **Use the TUI client** (in another terminal):
```bash
cargo run --example tui_client -- localhost
```

Or connect programmatically:
```rust
let client = RbkClient::new("localhost");
// ... use the client as normal
```

## API Categories

The RBK protocol uses different ports for different API categories:

- **State APIs** (1000-1999): port 19204 - Robot state queries (55 variants)
- **Control APIs** (2000-2999): port 19205 - Robot control commands (9 variants)
- **Navigation APIs** (3000-3999): port 19206 - Navigation commands (16 variants)
- **Config APIs** (4000-5999): port 19207 - Configuration management (46 variants)
- **Kernel APIs** (7000-7999): port 19208 - Kernel operations (3 variants)
- **Peripheral APIs** (6000-6998): port 19210 - Peripheral operations (78 variants)
- **Push APIs** (9000+): Push configuration and data (2 variants)

## API Request Types

The SDK provides type-safe request DTOs for all RBK APIs. Each request type is generated using the `impl_api_request!` macro and implements the `ToRequestBody` and `FromResponseBody` traits.

### State APIs (55 variants)

The StateApi enum includes over 55 robot state query operations covering:
- Robot information and status (Info, Run, Loc, Speed, Block, Battery, etc.)
- Sensors and I/O (Laser, Io, Imu, Rfid, Ultrasonic, Encoder, etc.)
- Navigation and mapping (Task, Reloc, LoadMap, Slam, Map, Station, etc.)
- Peripherals (Jack, Fork, Roller, Motor, etc.)
- Robotic arm operations (ArmStatus, ArmCalculate, ArmTask, ArmMove, etc.)
- Scripts and files (ScriptInfo, ListFile, UploadFile, DownloadFile, etc.)
- And many more...

Examples:
- `CommonInfoRequest` - Query robot information (API 1000)
- `BatteryStatusRequest` - Check battery status (API 1007)
- `RobotPoseRequest` - Query robot location (API 1004)

### Control APIs (9 variants)

Control operations including:
- Stop, Reloc, ConfirmLoc (ComfirmLoc in API), CancelReloc
- Motion, LoadMap, ClearMotorEncoder
- UploadAndLoadMap, ClearWeightdevvalue

Examples:
- `StopExerciseRequest` - Stop open loop motion (API 2000)
- `RelocateRequest` - Relocation (API 2002)
- `SwitchMapRequest` - Switch map (API 2022)

### Navigation APIs (16 variants)

Navigation and path planning operations:
- Pause, Resume, Cancel
- MoveToTarget, MoveToTargetList
- Translate, Turn, Spin, Circular
- Path, TargetPath, ClearTargetList, SafeClearMovements
- TaskListStatus, TaskListName, TaskListList

Example:
- `MoveToTargetRequest` - Path navigation (API 3051)

### Config APIs (46 variants)

Configuration management operations including:
- Lock/Unlock control, map management (UploadMap, DownloadMap, RemoveMap)
- Script management (UploadScript, DownloadScript, RemoveScript)
- Parameter management (SetParams, SaveParams, ReloadParams)
- Motor operations (MotorCalib, MotorClearFault)
- Calibration (CalibPushData, CalibConfirm, CalibClear, CalibClearAll)
- Obstacle management (AddObstacle, RemoveObstacle)
- Error and warning handling (SetError, ClearError, SetWarning, ClearWarning)
- And many more...

### Peripheral APIs (78 variants)

Extensive peripheral control operations:
- Audio control (PlayAudio, PauseAudio, ResumeAudio, StopAudio, AudioList)
- Digital I/O (SetDo, SetDos, SetRelay, SetVdi)
- Roller/belt operations (RollerFrontRoll, RollerBackRoll, RollerLeftLoad, etc.)
- Jack operations (JackLoad, JackUnload, JackStop, JackSetHeight)
- Fork operations (SetForkHeight, StopFork)
- Calibration (Calibrate, EndCalibrate, CalibResult)
- SLAM operations (Slam, EndSlam)
- Container and goods management
- And many more...

Examples:
- `LoadJackRequest` - Jacking load (API 6070)
- `UnloadJackRequest` - Jacking unload (API 6071)

### Push APIs (2 variants)

- Push configuration and push data operations

### Kernel APIs (3 variants)

- Shutdown, Reboot, ResetFirmware

## Examples

The `examples/` directory contains several demonstration programs:

### Basic Usage

Query battery status from a real robot:

```bash
cargo run --example battery_query
```

### Mock Robot Server

A standalone binary that emulates a complete RBK robot with mock navigation logic. Perfect for testing and development without physical hardware.

```bash
# Start the mock robot server
cargo run --example mock_robot_server

# The server listens on:
# - Port 19204: State APIs (battery, position, etc.)
# - Port 19205: Control APIs (stop, relocate, etc.)
# - Port 19206: Navigation APIs (move, pause, resume, etc.)
# - Port 19207: Config APIs (parameters, maps, etc.)
# - Port 19208: Kernel APIs (shutdown, reboot)
# - Port 19210: Peripheral APIs (jack, audio, I/O, etc.)
# - Port 8080: HTTP REST API for waypoint management
```

The mock server features:
- Full RBK protocol implementation
- Realistic robot state simulation (battery drain, navigation progress, position updates)
- Support for all major API endpoints
- Concurrent client connections
- **HTTP REST API for waypoint management** (see [WAYPOINT_MANAGEMENT.md](WAYPOINT_MANAGEMENT.md))

#### Default Waypoints

The mock server initializes with three default waypoints:

| ID | X | Y | Description |
|----|---|---|-------------|
| `home` | 0.0 | 0.0 | Home position (origin) |
| `station_a` | 10.0 | 5.0 | Station A location |
| `station_b` | -5.0 | 10.0 | Station B location |

You can list, add, and delete waypoints using the TUI client commands (`wp list`, `wp add`, `wp delete`) or via the HTTP REST API.

#### Docker Deployment

Run the mock server in Docker for easy deployment:

**Using Pre-built Image (from GitHub Container Registry):**

```bash
# Pull and run the latest image
docker run -d \
  --name mocked-robot-server \
  -p 19204-19210:19204-19210 \
  -p 8080:8080 \
  ghcr.io/paval-shlyk/seersdk-rs/mocked-robot:latest

# Or with docker-compose (see docker/docker-compose.yml)
```

**Building Locally:**

```bash
# Build the Docker image
./docker/build.sh

# Run the container
./docker/run.sh

# Or use Docker Compose
docker-compose -f docker/docker-compose.yml up -d
```

The Docker image is automatically built and published to GitHub Container Registry on every push to master. Available for both `linux/amd64` and `linux/arm64` platforms.

See [docker/README.md](docker/README.md) for detailed Docker documentation and [.github/DOCKER_CI.md](.github/DOCKER_CI.md) for CI/CD information.

### TUI Client

An interactive Terminal User Interface for sending and receiving RBK messages. Uses the seersdk-rs crate to communicate with robots.

```bash
# Connect to mock server
cargo run --example tui_client -- localhost

# Or connect to a real robot
cargo run --example tui_client -- 192.168.8.114
```

Features:
- Interactive command-line interface with ratatui
- Vim-like navigation (j/k for scrolling, g/G for top/bottom)
- Scrollable message history with visual indicators
- Real-time message display
- Support for common commands:
  - `battery` / `bat` / `1` - Query battery status
  - `position` / `pos` / `2` - Query robot position
  - `info` / `3` - Query robot information
  - `nav <target>` / `4` - Navigate to target
  - `stop` / `5` - Stop navigation
  - `pause` / `6` - Pause navigation
  - `resume` / `7` - Resume navigation
  - `jack load` / `8` - Load jack
  - `jack unload` / `9` - Unload jack
  - **`wp list`** - List all waypoints
  - **`wp add <id> <x> <y>`** - Add waypoint
  - **`wp delete <id>`** - Delete waypoint
  - `help` - Show all available commands
  - `clear` - Clear message history

Controls:
- **Normal Mode** (press `Esc` to enter):
  - `i` - Enter editing mode
  - `q` - Quit application
  - `?` - Show help
  - `c` - Clear screen
  - `j`/`↓` - Scroll down
  - `k`/`↑` - Scroll up
  - `d`/`PgDn` - Page down
  - `u`/`PgUp` - Page up
  - `g`/`Home` - Go to top
  - `G`/`End` - Go to bottom
- **Editing Mode** (default):
  - Type commands and press `Enter` to send
  - `Esc` - Enter normal mode
  - `Ctrl+j`/`Ctrl+k` - Scroll while typing
  - `Ctrl+c` - Clear screen

### Test the Mock Server

A simple test client that verifies the mock server is working correctly:

```bash
# Make sure the mock server is running first
cargo run --example test_mock_server
```

### Comprehensive Demo

A visual demonstration showing various API calls in action:

```bash
# Make sure the mock server is running first
cargo run --example demo

# Or connect to a real robot
cargo run --example demo -- 192.168.8.114
```

This demo showcases:
- Robot information queries
- Battery status monitoring
- Position tracking
- Navigation commands (start, pause, resume, cancel)
- Jack operations (load, height adjustment, unload)
- Control commands (relocation, confirmation)

## Testing

The project includes both unit tests and integration tests. Integration tests **automatically start the mock server** if it's not already running!

### Quick Start

```bash
# Run all tests (auto-starts mock server if needed)
cargo test
# Or manually manage the server
# Terminal 1:
cargo run --example mock_robot_server
# Terminal 2:
cargo test
```

### Test Types

**Unit Tests:**
```bash
cargo test --lib
```

**Integration Tests:**
```bash
cargo test --test integration_tests
```

The integration tests verify:
- Protocol communication correctness
- Various API request/response types
- Navigation commands
- Control commands
- Peripheral operations
- Concurrent request handling

### Automatic Mock Server

Integration tests now include a test fixture that:
- ✅ Checks if mock server is already running
- ✅ Automatically starts server if needed
- ✅ Waits for server to be ready
- ✅ Keeps server running for all tests
- ✅ Stops server when tests complete

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

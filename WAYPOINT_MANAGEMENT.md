# Waypoint Management Feature

This document describes the waypoint management functionality added to the Mock Robot Server and TUI Client.

## Mock Robot Server

The mock robot server now includes an HTTP REST API for managing waypoints on port 8080.

### Starting the Server

```bash
cargo run --example mock_robot_server
```

The server will start with the following services:
- **RBK Protocol APIs**: Ports 19204-19208, 19210
- **HTTP REST API**: Port 8080

### HTTP Endpoints

#### 1. POST /waypoints
Add one or more waypoints.

**Request Body:**
```json
[
  {
    "id": "station_c",
    "x": 25.5,
    "y": 30.2
  },
  {
    "id": "station_d",
    "x": -10.0,
    "y": 5.5
  }
]
```

**Response:** `201 Created`

**Example:**
```bash
curl -X POST http://localhost:8080/waypoints \
  -H "Content-Type: application/json" \
  -d '[{"id":"station_c","x":25.5,"y":30.2}]'
```

#### 2. GET /waypoints
Retrieve all waypoints.

**Response:**
```json
[
  {
    "id": "home",
    "x": 0.0,
    "y": 0.0
  },
  {
    "id": "station_a",
    "x": 10.0,
    "y": 5.0
  }
]
```

**Example:**
```bash
curl http://localhost:8080/waypoints
```

#### 3. DELETE /waypoints/{id}
Delete a waypoint by ID.

**Response:** 
- `204 No Content` - Successfully deleted
- `404 Not Found` - Waypoint ID doesn't exist

**Example:**
```bash
curl -X DELETE http://localhost:8080/waypoints/station_a
```

### Default Waypoints

The server initializes with these default waypoints:
- `home`: (0.0, 0.0)
- `station_a`: (10.0, 5.0)
- `station_b`: (-5.0, 10.0)

## TUI Client

The TUI client now includes commands to manage waypoints via the HTTP API.

### Starting the Client

```bash
cargo run --example tui_client -- localhost
```

### Waypoint Commands

#### List Waypoints
```
wp list
```
or
```
wp ls
```

**Output:**
```
Waypoints:
  home - (0.00, 0.00)
  station_a - (10.00, 5.00)
  station_b - (-5.00, 10.00)
```

#### Add Waypoint
```
wp add <id> <x> <y>
```

**Example:**
```
wp add charging_station 15.5 20.3
```

**Output:**
```
✓ Waypoint 'charging_station' added at (15.50, 20.30)
```

#### Delete Waypoint
```
wp delete <id>
```
or
```
wp del <id>
```
or
```
wp rm <id>
```

**Example:**
```
wp delete charging_station
```

**Output:**
```
✓ Waypoint 'charging_station' deleted
```

### Complete Workflow Example

1. Start the mock server:
```bash
cargo run --example mock_robot_server
```

2. In another terminal, start the TUI client:
```bash
cargo run --example tui_client -- localhost
```

3. In the TUI, type commands:
```
> wp list
> wp add warehouse_1 50.0 25.0
> wp add warehouse_2 55.0 30.0
> wp list
> nav warehouse_1
> wp delete warehouse_1
> wp list
```

## Testing

A test script is provided to test the HTTP endpoints:

```bash
./test_waypoints.sh
```

This script tests all three endpoints (POST, GET, DELETE) and verifies the responses.

## Architecture

### Mock Server
- Uses `axum` web framework for HTTP endpoints
- Stores waypoints in `HashMap<String, Waypoint>` wrapped in `Arc<RwLock<>>`
- CORS enabled for cross-origin requests
- Separate from RBK protocol TCP servers

### TUI Client
- Uses `reqwest` HTTP client for API calls
- Async/await for non-blocking HTTP requests
- Integrated into existing command system
- Error handling with user-friendly messages

## Error Handling

### Server
- Returns appropriate HTTP status codes
- 201 Created for successful POST
- 200 OK for successful GET
- 204 No Content for successful DELETE
- 404 Not Found when waypoint doesn't exist

### Client
- Connection errors: "Failed to connect: ..."
- HTTP errors: "HTTP error: ..."
- Parse errors: "Failed to parse waypoints: ..."
- Not found: "Waypoint 'id' not found"

## Future Enhancements

Possible improvements:
- Waypoint update/modify endpoint (PUT /waypoints/{id})
- Waypoint validation (coordinates range, ID format)
- Persistent storage (save to file/database)
- Waypoint metadata (description, tags, creation time)
- Batch operations (delete multiple waypoints)
- Search/filter waypoints by area or name pattern

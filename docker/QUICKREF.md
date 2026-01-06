# Quick Reference: Docker Mocked RBK Robot

## Build & Run

```bash
# Quick start
./docker/build.sh && ./docker/run.sh

# Or with Docker Compose
docker-compose -f docker/docker-compose.yml up -d
```

## Default Waypoints

| ID | X | Y |
|----|---|---|
| `home` | 0.0 | 0.0 |
| `station_a` | 10.0 | 5.0 |
| `station_b` | -5.0 | 10.0 |

## Ports

- **19204-19210**: RBK Protocol (State, Control, Navigation, Config, Kernel, Peripheral)
- **8080**: HTTP REST API (Waypoint Management)

## Quick Tests

```bash
# List waypoints
curl http://localhost:8080/waypoints | jq

# Add waypoint
curl -X POST http://localhost:8080/waypoints \
  -H "Content-Type: application/json" \
  -d '[{"id":"test","x":10.0,"y":20.0}]'

# Delete waypoint
curl -X DELETE http://localhost:8080/waypoints/test

# Connect with TUI
cargo run --example tui_client -- localhost
```

## Management

```bash
# View logs
docker logs -f mocked-robot-server

# Stop container
docker stop mocked-robot-server

# Restart container
docker restart mocked-robot-server

# Remove container
docker rm -f mocked-robot-server
```

## Image Info

- **Size**: ~90MB (runtime)
- **Base**: debian:bookworm-slim
- **User**: robot (UID 1000, non-root)
- **Health Check**: Enabled (checks HTTP API)

# Docker Setup for Mocked RBK Robot

This directory contains Docker configuration files for building and running the Mocked RBK Robot Server in a containerized environment.

## Quick Start

### Option 1: Using Helper Scripts

```bash
# Build the Docker image
./docker/build.sh

# Run the container
./docker/run.sh
```

### Option 2: Using Docker Compose

```bash
# Build and start the container
docker-compose -f docker/docker-compose.yml up -d

# View logs
docker-compose -f docker/docker-compose.yml logs -f

# Stop the container
docker-compose -f docker/docker-compose.yml down
```

### Option 3: Manual Docker Commands

```bash
# Build the image
docker build -f docker/mocked-robot.Dockerfile -t mocked-rbk-robot:latest .

# Run the container
docker run -d \
  --name mocked-robot-server \
  -p 19204-19210:19204-19210 \
  -p 8080:8080 \
  mocked-rbk-robot:latest

# View logs
docker logs -f mocked-robot-server

# Stop the container
docker stop mocked-robot-server
```

## Dockerfile Details

The `mocked-robot.Dockerfile` uses a multi-stage build process:

### Build Stage
- Base image: `rust:1.83-slim-bookworm`
- Installs build dependencies (pkg-config, libssl-dev)
- Compiles the `mock_robot_server` example in release mode
- Strips the binary to minimize size

### Runtime Stage
- Base image: `debian:bookworm-slim` (minimal Debian)
- Installs only runtime dependencies (ca-certificates, libssl3)
- Creates non-root user `robot` (UID 1000) for security
- Copies only the compiled binary
- Exposes necessary ports
- Includes health check

### Image Size Optimization
The multi-stage build significantly reduces the final image size:
- Build stage: ~1.5GB (includes Rust toolchain and build artifacts)
- Runtime stage: ~100-150MB (only binary and runtime dependencies)

## Exposed Ports

| Port | Protocol | Description |
|------|----------|-------------|
| 19204 | RBK | State APIs (battery, position, etc.) |
| 19205 | RBK | Control APIs (stop, relocate, etc.) |
| 19206 | RBK | Navigation APIs (move, pause, resume) |
| 19207 | RBK | Config APIs (parameters, maps) |
| 19208 | RBK | Kernel APIs (shutdown, reboot) |
| 19210 | RBK | Peripheral APIs (jack, audio, I/O) |
| 8080 | HTTP | REST API for waypoint management |

## Default Waypoints

The mocked robot server initializes with three default waypoints:

| ID | X | Y | Description |
|----|---|---|-------------|
| `home` | 0.0 | 0.0 | Home position (origin) |
| `station_a` | 10.0 | 5.0 | Station A location |
| `station_b` | -5.0 | 10.0 | Station B location |

## Usage Examples

### Connect with TUI Client

```bash
# Install TUI client (from host machine)
cargo run --example tui_client -- localhost
```

In the TUI:
```
> wp list
> wp add warehouse_1 50.0 25.0
> nav warehouse_1
> battery
> position
```

### Test HTTP API

```bash
# List waypoints
curl http://localhost:8080/waypoints | jq

# Add a waypoint
curl -X POST http://localhost:8080/waypoints \
  -H "Content-Type: application/json" \
  -d '[{"id":"test_point","x":15.5,"y":20.3}]'

# Delete a waypoint
curl -X DELETE http://localhost:8080/waypoints/test_point
```

### Test RBK Protocol

Use the TUI client or any RBK-compatible client to connect to `localhost` on the appropriate port.

## Health Check

The container includes a health check that verifies the HTTP API is responding:

```bash
# Check container health status
docker ps --filter "name=mocked-robot-server"

# The HEALTH status will show:
# - starting: Container is initializing
# - healthy: Server is running and responding
# - unhealthy: Server is not responding
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Logging level (error, warn, info, debug, trace) |

Example with custom log level:
```bash
docker run -e RUST_LOG=debug -p 19204-19210:19204-19210 -p 8080:8080 mocked-rbk-robot:latest
```

## Troubleshooting

### Container won't start

Check logs:
```bash
docker logs mocked-robot-server
```

### Ports already in use

Stop conflicting services or change port mappings:
```bash
docker run -p 20204-20210:19204-19210 -p 9080:8080 mocked-rbk-robot:latest
```

### Connection refused

Ensure the container is running and healthy:
```bash
docker ps
docker logs mocked-robot-server
curl http://localhost:8080/waypoints
```

### Rebuild after code changes

```bash
docker-compose -f docker/docker-compose.yml down
docker-compose -f docker/docker-compose.yml build --no-cache
docker-compose -f docker/docker-compose.yml up -d
```

## Security Notes

- Container runs as non-root user (`robot`, UID 1000)
- Only necessary runtime dependencies are included
- No build tools or source code in final image
- Health checks ensure service availability

## Development

To modify the mock server behavior:

1. Update `examples/mock_robot_server.rs`
2. Rebuild the Docker image:
   ```bash
   ./docker/build.sh
   ```
3. Restart the container:
   ```bash
   ./docker/run.sh
   ```

## Production Deployment

For production use, consider:

- Using specific version tags instead of `latest`
- Implementing persistent storage for waypoints
- Adding authentication/authorization
- Using a reverse proxy (nginx, traefik)
- Setting up monitoring and alerting
- Using orchestration (Kubernetes, Docker Swarm)

Example with version tag:
```bash
docker build -f docker/mocked-robot.Dockerfile -t mocked-rbk-robot:v1.0.0 .
docker tag mocked-rbk-robot:v1.0.0 your-registry.com/mocked-rbk-robot:v1.0.0
docker push your-registry.com/mocked-rbk-robot:v1.0.0
```

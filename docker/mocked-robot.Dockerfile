# Multi-stage Dockerfile for Mocked RBK Robot Server
# This creates a minimal Docker image for running the mock robot server

# ============================================================================
# Build Stage: Compile the Rust application
# ============================================================================
FROM rust:1.83-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty project
WORKDIR /build

# Copy manifests
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./

# Copy source code
COPY src ./src
COPY examples ./examples

# Build the mock_robot_server example in release mode
# This will create an optimized binary with minimal size
RUN cargo build --release --example mock_robot_server

# Strip the binary to reduce size further
RUN strip /build/target/release/examples/mock_robot_server

# ============================================================================
# Runtime Stage: Minimal image with only runtime dependencies
# ============================================================================
FROM debian:bookworm-slim

# Install only runtime dependencies (OpenSSL for HTTPS support)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user to run the application
RUN useradd -m -u 1000 robot && \
    mkdir -p /app && \
    chown -R robot:robot /app

# Set working directory
WORKDIR /app

# Copy the compiled binary from builder stage
COPY --from=builder /build/target/release/examples/mock_robot_server /app/mock_robot_server

# Change ownership
RUN chown robot:robot /app/mock_robot_server

# Switch to non-root user
USER robot

# Expose ports
# RBK Protocol ports
EXPOSE 19204 19205 19206 19207 19208 19210
# HTTP REST API port
EXPOSE 8080

# Health check to ensure the server is running
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/waypoints || exit 1

# Set environment variables
ENV RUST_LOG=info

# Run the mock robot server
CMD ["/app/mock_robot_server"]


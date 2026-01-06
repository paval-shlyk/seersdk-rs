#!/bin/bash
# Run script for Mocked RBK Robot Docker container

set -e

IMAGE_NAME="mocked-rbk-robot"
IMAGE_TAG="${1:-latest}"
CONTAINER_NAME="mocked-robot-server"

echo "=== Running Mocked RBK Robot Docker Container ==="
echo "Image: ${IMAGE_NAME}:${IMAGE_TAG}"
echo "Container: ${CONTAINER_NAME}"
echo ""

# Stop and remove existing container if it exists
if docker ps -a --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
    echo "Stopping and removing existing container..."
    docker stop "${CONTAINER_NAME}" 2>/dev/null || true
    docker rm "${CONTAINER_NAME}" 2>/dev/null || true
fi

# Run the container
docker run -d \
    --name "${CONTAINER_NAME}" \
    -p 19204:19204 \
    -p 19205:19205 \
    -p 19206:19206 \
    -p 19207:19207 \
    -p 19208:19208 \
    -p 19210:19210 \
    -p 8080:8080 \
    "${IMAGE_NAME}:${IMAGE_TAG}"

echo ""
echo "✓ Container started successfully!"
echo ""
echo "Container status:"
docker ps --filter "name=${CONTAINER_NAME}"
echo ""
echo "Services available at:"
echo "  - RBK Protocol: localhost:19204-19210"
echo "  - HTTP REST API: http://localhost:8080"
echo ""
echo "View logs:"
echo "  docker logs -f ${CONTAINER_NAME}"
echo ""
echo "Test the server:"
echo "  curl http://localhost:8080/waypoints"
echo ""
echo "Stop the container:"
echo "  docker stop ${CONTAINER_NAME}"

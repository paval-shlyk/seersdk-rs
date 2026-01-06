#!/bin/bash
# Build script for Mocked RBK Robot Docker image

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

IMAGE_NAME="mocked-rbk-robot"
IMAGE_TAG="${1:-latest}"

echo "=== Building Mocked RBK Robot Docker Image ==="
echo "Image: ${IMAGE_NAME}:${IMAGE_TAG}"
echo ""

cd "$PROJECT_ROOT"

# Build the Docker image
docker build \
    -f docker/mocked-robot.Dockerfile \
    -t "${IMAGE_NAME}:${IMAGE_TAG}" \
    .

echo ""
echo "✓ Docker image built successfully!"
echo ""
echo "Image details:"
docker images "${IMAGE_NAME}:${IMAGE_TAG}"
echo ""
echo "To run the container:"
echo "  docker run -p 19204-19210:19204-19210 -p 8080:8080 ${IMAGE_NAME}:${IMAGE_TAG}"
echo ""
echo "Or use the run script:"
echo "  ./docker/run.sh"

#!/bin/bash
# Test script for waypoint HTTP endpoints

BASE_URL="http://localhost:8080"

echo "=== Testing Waypoint HTTP Endpoints ==="
echo ""

echo "1. GET /waypoints - List all waypoints"
curl -s -X GET "$BASE_URL/waypoints" | jq '.'
echo ""
echo ""

echo "2. POST /waypoints - Add new waypoints"
curl -s -X POST "$BASE_URL/waypoints" \
  -H "Content-Type: application/json" \
  -d '[{"id":"test_point","x":15.5,"y":20.3}]'
echo ""
echo "Status code: $?"
echo ""

echo "3. GET /waypoints - List all waypoints (should include test_point)"
curl -s -X GET "$BASE_URL/waypoints" | jq '.'
echo ""
echo ""

echo "4. DELETE /waypoints/test_point - Delete test_point"
curl -s -X DELETE "$BASE_URL/waypoints/test_point" -w "HTTP Status: %{http_code}\n"
echo ""

echo "5. DELETE /waypoints/nonexistent - Try to delete nonexistent waypoint (should return 404)"
curl -s -X DELETE "$BASE_URL/waypoints/nonexistent" -w "HTTP Status: %{http_code}\n"
echo ""

echo "6. GET /waypoints - List all waypoints (test_point should be gone)"
curl -s -X GET "$BASE_URL/waypoints" | jq '.'
echo ""

echo "=== Test Complete ==="

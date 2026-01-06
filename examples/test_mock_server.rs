use seersdk_rs::{
    BatteryStatusRequest, CommonInfoRequest, MoveToTarget, MoveToTargetRequest,
    RbkClient, RobotPoseRequest,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Mock Robot Server ===\n");

    let client = RbkClient::new("localhost");

    // Test 1: Query robot info
    println!("Test 1: Query robot info");
    let request = CommonInfoRequest::new();
    let response = client.request(request, Duration::from_secs(5)).await?;
    println!("  ID: {}", response.id);
    println!("  Model: {}", response.model);
    println!("  Version: {}\n", response.version);

    // Test 2: Query battery
    println!("Test 2: Query battery status");
    let request = BatteryStatusRequest::new();
    let response = client.request(request, Duration::from_secs(5)).await?;
    println!("  Battery Level: {:.1}%", response.battery_level * 100.0);
    println!("  Voltage: {:.2}V", response.voltage);
    println!("  Charging: {}\n", response.charging);

    // Test 3: Query position
    println!("Test 3: Query robot position");
    let request = RobotPoseRequest::new();
    let response = client.request(request, Duration::from_secs(5)).await?;
    println!("  X: {:.3}m", response.x);
    println!("  Y: {:.3}m", response.y);
    println!("  Angle: {:.3}rad\n", response.angle);

    // Test 4: Navigate to target
    println!("Test 4: Navigate to target");
    let move_cmd = MoveToTarget::new("station_1");
    let request = MoveToTargetRequest::new(move_cmd);
    let response = client.request(request, Duration::from_secs(5)).await?;
    println!("  Response: {}\n", response.message);

    println!("✓ All tests passed!");

    Ok(())
}

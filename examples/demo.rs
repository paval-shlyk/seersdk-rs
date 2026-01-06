//! Demonstration script showing various API calls
//!
//! This script demonstrates the capabilities of the seersdk-rs crate
//! by making various API calls to a robot (mock or real).

use seersdk_rs::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let robot_ip = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "localhost".to_string());

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║       RBK Robot SDK Demonstration                         ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("Connecting to robot at: {}\n", robot_ip);

    let client = RbkClient::new(&robot_ip);

    // Section 1: Robot Information
    println!("┌─ Robot Information ──────────────────────────────────────┐");
    match client
        .request(CommonInfoRequest::new(), Duration::from_secs(5))
        .await
    {
        Ok(info) => {
            println!("│ ✓ Robot ID:      {:<38} │", info.id);
            println!("│ ✓ Model:         {:<38} │", info.model);
            println!("│ ✓ Version:       {:<38} │", info.version);
        }
        Err(e) => println!("│ ✗ Failed: {:<43} │", e),
    }
    println!("└──────────────────────────────────────────────────────────┘\n");

    // Section 2: Battery Status
    println!("┌─ Battery Status ─────────────────────────────────────────┐");
    match client
        .request(BatteryStatusRequest::new(), Duration::from_secs(5))
        .await
    {
        Ok(battery) => {
            println!(
                "│ ✓ Level:         {:<38.1}% │",
                battery.battery_level * 100.0
            );
            println!("│ ✓ Voltage:       {:<38.2}V │", battery.voltage);
            println!("│ ✓ Current:       {:<38.2}A │", battery.current);
            println!("│ ✓ Temperature:   {:<38.1}°C│", battery.battery_temp);
            println!("│ ✓ Charging:      {:<38} │", battery.charging);
        }
        Err(e) => println!("│ ✗ Failed: {:<43} │", e),
    }
    println!("└──────────────────────────────────────────────────────────┘\n");

    // Section 3: Robot Position
    println!("┌─ Robot Position ─────────────────────────────────────────┐");
    match client
        .request(RobotPoseRequest::new(), Duration::from_secs(5))
        .await
    {
        Ok(pose) => {
            println!("│ ✓ X:             {:<38.3}m │", pose.x);
            println!("│ ✓ Y:             {:<38.3}m │", pose.y);
            println!("│ ✓ Angle:         {:<38.3}rad│", pose.angle);
            println!("│ ✓ Angle:         {:<38.1}° │", pose.angle.to_degrees());
            println!("│ ✓ Confidence:    {:<38.1}% │", pose.confidence * 100.0);
        }
        Err(e) => println!("│ ✗ Failed: {:<43} │", e),
    }
    println!("└──────────────────────────────────────────────────────────┘\n");

    // Section 4: Operating Information
    println!("┌─ Operating Information ──────────────────────────────────┐");
    match client
        .request(OperationInfoRequest::new(), Duration::from_secs(5))
        .await
    {
        Ok(info) => {
            println!("│ ✓ Mileage:       {:<38.2}m │", info.mileage);
            println!(
                "│ ✓ Total Time:    {:<38.1}s │",
                info.total_time_ms / 1000.0
            );
            println!("│ ✓ Controller:    {:<38.1}°C│", info.controller_temp);
        }
        Err(e) => println!("│ ✗ Failed: {:<43} │", e),
    }
    println!("└──────────────────────────────────────────────────────────┘\n");

    // Section 5: Block Status
    println!("┌─ Obstacle Status ────────────────────────────────────────┐");
    match client
        .request(BlockStatusRequest::new(), Duration::from_secs(5))
        .await
    {
        Ok(status) => {
            println!("│ ✓ Blocked:       {:<38} │", status.is_blocked);
            if let Some(reason) = status.reason {
                println!("│ ✓ Reason:        {:<38} │", reason);
            }
        }
        Err(e) => println!("│ ✗ Failed: {:<43} │", e),
    }
    println!("└──────────────────────────────────────────────────────────┘\n");

    // Section 6: Navigation Commands
    println!("┌─ Navigation Test ────────────────────────────────────────┐");

    // Start navigation
    let move_cmd = MoveToTarget::new("demo_target");
    match client
        .request(MoveToTargetRequest::new(move_cmd), Duration::from_secs(5))
        .await
    {
        Ok(_) => {
            println!("│ ✓ Navigation started to target: demo_target          │")
        }
        Err(e) => println!("│ ✗ Failed to start: {:<32} │", e),
    }

    tokio::time::sleep(Duration::from_millis(500)).await;

    // Query navigation status
    match client
        .request(
            NavStatusRequest::new(GetNavStatus::new()),
            Duration::from_secs(5),
        )
        .await
    {
        Ok(nav) => {
            println!(
                "│ ✓ Nav Status:    {:<38} │",
                format!("{:?}", nav.status)
            );
            println!("│ ✓ Nav Type:      {:<38} │", format!("{:?}", nav.ty));
            println!("│ ✓ Target:        {:<38} │", nav.target_id);
        }
        Err(e) => println!("│ ✗ Failed: {:<43} │", e),
    }

    tokio::time::sleep(Duration::from_millis(500)).await;

    // Pause navigation
    match client
        .request(PauseTaskRequest::new(), Duration::from_secs(5))
        .await
    {
        Ok(_) => {
            println!("│ ✓ Navigation paused                                  │")
        }
        Err(e) => println!("│ ✗ Failed to pause: {:<31} │", e),
    }

    tokio::time::sleep(Duration::from_millis(500)).await;

    // Resume navigation
    match client
        .request(ResumeTaskRequest::new(), Duration::from_secs(5))
        .await
    {
        Ok(_) => {
            println!("│ ✓ Navigation resumed                                 │")
        }
        Err(e) => println!("│ ✗ Failed to resume: {:<30} │", e),
    }

    tokio::time::sleep(Duration::from_millis(500)).await;

    // Stop navigation
    match client
        .request(CancelTaskRequest::new(), Duration::from_secs(5))
        .await
    {
        Ok(_) => {
            println!("│ ✓ Navigation canceled                                │")
        }
        Err(e) => println!("│ ✗ Failed to cancel: {:<30} │", e),
    }

    println!("└──────────────────────────────────────────────────────────┘\n");

    // Section 7: Jack Operations
    println!("┌─ Jack Operations ────────────────────────────────────────┐");

    match client
        .request(LoadJackRequest::new(), Duration::from_secs(5))
        .await
    {
        Ok(_) => {
            println!("│ ✓ Jack load initiated                                │")
        }
        Err(e) => println!("│ ✗ Failed to load: {:<32} │", e),
    }

    tokio::time::sleep(Duration::from_millis(500)).await;

    let height_cmd = SetJackHeight::new(0.3);
    match client
        .request(
            SetJackHeightRequest::new(height_cmd),
            Duration::from_secs(5),
        )
        .await
    {
        Ok(_) => {
            println!("│ ✓ Jack height set to 0.3m                            │")
        }
        Err(e) => println!("│ ✗ Failed to set height: {:<25} │", e),
    }

    tokio::time::sleep(Duration::from_millis(500)).await;

    match client
        .request(UnloadJackRequest::new(), Duration::from_secs(5))
        .await
    {
        Ok(_) => {
            println!("│ ✓ Jack unload initiated                              │")
        }
        Err(e) => println!("│ ✗ Failed to unload: {:<30} │", e),
    }

    println!("└──────────────────────────────────────────────────────────┘\n");

    // Section 8: Control Commands
    println!("┌─ Control Commands ───────────────────────────────────────┐");

    match client
        .request(RelocateRequest::new(), Duration::from_secs(5))
        .await
    {
        Ok(_) => {
            println!("│ ✓ Relocation initiated                               │")
        }
        Err(e) => println!("│ ✗ Failed: {:<43} │", e),
    }

    tokio::time::sleep(Duration::from_millis(500)).await;

    match client
        .request(ConfirmLocationRequest::new(), Duration::from_secs(5))
        .await
    {
        Ok(_) => {
            println!("│ ✓ Location confirmed                                 │")
        }
        Err(e) => println!("│ ✗ Failed: {:<43} │", e),
    }

    println!("└──────────────────────────────────────────────────────────┘\n");

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║       ✓ All demonstration tests completed                 ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    Ok(())
}

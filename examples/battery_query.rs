use seersdk_rs::{BatteryStatusRequest, RbkClient};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Create a client connection to the robot
    let rbk_client = RbkClient::new("192.168.8.114");

    // Step 2: Send a request to query the robot's battery level
    // According to RBK protocol, API 1007 is for querying battery
    let request = BatteryStatusRequest::new();
    let response = rbk_client.request(request, Duration::from_secs(10)).await?;

    // The response is now automatically deserialized to StatusMessage
    if let Some(code) = response.code
        && code == seersdk_rs::StatusCode::Success
    {
        // Robot returned success
        println!("Battery status query succeeded!");
        println!("Response: {:?}", response);
    } else {
        // Robot returned failure
        println!("Robot error: {}", response.message);
    }

    // Note: RbkClient now implements Drop for automatic cleanup
    // The connection will be cleaned up when the client goes out of scope

    Ok(())
}

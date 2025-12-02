use seersdk_rs::{RbkClient, RbkResultKind};
use serde_json::Value;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Create a client connection to the robot
    let rbk_client = RbkClient::new("192.168.8.114");

    // Step 2: Send a request to query the robot's battery level
    // According to RBK protocol, API 1007 is for querying battery with parameter {"simple":true}
    let req_str = r#"{"simple": true}"#; // Request parameter as JSON string
    let result = rbk_client
        .request(1007, req_str, Duration::from_secs(10))
        .await?;

    if result.kind == RbkResultKind::Ok {
        // SDK request to robot succeeded
        let res_str = &result.res_str; // Get response from robot
        let res_json: Value = serde_json::from_str(res_str)?;

        if res_json["ret_code"].as_i64() == Some(0) {
            // Robot returned success
            let battery_level =
                res_json["battery_level"].as_f64().unwrap_or(0.0);
            println!("Battery level: {:.2}%", battery_level);
        } else {
            // Robot returned failure
            let robot_err_msg =
                res_json["err_msg"].as_str().unwrap_or("Unknown error");
            println!("Robot error: {}", robot_err_msg);
        }
    } else {
        // SDK request to robot failed
        println!("SDK error: {}", result.err_msg);
    }

    // Note: RbkClient now implements Drop for automatic cleanup
    // The connection will be cleaned up when the client goes out of scope

    Ok(())
}

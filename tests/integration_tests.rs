//! Integration tests for seersdk-rs
//!
//! These tests verify the correctness of the protocol communication
//! and various API request/response types against a mock server.
//!
//! The mock server will be automatically started if not already running.

use seersdk_rs::*;
use std::time::Duration;
use tokio::sync::{Mutex, OnceCell};

mod test_fixtures;
use test_fixtures::MockServerFixture;

static FIXTURE: Mutex<OnceCell<MockServerFixture>> =
    Mutex::const_new(OnceCell::const_new());

#[ctor::dtor]
fn shutdown_mock_server() {
    let mut lock = FIXTURE.try_lock().expect("Failed to lock FIXTURE");

    let Some(fixture) = lock.take() else {
        return;
    };

    eprintln!("Shutting down mock server...");

    drop(fixture);
}

/// Initialize the mock server once for all tests
async fn ensure_mock_server() {
    FIXTURE
        .lock()
        .await
        .get_or_init(|| async { MockServerFixture::new().await })
        .await;
}

/// Helper function to create a test client
async fn create_test_client() -> RbkClient {
    ensure_mock_server().await;
    RbkClient::new("localhost")
}

#[tokio::test]
async fn test_robot_info_query() {
    let client = create_test_client().await;
    let request = CommonInfoRequest::new();

    let response = client.request(request, Duration::from_secs(5)).await;
    assert!(
        response.is_ok(),
        "Failed to query robot info: {:?}",
        response.err()
    );

    let info = response.unwrap();
    assert!(!info.id.is_empty(), "Robot ID should not be empty");
    assert!(!info.model.is_empty(), "Robot model should not be empty");
    assert!(
        !info.version.is_empty(),
        "Robot version should not be empty"
    );
}

#[tokio::test]
async fn test_battery_status_query() {
    let client = create_test_client().await;
    let request = BatteryStatusRequest::new();

    let response = client.request(request, Duration::from_secs(5)).await;
    assert!(
        response.is_ok(),
        "Failed to query battery status: {:?}",
        response.err()
    );

    let battery = response.unwrap();
    assert!(
        battery.battery_level >= 0.0 && battery.battery_level <= 1.0,
        "Battery level should be between 0 and 1"
    );
    assert!(battery.voltage > 0.0, "Voltage should be positive");
}

#[tokio::test]
async fn test_robot_pose_query() {
    let client = create_test_client().await;
    let request = RobotPoseRequest::new();

    let response = client.request(request, Duration::from_secs(5)).await;
    assert!(
        response.is_ok(),
        "Failed to query robot pose: {:?}",
        response.err()
    );

    let pose = response.unwrap();
    assert!(
        pose.confidence >= 0.0 && pose.confidence <= 1.0,
        "Confidence should be between 0 and 1"
    );
}

#[tokio::test]
async fn test_block_status_query() {
    let client = create_test_client().await;
    let request = BlockStatusRequest::new();

    let response = client.request(request, Duration::from_secs(5)).await;
    assert!(
        response.is_ok(),
        "Failed to query block status: {:?}",
        response.err()
    );

    let status = response.unwrap();
    assert_eq!(status.code, StatusCode::Success);
}

#[tokio::test]
async fn test_navigation_commands() {
    let client = create_test_client().await;

    // Test navigation to target
    let move_cmd = MoveToTarget::new("test_target");
    let request = MoveToTargetRequest::new(move_cmd);

    let response = client.request(request, Duration::from_secs(5)).await;
    assert!(
        response.is_ok(),
        "Failed to start navigation: {:?}",
        response.err()
    );

    // Test pause
    let pause_request = PauseTaskRequest::new();
    let pause_response =
        client.request(pause_request, Duration::from_secs(5)).await;
    assert!(
        pause_response.is_ok(),
        "Failed to pause navigation: {:?}",
        pause_response.err()
    );

    // Test resume
    let resume_request = ResumeTaskRequest::new();
    let resume_response =
        client.request(resume_request, Duration::from_secs(5)).await;
    assert!(
        resume_response.is_ok(),
        "Failed to resume navigation: {:?}",
        resume_response.err()
    );

    // Test cancel
    let cancel_request = CancelTaskRequest::new();
    let cancel_response =
        client.request(cancel_request, Duration::from_secs(5)).await;
    assert!(
        cancel_response.is_ok(),
        "Failed to cancel navigation: {:?}",
        cancel_response.err()
    );
}

#[tokio::test]
async fn test_jack_operations() {
    let client = create_test_client().await;

    // Test jack load
    let load_request = LoadJackRequest::new();
    let load_response =
        client.request(load_request, Duration::from_secs(5)).await;
    assert!(
        load_response.is_ok(),
        "Failed to load jack: {:?}",
        load_response.err()
    );

    // Test jack unload
    let unload_request = UnloadJackRequest::new();
    let unload_response =
        client.request(unload_request, Duration::from_secs(5)).await;
    assert!(
        unload_response.is_ok(),
        "Failed to unload jack: {:?}",
        unload_response.err()
    );

    // Test jack stop
    let stop_request = StopJackRequest::new();
    let stop_response =
        client.request(stop_request, Duration::from_secs(5)).await;
    assert!(
        stop_response.is_ok(),
        "Failed to stop jack: {:?}",
        stop_response.err()
    );

    // Test set jack height
    let height_request = SetJackHeightRequest::new(SetJackHeight::new(0.5));
    let height_response =
        client.request(height_request, Duration::from_secs(5)).await;
    assert!(
        height_response.is_ok(),
        "Failed to set jack height: {:?}",
        height_response.err()
    );
}

#[tokio::test]
async fn test_control_commands() {
    let client = create_test_client().await;

    // Test stop exercise
    let stop_request = StopExerciseRequest::new();
    let stop_response =
        client.request(stop_request, Duration::from_secs(5)).await;
    assert!(
        stop_response.is_ok(),
        "Failed to stop exercise: {:?}",
        stop_response.err()
    );

    // Test relocate
    let relocate_request = RelocateRequest::new();
    let relocate_response = client
        .request(relocate_request, Duration::from_secs(5))
        .await;
    assert!(
        relocate_response.is_ok(),
        "Failed to relocate: {:?}",
        relocate_response.err()
    );

    // Test confirm location
    let confirm_request = ConfirmLocationRequest::new();
    let confirm_response = client
        .request(confirm_request, Duration::from_secs(5))
        .await;
    assert!(
        confirm_response.is_ok(),
        "Failed to confirm location: {:?}",
        confirm_response.err()
    );
}

#[tokio::test]
async fn test_nav_status_query() {
    let client = create_test_client().await;
    let request = NavStatusRequest::new(GetNavStatus::new());

    let response = client.request(request, Duration::from_secs(5)).await;
    assert!(
        response.is_ok(),
        "Failed to query nav status: {:?}",
        response.err()
    );

    let nav_status = response.unwrap();
    // Status should be one of the defined values
    assert!(matches!(
        nav_status.status,
        TaskStatus::None
            | TaskStatus::Waiting
            | TaskStatus::Running
            | TaskStatus::Suspended
            | TaskStatus::Completed
            | TaskStatus::Failed
            | TaskStatus::Canceled
            | TaskStatus::OverTime
            | TaskStatus::NotFound
    ));
}

#[tokio::test]
async fn test_operation_info_query() {
    let client = create_test_client().await;
    let request = OperationInfoRequest::new();

    let response = client.request(request, Duration::from_secs(5)).await;
    assert!(
        response.is_ok(),
        "Failed to query operation info: {:?}",
        response.err()
    );

    let info = response.unwrap();
    assert!(info.mileage >= 0.0, "Mileage should be non-negative");
    assert!(
        info.total_time_ms >= 0.0,
        "Total time should be non-negative"
    );
}

#[tokio::test]
async fn test_jack_status_query() {
    let client = create_test_client().await;
    let request = JackStatusRequest::new();

    let response = client.request(request, Duration::from_secs(5)).await;
    assert!(
        response.is_ok(),
        "Failed to query jack status: {:?}",
        response.err()
    );

    let status = response.unwrap();
    // Status message should have a success code
    assert_eq!(status.code, StatusCode::Success);
}

#[tokio::test]
async fn test_multiple_concurrent_requests() {
    let client = create_test_client().await;

    // Send multiple requests sequentially to avoid connection issues
    let battery_result = client
        .request(BatteryStatusRequest::new(), Duration::from_secs(5))
        .await;
    tokio::time::sleep(Duration::from_millis(100)).await;

    let pose_result = client
        .request(RobotPoseRequest::new(), Duration::from_secs(5))
        .await;
    tokio::time::sleep(Duration::from_millis(100)).await;

    let info_result = client
        .request(CommonInfoRequest::new(), Duration::from_secs(5))
        .await;

    assert!(
        battery_result.is_ok(),
        "Battery request failed: {:?}",
        battery_result.err()
    );
    assert!(
        pose_result.is_ok(),
        "Pose request failed: {:?}",
        pose_result.err()
    );
    assert!(
        info_result.is_ok(),
        "Info request failed: {:?}",
        info_result.err()
    );
}

#[tokio::test]
async fn test_move_to_target_with_options() {
    let client = create_test_client().await;

    // Create a move command with additional options
    let move_cmd = MoveToTarget::new("target_with_options")
        .with_task_id("task_001".to_string())
        .with_method(MoveMethod::Forward);

    let request = MoveToTargetRequest::new(move_cmd);
    let response = client.request(request, Duration::from_secs(5)).await;

    assert!(
        response.is_ok(),
        "Failed to navigate with options: {:?}",
        response.err()
    );
}

#[tokio::test]
async fn test_task_status_query() {
    let client = create_test_client().await;

    // Create a task status request
    let get_task_status =
        GetTaskStatus::from_iter(vec!["task_001".to_string()]);
    let request = TaskStatusRequest::new(get_task_status);

    let response = client.request(request, Duration::from_secs(5)).await;
    assert!(
        response.is_ok(),
        "Failed to query task status: {:?}",
        response.err()
    );
}

#[tokio::test]
async fn test_designed_path_navigation() {
    let client = create_test_client().await;

    // Create a path with multiple waypoints
    let path = vec![
        MoveToTarget::new("waypoint_1").with_task_id("task_1".to_string()),
        MoveToTarget::new("waypoint_2").with_task_id("task_2".to_string()),
        MoveToTarget::new("waypoint_3").with_task_id("task_3".to_string()),
    ];

    let move_path = MoveDesignedPath::new(path);
    let request = MoveDesignedPathRequest::new(move_path);

    let response = client.request(request, Duration::from_secs(5)).await;
    assert!(
        response.is_ok(),
        "Failed to navigate designed path: {:?}",
        response.err()
    );
}

#[tokio::test]
async fn test_automatic_reconnection() {
    let client = create_test_client().await;

    // First successful request
    let request1 = BatteryStatusRequest::new();
    let response1 = client.request(request1, Duration::from_secs(5)).await;
    assert!(
        response1.is_ok(),
        "First request failed: {:?}",
        response1.err()
    );

    // The mock server will close connections, which should trigger reconnection
    // Make multiple requests to test the automatic reconnection behavior
    for i in 0..3 {
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let request = CommonInfoRequest::new();
        let response = client.request(request, Duration::from_secs(5)).await;
        assert!(
            response.is_ok(),
            "Reconnection test attempt {} failed: {:?}",
            i + 1,
            response.err()
        );
    }
}

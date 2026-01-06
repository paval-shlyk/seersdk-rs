//! Test fixtures for integration tests
//!
//! This module provides utilities for running integration tests,
//! including automatic mock server management.

use std::time::Duration;

use tokio::process::{Child, Command};

/// Mock server fixture that automatically starts and stops the server
pub struct MockServerFixture {
    process: Option<Child>,
    auto_started: bool,
}

impl MockServerFixture {
    /// Create a new mock server fixture
    ///
    /// This will check if a server is already running on localhost:8080.
    /// If not, it will start one automatically.
    pub async fn new() -> Self {
        let server_running = Self::check_server_running().await;

        if server_running {
            println!("✓ Using existing mock server");
            MockServerFixture {
                process: None,
                auto_started: false,
            }
        } else {
            println!("Starting mock server for tests...");

            let process = Command::new("cargo")
                .args(&["run", "--example", "mock_robot_server"])
                .spawn()
                .expect("Failed to start mock server");

            // Wait for server to be ready
            let mut attempts = 0;
            while !Self::check_server_running().await {
                tokio::time::sleep(Duration::from_millis(500)).await;
                attempts += 1;

                if attempts > 20 {
                    panic!("Mock server failed to start after 10 seconds");
                }
            }

            println!("✓ Mock server started successfully");

            MockServerFixture {
                process: Some(process),
                auto_started: true,
            }
        }
    }

    /// Check if the mock server is running
    async fn check_server_running() -> bool {
        reqwest::Client::new()
            .get("http://localhost:8080/waypoints")
            .timeout(Duration::from_secs(1))
            .send()
            .await
            .is_ok()
    }
}

impl Drop for MockServerFixture {
    fn drop(&mut self) {
        if self.auto_started {
            if let Some(mut process) = self.process.take() {
                println!("Stopping mock server...");
                let _ = process.kill();
                let _ = process.wait();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fixture_creation() {
        let _fixture = MockServerFixture::new().await;
        // Server should be running
        assert!(MockServerFixture::check_server_running().await);
    }
}

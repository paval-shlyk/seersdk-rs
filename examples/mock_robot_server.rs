//! Mock RBK Robot Server
//!
//! This is a standalone binary that emulates a complete RBK robot server
//! with mock navigation logic. It implements all API endpoints across
//! multiple ports as per the RBK protocol specification.
//!
//! ## Navigation Task Emulation
//!
//! The server now fully supports the MoveToTargetList API (3066) with realistic task simulation:
//! - Parses navigation task lists and creates a task queue
//! - Simulates robot movement from start to target for each task
//! - Updates position smoothly over time (0.1 units per 500ms)
//! - Tracks individual task status (waiting, running, completed, failed)
//! - Reports progress via NavStatus (API 1020) and TaskPackage (API 1110)
//! - Supports pause (3001), resume (3002), and cancel (3003) operations
//!
//! ## HTTP REST API
//!
//! Additionally, it provides HTTP REST API for waypoint management:
//! - POST /waypoints: Add waypoints (JSON array with id, x, y)
//! - GET /waypoints: Retrieve all waypoints
//! - DELETE /waypoints/{ID}: Delete waypoint by ID
//!
//! # Usage
//!
//! ```bash
//! cargo run --example mock_robot_server
//! ```
//!
//! The server will listen on:
//! - Port 19204: State APIs (including NavStatus 1020, TaskPackage 1110)
//! - Port 19205: Control APIs (Stop 2000)
//! - Port 19206: Navigation APIs (MoveToTargetList 3066, Pause 3001, Resume 3002, Cancel 3003)
//! - Port 19207: Config APIs
//! - Port 19208: Kernel APIs
//! - Port 19210: Peripheral APIs
//! - Port 8080: HTTP REST API for waypoints
//!
//! # Testing
//!
//! Run the test scripts to verify navigation functionality:
//! ```bash
//! ./test_tasks.sh
//! ./test_navigation_states.sh
//! ```
//!
//! See TASK_NAVIGATION.md for detailed documentation.

use axum::{
    Json, Router,
    extract::{Path, State as AxumState},
    http::StatusCode,
    routing::{delete, get, post},
};
use bytes::{Buf, BufMut, BytesMut};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

// Protocol constants
const START_MARK: u8 = 0x5A;
const PROTO_VERSION: u8 = 0x01;

/// Waypoint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Waypoint {
    id: String,
    x: f64,
    y: f64,
}

/// Shared application state
struct AppState {
    robot: Arc<RwLock<RobotState>>,
    waypoints: Arc<RwLock<HashMap<String, Waypoint>>>,
}

/// Navigation task item
#[derive(Debug, Clone)]
struct NavTask {
    task_id: String,
    start: String,
    target: String,
    start_pos: [f64; 3],
    target_pos: [f64; 3],
    status: u32, // 1=waiting, 2=running, 4=completed, 5=failed
}

/// Shared robot state
#[derive(Clone)]
struct RobotState {
    // Robot info
    id: String,
    version: String,
    model: String,

    // Position and navigation
    x: f64,
    y: f64,
    angle: f64,
    confidence: f64,

    // Battery
    battery_level: f64,
    battery_temp: f64,
    charging: bool,
    voltage: f64,
    current: f64,

    // Status
    is_blocked: bool,
    block_reason: Option<u8>,

    // Navigation
    nav_status: u32, // 0=none, 1=waiting, 2=running, 3=suspended, 4=completed, 5=failed
    nav_type: u32,
    target_id: String,
    target_point: [f64; 3],

    // Task queue for MoveToTargetList
    task_queue: Vec<NavTask>,
    current_task_index: usize,

    // Jack
    jack_height: f64,
    jack_has_payload: bool,
    jack_enabled: bool,

    // Odometry
    mileage: f64,
    total_time: f64,

    // Map
    current_map: String,
}

impl Default for RobotState {
    fn default() -> Self {
        Self {
            id: "MOCK_ROBOT_001".to_string(),
            version: "v1.0.0-mock".to_string(),
            model: "RBK-MOCK".to_string(),

            x: 0.0,
            y: 0.0,
            angle: 0.0,
            confidence: 0.98,

            battery_level: 0.85,
            battery_temp: 25.0,
            charging: false,
            voltage: 48.0,
            current: 2.5,

            is_blocked: false,
            block_reason: None,

            nav_status: 0,
            nav_type: 0,
            target_id: String::new(),
            target_point: [0.0, 0.0, 0.0],

            task_queue: Vec::new(),
            current_task_index: 0,

            jack_height: 0.0,
            jack_has_payload: false,
            jack_enabled: true,

            mileage: 1234.56,
            total_time: 3600000.0,

            current_map: "default_map".to_string(),
        }
    }
}

/// RBK frame structure
#[derive(Debug, Clone)]
struct RbkFrame {
    flow_no: u16,
    api_no: u16,
    body: String,
}

/// Encode RBK response
fn encode_response(api_no: u16, body_str: &str, flow_no: u16) -> BytesMut {
    let body_bytes = body_str.as_bytes();
    let body_len = body_bytes.len() as u32;
    let head_size = 16;

    let mut buf = BytesMut::with_capacity(head_size + body_bytes.len());

    buf.put_u8(START_MARK);
    buf.put_u8(PROTO_VERSION);
    buf.put_u16(flow_no);
    buf.put_u32(body_len);
    buf.put_u16(api_no);
    buf.put_slice(&[0u8; 6]); // reserved
    buf.put_slice(body_bytes);

    buf
}

/// Decode RBK request
struct RbkDecoder {
    started: bool,
    flow_no: u16,
    api_no: u16,
    body_size: i32,
}

impl RbkDecoder {
    fn new() -> Self {
        Self {
            started: false,
            flow_no: 0,
            api_no: 0,
            body_size: -1,
        }
    }

    fn decode(&mut self, buf: &mut BytesMut) -> Option<RbkFrame> {
        if !self.started {
            while buf.has_remaining() {
                if buf.get_u8() == START_MARK {
                    self.started = true;
                    break;
                }
            }
            if !self.started {
                return None;
            }
        }

        if self.body_size < 0 {
            if buf.remaining() < 15 {
                return None;
            }

            let _version = buf.get_u8();
            self.flow_no = buf.get_u16();
            self.body_size = buf.get_u32() as i32;
            self.api_no = buf.get_u16();
            buf.advance(6);
        }

        if buf.remaining() < self.body_size as usize {
            return None;
        }

        let body = if self.body_size == 0 {
            String::new()
        } else {
            let body_bytes = buf.split_to(self.body_size as usize);
            String::from_utf8_lossy(&body_bytes).to_string()
        };

        let frame = RbkFrame {
            flow_no: self.flow_no,
            api_no: self.api_no,
            body,
        };

        self.started = false;
        self.flow_no = 0;
        self.api_no = 0;
        self.body_size = -1;

        Some(frame)
    }
}

/// Get current timestamp
fn get_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("{}", now)
}

/// Handle API request and generate response
async fn handle_request(
    state: Arc<RwLock<RobotState>>,
    waypoints: Arc<RwLock<HashMap<String, Waypoint>>>,
    frame: RbkFrame,
) -> String {
    let api_no = frame.api_no;

    // State APIs (1000-1999)
    match api_no {
        1000 => {
            // CommonInfo - Robot information
            let s = state.read().await;
            json!({
                "id": s.id,
                "version": s.version,
                "model": s.model,
                "ret_code": 0,
                "err_msg": ""
            })
            .to_string()
        }
        1002 => {
            // OperationInfo - Running info
            let s = state.read().await;
            json!({
                "odo": s.mileage,
                "total": s.total_time,
                "total_time": s.total_time,
                "controller_temp": 35.5,
                "controller_humi": 45.0,
                "controller_voltage": 12.0,
                "ret_code": 0,
                "err_msg": ""
            })
            .to_string()
        }
        1004 => {
            // RobotPose - Location
            let s = state.read().await;
            json!({
                "x": s.x,
                "y": s.y,
                "angle": s.angle,
                "confidence": s.confidence,
                "ret_code": 0,
                "err_msg": ""
            })
            .to_string()
        }
        1005 => {
            // RobotSpeed
            json!({
                "vx": 0.5,
                "vy": 0.0,
                "w": 0.1,
                "ret_code": 0,
                "err_msg": ""
            })
            .to_string()
        }
        1006 => {
            // BlockStatus
            let s = state.read().await;
            json!({
                "blocked": s.is_blocked,
                "block_reason": s.block_reason,
                "block_x": null,
                "block_y": null,
                "ret_code": 0,
                "err_msg": ""
            })
            .to_string()
        }
        1007 => {
            // BatteryStatus
            let s = state.read().await;
            json!({
                "battery_level": s.battery_level,
                "battery_temp": s.battery_temp,
                "charging": s.charging,
                "voltage": s.voltage,
                "current": s.current,
                "ret_code": 0,
                "err_msg": ""
            })
            .to_string()
        }
        1020 => {
            // NavStatus
            let s = state.read().await;

            // Build finished/unfinished paths based on task queue
            let finished_path: Vec<String> = if s.task_queue.is_empty() {
                vec![]
            } else {
                s.task_queue
                    .iter()
                    .take(s.current_task_index)
                    .map(|t| t.target.clone())
                    .collect()
            };

            let unfinished_path: Vec<String> = if s.task_queue.is_empty() {
                vec![]
            } else {
                s.task_queue
                    .iter()
                    .skip(s.current_task_index + 1)
                    .map(|t| t.target.clone())
                    .collect()
            };

            json!({
                "task_status": s.nav_status,
                "task_type": s.nav_type,
                "target_id": s.target_id,
                "target_point": s.target_point,
                "finished_path": finished_path,
                "unfinished_path": unfinished_path,
                "move_status_info": "Mock navigation running",
                "ret_code": 0,
                "create_on": get_timestamp(),
                "err_msg": ""
            })
            .to_string()
        }
        1027 => {
            // JackStatus
            let s = state.read().await;
            json!({
                "jack_mode": true,
                "jack_enable": s.jack_enabled,
                "jack_error_code": 0,
                "jack_state": 4,
                "jack_isFull": s.jack_has_payload,
                "jack_speed": 0,
                "jack_emc": false,
                "jack_height": s.jack_height,
                "peripheral_data": [],
                "ret_code": 0,
                "err_msg": "",
                "create_on": get_timestamp()
            })
            .to_string()
        }
        1110 => {
            // TaskPackage
            let s = state.read().await;
            
            // Parse request body to get task_ids filter
            let requested_task_ids: Option<Vec<String>> = if frame.body.is_empty() {
                None // Field omitted - return most recent completed + all incomplete
            } else {
                serde_json::from_str::<serde_json::Value>(&frame.body)
                    .ok()
                    .and_then(|req| req.get("task_ids").cloned())
                    .and_then(|ids| serde_json::from_value(ids).ok())
            };
            
            // Build task status list based on request
            let task_status_list: Vec<serde_json::Value> = match requested_task_ids {
                Some(ids) if ids.is_empty() => {
                    // Empty array - return empty list
                    vec![]
                }
                Some(ids) => {
                    // Specific task_ids requested - filter to only those
                    s.task_queue
                        .iter()
                        .filter(|t| ids.contains(&t.task_id))
                        .map(|t| {
                            json!({
                                "task_id": t.task_id,
                                "status": t.status
                            })
                        })
                        .collect()
                }
                None => {
                    // Field omitted - return most recent completed + all incomplete
                    let mut tasks_to_return = Vec::new();
                    let mut found_last_completed = false;
                    
                    // Iterate in reverse to find most recent completed task
                    for task in s.task_queue.iter().rev() {
                        if task.status == 4 && !found_last_completed {
                            // Most recent completed task
                            tasks_to_return.push(task);
                            found_last_completed = true;
                        } else if task.status != 4 {
                            // All incomplete tasks (not completed)
                            tasks_to_return.push(task);
                        }
                    }
                    
                    // Reverse to maintain original order
                    tasks_to_return.reverse();
                    tasks_to_return
                        .into_iter()
                        .map(|t| {
                            json!({
                                "task_id": t.task_id,
                                "status": t.status
                            })
                        })
                        .collect()
                }
            };

            // Calculate percentage: (completed_tasks + progress_in_current) / total_tasks
            let percentage = if s.task_queue.is_empty() {
                0.0
            } else {
                let total_tasks = s.task_queue.len() as f64;
                let completed_tasks = s.current_task_index as f64;

                // Calculate progress within current task
                let current_task_progress = if s.current_task_index
                    < s.task_queue.len()
                    && s.nav_status == 2
                {
                    let current_task = &s.task_queue[s.current_task_index];
                    let target_x = current_task.target_pos[0];
                    let target_y = current_task.target_pos[1];
                    let start_x = current_task.start_pos[0];
                    let start_y = current_task.start_pos[1];

                    // Total distance for this task
                    let total_dist = ((target_x - start_x).powi(2)
                        + (target_y - start_y).powi(2))
                    .sqrt();

                    if total_dist > 0.01 {
                        // Distance covered
                        let covered_dist = ((s.x - start_x).powi(2)
                            + (s.y - start_y).powi(2))
                        .sqrt();
                        (covered_dist / total_dist).min(1.0)
                    } else {
                        1.0 // Already at target
                    }
                } else if s.nav_status == 4 {
                    // All completed
                    1.0
                } else {
                    0.0
                };

                ((completed_tasks + current_task_progress) / total_tasks)
                    .min(1.0)
            };

            // Calculate actual distance to current target
            let distance = if s.current_task_index < s.task_queue.len()
                && s.nav_status == 2
            {
                let current_task = &s.task_queue[s.current_task_index];
                let target_x = current_task.target_pos[0];
                let target_y = current_task.target_pos[1];
                let dx = target_x - s.x;
                let dy = target_y - s.y;
                (dx * dx + dy * dy).sqrt()
            } else {
                0.0
            };

            json!({
                "closest_target": if s.task_queue.is_empty() {
                    "".to_string()
                } else {
                    s.target_id.clone()
                },
                "source_name": "SELF_POSITION",
                "target_name": s.target_id,
                "percentage": percentage,
                "distance": distance,
                "task_status_list": task_status_list,
                "info": "Navigation in progress",
                "ret_code": 0,
                "err_msg": "",
                "create_on": get_timestamp()
            })
            .to_string()
        }
        1300 => {
            // Map info
            let s = state.read().await;
            json!({
                "current_map": s.current_map,
                "map_list": ["default_map", "warehouse_map"],
                "ret_code": 0,
                "err_msg": ""
            })
            .to_string()
        }

        // Control APIs (2000-2999)
        2000 => {
            // Stop
            let mut s = state.write().await;
            s.nav_status = 6; // Canceled
            // Don't clear task queue - keep history until new navigation starts
            json!({
                "ret_code": 0,
                "err_msg": "Stopped successfully"
            })
            .to_string()
        }
        2002 => {
            // Relocation
            json!({
                "ret_code": 0,
                "err_msg": "Relocation initiated"
            })
            .to_string()
        }
        2003 => {
            // Confirm location
            json!({
                "ret_code": 0,
                "err_msg": "Location confirmed"
            })
            .to_string()
        }
        2022 => {
            // Switch map
            let mut s = state.write().await;
            if let Ok(req) =
                serde_json::from_str::<serde_json::Value>(&frame.body)
            {
                if let Some(map_name) =
                    req.get("map_name").and_then(|v| v.as_str())
                {
                    s.current_map = map_name.to_string();
                }
            }
            json!({
                "ret_code": 0,
                "err_msg": "Map switched successfully"
            })
            .to_string()
        }

        // Navigation APIs (3000-3999)
        3001 => {
            // Pause navigation
            let mut s = state.write().await;
            s.nav_status = 3; // Suspended
            json!({
                "ret_code": 0,
                "err_msg": "Navigation paused"
            })
            .to_string()
        }
        3002 => {
            // Resume navigation
            let mut s = state.write().await;
            s.nav_status = 2; // Running
            json!({
                "ret_code": 0,
                "err_msg": "Navigation resumed"
            })
            .to_string()
        }
        3003 => {
            // Cancel navigation
            let mut s = state.write().await;
            s.nav_status = 6; // Canceled
            // Don't clear task queue - keep history until new navigation starts
            json!({
                "ret_code": 0,
                "err_msg": "Navigation canceled"
            })
            .to_string()
        }
        3051 => {
            // MoveToTarget - Single task navigation
            let mut s = state.write().await;
            let wp = waypoints.read().await;

            if let Ok(req) =
                serde_json::from_str::<serde_json::Value>(&frame.body)
            {
                if let Some(target) = req.get("id").and_then(|v| v.as_str()) {
                    // Clear old task queue - starting new navigation
                    s.task_queue.clear();
                    s.current_task_index = 0;
                    
                    let start = req.get("source_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("SELF_POSITION");
                    
                    let task_id = req.get("task_id")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "single_task".to_string());
                    
                    // Get positions from waypoints
                    let start_pos = if start == "SELF_POSITION" {
                        [s.x, s.y, s.angle]
                    } else {
                        wp.get(start).map(|w| [w.x, w.y, 0.0]).unwrap_or([s.x, s.y, s.angle])
                    };
                    
                    let target_pos = wp.get(target)
                        .map(|w| [w.x, w.y, 0.0])
                        .unwrap_or([start_pos[0] + 5.0, start_pos[1] + 5.0, 0.0]);
                    
                    // Create single task
                    s.task_queue.push(NavTask {
                        task_id,
                        start: start.to_string(),
                        target: target.to_string(),
                        start_pos,
                        target_pos,
                        status: 2, // Running
                    });
                    
                    s.nav_status = 2; // Running
                    s.nav_type = 3; // Path nav
                    s.target_id = target.to_string();
                    s.target_point = target_pos;
                }
            }

            json!({
                "ret_code": 0,
                "err_msg": "Navigation started",
                "create_on": get_timestamp()
            })
            .to_string()
        }
        3066 => {
            // MoveToTargetList
            let mut s = state.write().await;
            let wp = waypoints.read().await;

            if let Ok(req) =
                serde_json::from_str::<serde_json::Value>(&frame.body)
            {
                if let Some(task_list) =
                    req.get("move_task_list").and_then(|v| v.as_array())
                {
                    // Clear old task queue only when starting new navigation
                    s.task_queue.clear();
                    s.current_task_index = 0;

                    // Parse each task in the list
                    for (idx, task) in task_list.iter().enumerate() {
                        let target = task
                            .get("id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        let start = task
                            .get("source_id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("SELF_POSITION");
                        let task_id = task
                            .get("task_id")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| format!("task_{}", idx));

                        // Get positions from waypoints
                        let start_pos = if start == "SELF_POSITION" {
                            [s.x, s.y, s.angle]
                        } else {
                            wp.get(start)
                                .map(|w| [w.x, w.y, 0.0])
                                .unwrap_or([s.x, s.y, s.angle])
                        };

                        let target_pos =
                            wp.get(target).map(|w| [w.x, w.y, 0.0]).unwrap_or(
                                [start_pos[0] + 5.0, start_pos[1] + 5.0, 0.0],
                            );

                        s.task_queue.push(NavTask {
                            task_id,
                            start: start.to_string(),
                            target: target.to_string(),
                            start_pos,
                            target_pos,
                            status: if idx == 0 { 2 } else { 1 }, // First task running, others waiting
                        });
                    }

                    if !s.task_queue.is_empty() {
                        s.nav_status = 2; // Running
                        s.nav_type = 3; // Path nav
                        s.target_id = s.task_queue[0].target.clone();
                        s.target_point = s.task_queue[0].target_pos;
                    }
                }
            }

            json!({
                "ret_code": 0,
                "err_msg": "Path navigation started",
                "create_on": get_timestamp()
            })
            .to_string()
        }

        // Config APIs (4000-5999)
        4005 => {
            // Lock control
            json!({
                "ret_code": 0,
                "err_msg": "Control locked"
            })
            .to_string()
        }
        4006 => {
            // Unlock control
            json!({
                "ret_code": 0,
                "err_msg": "Control unlocked"
            })
            .to_string()
        }
        4009 => {
            // Clear all errors
            json!({
                "ret_code": 0,
                "err_msg": "All errors cleared"
            })
            .to_string()
        }
        4100 => {
            // Set params
            json!({
                "ret_code": 0,
                "err_msg": "Parameters set"
            })
            .to_string()
        }

        // Peripheral APIs (6000-6998)
        6000 => {
            // Play audio
            json!({
                "ret_code": 0,
                "err_msg": "Audio playing"
            })
            .to_string()
        }
        6001 => {
            // Set DO
            json!({
                "ret_code": 0,
                "err_msg": "DO set"
            })
            .to_string()
        }
        6070 => {
            // Jack load
            let mut s = state.write().await;
            s.jack_has_payload = true;
            s.jack_height = 0.2;
            json!({
                "ret_code": 0,
                "err_msg": "Jack loading"
            })
            .to_string()
        }
        6071 => {
            // Jack unload
            let mut s = state.write().await;
            s.jack_has_payload = false;
            s.jack_height = 0.0;
            json!({
                "ret_code": 0,
                "err_msg": "Jack unloading"
            })
            .to_string()
        }
        6072 => {
            // Jack stop
            json!({
                "ret_code": 0,
                "err_msg": "Jack stopped"
            })
            .to_string()
        }
        6073 => {
            // Set jack height
            let mut s = state.write().await;
            if let Ok(req) =
                serde_json::from_str::<serde_json::Value>(&frame.body)
            {
                if let Some(height) = req.get("height").and_then(|v| v.as_f64())
                {
                    s.jack_height = height;
                }
            }
            json!({
                "ret_code": 0,
                "err_msg": "Jack height set"
            })
            .to_string()
        }

        // Kernel APIs (5000, 5003, 5005 per KernelApi enum)
        5000 => {
            // Shutdown
            json!({
                "ret_code": 0,
                "err_msg": "Shutting down (mock)"
            })
            .to_string()
        }
        5003 => {
            // Reboot
            json!({
                "ret_code": 0,
                "err_msg": "Rebooting (mock)"
            })
            .to_string()
        }

        _ => {
            // Unknown API
            json!({
                "ret_code": 40000,
                "err_msg": format!("Unknown API: {}", api_no)
            })
            .to_string()
        }
    }
}

/// Handle a single client connection
async fn handle_client(
    mut stream: TcpStream,
    state: Arc<RwLock<RobotState>>,
    waypoints: Arc<RwLock<HashMap<String, Waypoint>>>,
    port: u16,
) {
    println!("New connection on port {}", port);

    let mut decoder = RbkDecoder::new();
    let mut buf = BytesMut::with_capacity(4096);
    let mut read_buf = vec![0u8; 4096];

    loop {
        match stream.read(&mut read_buf).await {
            Ok(0) => {
                println!("Connection closed on port {}", port);
                break;
            }
            Ok(n) => {
                buf.extend_from_slice(&read_buf[..n]);

                while let Some(frame) = decoder.decode(&mut buf) {
                    println!(
                        "Received API {} on port {}: {}",
                        frame.api_no, port, frame.body
                    );

                    let api_no = frame.api_no;
                    let flow_no = frame.flow_no;
                    let response_body =
                        handle_request(state.clone(), waypoints.clone(), frame)
                            .await;
                    let response_bytes =
                        encode_response(api_no, &response_body, flow_no);

                    if let Err(e) = stream.write_all(&response_bytes).await {
                        eprintln!("Failed to write response: {}", e);
                        break;
                    }

                    stream.flush().await.unwrap();
                }
            }
            Err(e) => {
                eprintln!("Read error on port {}: {}", port, e);
                break;
            }
        }
    }
}

/// Start a server on a specific port
async fn start_server(
    port: u16,
    state: Arc<RwLock<RobotState>>,
    waypoints: Arc<RwLock<HashMap<String, Waypoint>>>,
) {
    let addr = format!("0.0.0.0:{}", port);
    let listener = match TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind to {}: {}", addr, e);
            return;
        }
    };

    println!("Server listening on {}", addr);

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                let state = state.clone();
                let waypoints = waypoints.clone();
                tokio::spawn(async move {
                    handle_client(stream, state, waypoints, port).await;
                });
            }
            Err(e) => {
                eprintln!(
                    "Failed to accept connection on port {}: {}",
                    port, e
                );
            }
        }
    }
}

/// Background task to simulate robot state changes
async fn simulate_robot_behavior(state: Arc<RwLock<RobotState>>) {
    let mut interval =
        tokio::time::interval(tokio::time::Duration::from_millis(50));

    loop {
        interval.tick().await;

        let mut s = state.write().await;

        // Simulate battery drain
        if !s.charging && s.battery_level > 0.1 {
            s.battery_level -= 0.00005;
        }

        // Simulate navigation progress for task queue
        if s.nav_status == 2
            && !s.task_queue.is_empty()
            && s.current_task_index < s.task_queue.len()
        {
            let current_idx = s.current_task_index;
            let current_task = &s.task_queue[current_idx];
            let target_x = current_task.target_pos[0];
            let target_y = current_task.target_pos[1];
            let target_angle = current_task.target_pos[2];

            // Calculate distance to target
            let dx = target_x - s.x;
            let dy = target_y - s.y;
            let distance = (dx * dx + dy * dy).sqrt();

            // Movement speed: 0.1 units per tick (0.5s)
            let speed = 0.1;

            if distance > 0.05 {
                // Move towards target
                let move_ratio = speed / distance;
                s.x += dx * move_ratio;
                s.y += dy * move_ratio;
                s.mileage += speed;

                // Update task status
                s.task_queue[current_idx].status = 2; // Running
            } else {
                // Reached target - complete current task
                s.x = target_x;
                s.y = target_y;
                s.angle = target_angle;
                s.task_queue[current_idx].status = 4; // Completed

                // Move to next task
                s.current_task_index += 1;
                let next_idx = s.current_task_index;

                if next_idx < s.task_queue.len() {
                    // Start next task
                    s.task_queue[next_idx].status = 2; // Running
                    s.target_id = s.task_queue[next_idx].target.clone();
                    s.target_point = s.task_queue[next_idx].target_pos;
                    println!(
                        "Moving to next task: {} -> {}",
                        s.task_queue[next_idx].start,
                        s.task_queue[next_idx].target
                    );
                } else {
                    // All tasks completed
                    s.nav_status = 4; // Completed
                    println!("All navigation tasks completed!");
                }
            }
        }

        // Update total time
        s.total_time += 500.0;
    }
}

// HTTP API Handlers

/// POST /waypoints - Add waypoints
async fn add_waypoints(
    AxumState(state): AxumState<Arc<AppState>>,
    Json(waypoints): Json<Vec<Waypoint>>,
) -> StatusCode {
    let mut wp_store = state.waypoints.write().await;
    for wp in waypoints {
        wp_store.insert(wp.id.clone(), wp);
    }
    StatusCode::CREATED
}

/// GET /waypoints - Get all waypoints
async fn get_waypoints(
    AxumState(state): AxumState<Arc<AppState>>,
) -> Json<Vec<Waypoint>> {
    let wp_store = state.waypoints.read().await;
    let waypoints: Vec<Waypoint> = wp_store.values().cloned().collect();
    Json(waypoints)
}

/// DELETE /waypoints/:id - Delete waypoint by ID
async fn delete_waypoint(
    AxumState(state): AxumState<Arc<AppState>>,
    Path(id): Path<String>,
) -> StatusCode {
    let mut wp_store = state.waypoints.write().await;
    if wp_store.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

/// Start HTTP server for waypoint management
async fn start_http_server(state: Arc<AppState>) {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/waypoints", post(add_waypoints))
        .route("/waypoints", get(get_waypoints))
        .route("/waypoints/:id", delete(delete_waypoint))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind HTTP server");

    println!("Starting HTTP REST API on port 8080");

    axum::serve(listener, app)
        .await
        .expect("Failed to start HTTP server");
}

#[tokio::main]
async fn main() {
    println!("=== Mock RBK Robot Server ===");
    println!("Starting mock robot server on all ports...\n");

    let robot_state = Arc::new(RwLock::new(RobotState::default()));
    let waypoints = Arc::new(RwLock::new(HashMap::new()));

    // Initialize with some default waypoints
    {
        let mut wp = waypoints.write().await;
        wp.insert(
            "home".to_string(),
            Waypoint {
                id: "home".to_string(),
                x: 0.0,
                y: 0.0,
            },
        );
        wp.insert(
            "station_a".to_string(),
            Waypoint {
                id: "station_a".to_string(),
                x: 10.0,
                y: 5.0,
            },
        );
        wp.insert(
            "station_b".to_string(),
            Waypoint {
                id: "station_b".to_string(),
                x: -5.0,
                y: 10.0,
            },
        );
    }

    let app_state = Arc::new(AppState {
        robot: robot_state.clone(),
        waypoints: waypoints.clone(),
    });

    // Start behavior simulation
    let state_clone = robot_state.clone();
    tokio::spawn(async move {
        simulate_robot_behavior(state_clone).await;
    });

    // Start HTTP server for waypoint management
    let http_state = app_state.clone();
    tokio::spawn(async move {
        start_http_server(http_state).await;
    });

    // Start servers on all ports
    let ports = vec![
        (19204, "State APIs"),
        (19205, "Control APIs"),
        (19206, "Navigation APIs"),
        (19207, "Config APIs"),
        (19208, "Kernel APIs"),
        (19210, "Peripheral APIs"),
    ];

    let mut handles = vec![];

    for (port, name) in ports {
        println!("Starting {} on port {}", name, port);
        let state = robot_state.clone();
        let wp = waypoints.clone();
        let handle = tokio::spawn(async move {
            start_server(port, state, wp).await;
        });
        handles.push(handle);
    }

    println!("\n✓ All servers started successfully!");
    println!("  RBK Protocol: Connect to localhost with seersdk-rs client");
    println!("  HTTP REST API: http://localhost:8080");
    println!("    - POST   /waypoints");
    println!("    - GET    /waypoints");
    println!("    - DELETE /waypoints/{{id}}");
    println!("  Press Ctrl+C to stop\n");

    // Wait for all servers
    for handle in handles {
        let _ = handle.await;
    }
}

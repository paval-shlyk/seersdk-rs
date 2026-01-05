//! Mock RBK Robot Server
//!
//! This is a standalone binary that emulates a complete RBK robot server
//! with mock navigation logic. It implements all API endpoints across
//! multiple ports as per the RBK protocol specification.
//!
//! # Usage
//!
//! ```bash
//! cargo run --example mock_robot_server
//! ```
//!
//! The server will listen on:
//! - Port 19204: State APIs
//! - Port 19205: Control APIs
//! - Port 19206: Navigation APIs
//! - Port 19207: Config APIs
//! - Port 19208: Kernel APIs
//! - Port 19210: Peripheral APIs

use bytes::{Buf, BufMut, BytesMut};
use serde_json::json;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;

// Protocol constants
const START_MARK: u8 = 0x5A;
const PROTO_VERSION: u8 = 0x01;

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
            json!({
                "task_status": s.nav_status,
                "task_type": s.nav_type,
                "target_id": s.target_id,
                "target_point": s.target_point,
                "finished_path": [],
                "unfinished_path": [],
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
            json!({
                "closest_target": "station_1",
                "source_name": "SELF_POSITION",
                "target_name": s.target_id,
                "percentage": 0.5,
                "distance": 2.5,
                "task_status_list": [],
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
            json!({
                "ret_code": 0,
                "err_msg": "Navigation canceled"
            })
            .to_string()
        }
        3051 => {
            // MoveToTarget
            let mut s = state.write().await;

            if let Ok(req) =
                serde_json::from_str::<serde_json::Value>(&frame.body)
            {
                if let Some(target) = req.get("id").and_then(|v| v.as_str()) {
                    s.target_id = target.to_string();
                    s.nav_status = 2; // Running
                    s.nav_type = 3; // Path nav

                    // Simulate moving towards target (mock position update)
                    s.x += 0.1;
                    s.y += 0.1;
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
            s.nav_status = 2; // Running
            s.nav_type = 3;

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

        // Kernel APIs (7000-7999)
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
                        handle_request(state.clone(), frame).await;
                    let response_bytes =
                        encode_response(api_no, &response_body, flow_no);

                    if let Err(e) = stream.write_all(&response_bytes).await {
                        eprintln!("Failed to write response: {}", e);
                        break;
                    }
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
async fn start_server(port: u16, state: Arc<RwLock<RobotState>>) {
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
                tokio::spawn(async move {
                    handle_client(stream, state, port).await;
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
        tokio::time::interval(tokio::time::Duration::from_secs(1));

    loop {
        interval.tick().await;

        let mut s = state.write().await;

        // Simulate battery drain
        if !s.charging && s.battery_level > 0.1 {
            s.battery_level -= 0.0001;
        }

        // Simulate navigation progress
        if s.nav_status == 2 {
            // Running
            s.x += 0.01;
            s.y += 0.01;
            s.mileage += 0.01;

            // Random completion
            if s.mileage > 1250.0 {
                s.nav_status = 4; // Completed
            }
        }

        // Update total time
        s.total_time += 1000.0;
    }
}

#[tokio::main]
async fn main() {
    println!("=== Mock RBK Robot Server ===");
    println!("Starting mock robot server on all ports...\n");

    let state = Arc::new(RwLock::new(RobotState::default()));

    // Start behavior simulation
    let state_clone = state.clone();
    tokio::spawn(async move {
        simulate_robot_behavior(state_clone).await;
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
        let state = state.clone();
        let handle = tokio::spawn(async move {
            start_server(port, state).await;
        });
        handles.push(handle);
    }

    println!("\n✓ All servers started successfully!");
    println!("  Connect to localhost with seersdk-rs client");
    println!("  Press Ctrl+C to stop\n");

    // Wait for all servers
    for handle in handles {
        let _ = handle.await;
    }
}

//! API request types for RBK robot communication
//!
//! This module defines the API request enum that categorizes all RBK APIs
//! into their respective modules based on the RBK protocol specification.

mod request;
mod response;

pub use request::*;
pub use response::*;

/// API request enum representing all RBK robot APIs
///
/// The RBK protocol organizes APIs into modules, each with its own port:
/// - State APIs (1000-1999): Robot state queries on port 19204
/// - Control APIs (2000-2999): Robot control commands on port 19205
/// - Navigation APIs (3000-3999): Navigation commands on port 19206
/// - Config APIs (4000-5999): Configuration management on port 19207
/// - Kernel APIs (7000-7999): Kernel operations on port 19208
/// - Misc APIs (6000-6998): Miscellaneous operations on port 19210
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ApiRequest {
    /// State module APIs (1000-1999)
    State(StateApi),
    /// Control module APIs (2000-2999)
    Control(ControlApi),
    /// Navigation module APIs (3000-3999)
    Nav(NavApi),
    /// Config module APIs (4000-5999)
    Config(ConfigApi),
    /// Kernel module APIs (7000-7999)
    Kernel(KernelApi),
    /// Misc module APIs (6000-6998)
    Misc(MiscApi),
}

impl ApiRequest {
    /// Get the API number for this request
    pub fn api_no(&self) -> u16 {
        match *self {
            ApiRequest::State(api) => api as u16,
            ApiRequest::Control(api) => api as u16,
            ApiRequest::Nav(api) => api as u16,
            ApiRequest::Config(api) => api as u16,
            ApiRequest::Kernel(api) => api as u16,
            ApiRequest::Misc(api) => api as u16,
        }
    }
}

//generate documetation and dto types
//impl_api_request!(RobotInfoRequest, ApiRequest::State(RobotInfo), res: StatusMessage)
macro_rules! impl_api_request {
    // Pattern for requests without payload
    ($req_type:ident, $api_variant:expr, res: $res_type:ty) => {
        #[derive(Debug, Clone, Default)]
        pub struct $req_type;

        impl $req_type {
            pub fn new() -> Self {
                Self
            }
        }

        impl $crate::api::ToRequestBody for $req_type {
            fn to_request_body(&self) -> String {
                String::new()
            }

            fn to_api_request(&self) -> ApiRequest {
                $api_variant
            }
        }

        impl $crate::api::FromResponseBody for $req_type {
            type Response = $res_type;
        }
    };
    // Pattern for requests with payload
    ($req_type:ident, $api_variant:expr, req: $req_body_type:ty, res: $res_type:ty) => {
        #[derive(Debug, Clone)]
        pub struct $req_type {
            pub req_body: $req_body_type,
        }

        impl $req_type {
            pub fn new(req_body: $req_body_type) -> Self {
                Self { req_body }
            }
        }

        impl $crate::api::ToRequestBody for $req_type {
            fn to_request_body(&self) -> String {
                serde_json::to_string(&self.req_body).unwrap_or_default()
            }

            fn to_api_request(&self) -> ApiRequest {
                $api_variant
            }
        }

        impl $crate::api::FromResponseBody for $req_type {
            type Response = $res_type;
        }
    };
}

impl_api_request!(RobotInfoRequest, ApiRequest::State(StateApi::RobotInfo), res: StatusMessage);
impl_api_request!(RobotRunStatusRequest, ApiRequest::State(StateApi::RobotRunStatus), res: StatusMessage);
impl_api_request!(RobotModeRequest, ApiRequest::State(StateApi::RobotMode), res: StatusMessage);
impl_api_request!(RobotLocationRequest, ApiRequest::State(StateApi::RobotLocation), res: StatusMessage);
impl_api_request!(RobotSpeedRequest, ApiRequest::State(StateApi::RobotSpeed), res: StatusMessage);
impl_api_request!(RobotBlockStatusRequest, ApiRequest::State(StateApi::RobotBlockStatus), res: StatusMessage);
impl_api_request!(RobotBatteryStatusRequest, ApiRequest::State(StateApi::RobotBatteryStatus), res: StatusMessage);
impl_api_request!(RobotBrakeStatusRequest, ApiRequest::State(StateApi::RobotBrakeStatus), res: StatusMessage);
impl_api_request!(RobotLidarDataRequest, ApiRequest::State(StateApi::RobotLidarData), res: StatusMessage);
impl_api_request!(RobotPathDataRequest, ApiRequest::State(StateApi::RobotPathData), res: StatusMessage);
impl_api_request!(RobotCurrentAreaRequest, ApiRequest::State(StateApi::RobotCurrentArea), res: StatusMessage);
impl_api_request!(RobotEmergencyStatusRequest, ApiRequest::State(StateApi::RobotEmergencyStatus), res: StatusMessage);
impl_api_request!(RobotIODataRequest, ApiRequest::State(StateApi::RobotIOData), res: StatusMessage);
impl_api_request!(RobotTaskStatusRequest, ApiRequest::State(StateApi::RobotTaskStatus), res: StatusMessage);
impl_api_request!(RobotRelocationStatusRequest, ApiRequest::State(StateApi::RobotRelocationStatus), res: StatusMessage);
impl_api_request!(RobotLoadMapStatusRequest, ApiRequest::State(StateApi::RobotLoadMapStatus), res: StatusMessage);
impl_api_request!(RobotSlamStatusRequest, ApiRequest::State(StateApi::RobotSlamStatus), res: StatusMessage);
impl_api_request!(RobotAlarmStatusRequest, ApiRequest::State(StateApi::RobotAlarmStatus), res: StatusMessage);
impl_api_request!(RobotAllStatus1Request, ApiRequest::State(StateApi::RobotAllStatus1), res: StatusMessage);
impl_api_request!(RobotAllStatus2Request, ApiRequest::State(StateApi::RobotAllStatus2), res: StatusMessage);
impl_api_request!(RobotAllStatus3Request, ApiRequest::State(StateApi::RobotAllStatus3), res: StatusMessage);
impl_api_request!(RobotInitStatusRequest, ApiRequest::State(StateApi::RobotInitStatus), res: StatusMessage);
impl_api_request!(RobotMapInfoRequest, ApiRequest::State(StateApi::RobotMapInfo), res: StatusMessage);
impl_api_request!(RobotParamsRequest, ApiRequest::State(StateApi::RobotParams), res: StatusMessage);

// Control API requests
impl_api_request!(StartExerciseRequest, ApiRequest::Control(ControlApi::StartExercise), res: StatusMessage);
impl_api_request!(StopExerciseRequest, ApiRequest::Control(ControlApi::StopExercise), res: StatusMessage);
impl_api_request!(GyroCalibrateRequest, ApiRequest::Control(ControlApi::GyroCalibrate), res: StatusMessage);
impl_api_request!(RelocateRequest, ApiRequest::Control(ControlApi::Relocate), res: StatusMessage);
impl_api_request!(ConfirmLocationRequest, ApiRequest::Control(ControlApi::ConfirmLocation), res: StatusMessage);
impl_api_request!(OpenLoopMotionRequest, ApiRequest::Control(ControlApi::OpenLoopMotion), res: StatusMessage);
impl_api_request!(StartSlamRequest, ApiRequest::Control(ControlApi::StartSlam), res: StatusMessage);
impl_api_request!(StopSlamRequest, ApiRequest::Control(ControlApi::StopSlam), res: StatusMessage);
impl_api_request!(SwitchMapRequest, ApiRequest::Control(ControlApi::SwitchMap), res: StatusMessage);
impl_api_request!(ReloadMapObjectsRequest, ApiRequest::Control(ControlApi::ReloadMapObjects), res: StatusMessage);

// Navigation API requests
impl_api_request!(PausTaskRequest, ApiRequest::Nav(NavApi::PausTask), res: StatusMessage);
impl_api_request!(ResumeTaskRequest, ApiRequest::Nav(NavApi::ResumeTask), res: StatusMessage);
impl_api_request!(CancelTaskRequest, ApiRequest::Nav(NavApi::CancelTask), res: StatusMessage);
impl_api_request!(MoveToPointRequest, ApiRequest::Nav(NavApi::MoveToPoint), res: StatusMessage);
impl_api_request!(MoveToTargetRequest, ApiRequest::Nav(NavApi::MoveToTarget), res: StatusMessage);
impl_api_request!(PatrolRequest, ApiRequest::Nav(NavApi::Patrol), res: StatusMessage);
impl_api_request!(TranslateRequest, ApiRequest::Nav(NavApi::Translate), res: StatusMessage);
impl_api_request!(TurnRequest, ApiRequest::Nav(NavApi::Turn), res: StatusMessage);

// Config API requests
impl_api_request!(SwitchModeRequest, ApiRequest::Config(ConfigApi::SwitchMode), res: StatusMessage);
impl_api_request!(SetConfigRequest, ApiRequest::Config(ConfigApi::SetConfig), res: StatusMessage);
impl_api_request!(SaveConfigRequest, ApiRequest::Config(ConfigApi::SaveConfig), res: StatusMessage);
impl_api_request!(ReloadConfigRequest, ApiRequest::Config(ConfigApi::ReloadConfig), res: StatusMessage);

// Kernel API requests
impl_api_request!(ShutdownRequest, ApiRequest::Kernel(KernelApi::Shutdown), res: StatusMessage);
impl_api_request!(RebootRequest, ApiRequest::Kernel(KernelApi::Reboot), res: StatusMessage);
impl_api_request!(ResetFirmwareRequest, ApiRequest::Kernel(KernelApi::ResetFirmware), res: StatusMessage);

// Misc API requests
impl_api_request!(SpeakerRequest, ApiRequest::Misc(MiscApi::Speaker), res: StatusMessage);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum StateApi {
    /// Query robot information
    RobotInfo = 1000,
    /// Query the operating status information of the robot (such as running time, mileage, etc.)
    RobotRunStatus = 1002,
    /// Query the operating mode of the robot
    RobotMode = 1003,
    /// Query the location of the robot
    RobotLocation = 1004,
    /// Query robot speed
    RobotSpeed = 1005,
    /// Query the blocked status of the robot
    RobotBlockStatus = 1006,
    /// Check the status of the robot battery
    RobotBatteryStatus = 1007,
    /// Check the status of the robot brake
    RobotBrakeStatus = 1008,
    /// Query robot lidar data
    RobotLidarData = 1009,
    /// Query robot path data
    RobotPathData = 1010,
    /// Query the current area of the robot
    RobotCurrentArea = 1011,
    /// Query the emergency stop status of the robot
    RobotEmergencyStatus = 1012,
    /// Query robot IO data
    RobotIOData = 1013,
    /// Query the status of the robot task, the task site,Task-related paths, etc.
    RobotTaskStatus = 1020,
    /// Query the relocation status of the robot
    RobotRelocationStatus = 1021,
    /// Query the loading status of the robot map
    RobotLoadMapStatus = 1022,
    /// Query the status of the robot scanning pictures
    RobotSlamStatus = 1025,
    /// Query the alarm status of the robot
    RobotAlarmStatus = 1050,
    /// Query batch data 1
    RobotAllStatus1 = 1100,
    /// Query batch data 2
    RobotAllStatus2 = 1101,
    /// Query batch data 3
    RobotAllStatus3 = 1102,
    /// Query the initialization status of the robot
    RobotInitStatus = 1111,
    /// Query the maps loaded by the robot and the stored maps
    RobotMapInfo = 1300,
    /// Query robot parameters
    RobotParams = 1400,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ControlApi {
    /// Start exercising
    StartExercise = 2000,
    /// Stop exercising
    StopExercise = 2001,
    /// Calibrate the gyroscope
    GyroCalibrate = 2002,
    /// Relocate
    Relocate = 2003,
    /// Confirm that the positioning is correct
    ConfirmLocation = 2004,
    /// Open loop movement
    OpenLoopMotion = 2010,
    /// Start scanning the map
    StartSlam = 2020,
    /// Stop scanning the map
    StopSlam = 2021,
    /// Switch loaded maps
    SwitchMap = 2022,
    /// Reload elements in the map
    ReloadMapObjects = 2023,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum NavApi {
    /// Pause the current task
    PausTask = 3001,
    /// Resume the current task
    ResumeTask = 3002,
    /// Cancel the current task
    CancelTask = 3003,
    /// Free navigation (freely plan path navigation based on coordinate values or sites on the
    /// map)
    MoveToPoint = 3050,
    /// Fixed path navigation (based on the site on the map and fixed path navigation)
    MoveToTarget = 3051,
    /// Inspection (set the route for fixed path navigation)
    Patrol = 3052,
    /// Flat motion, linear motion at a fixed speed and a fixed distance
    Translate = 3055,
    /// Rotate, rotate at a fixed angle at a fixed angular velocity
    Turn = 3056,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ConfigApi {
    /// Switch operating mode (manual, automatic)
    SwitchMode = 4000,
    /// Set configuration parameters
    SetConfig = 4001,
    /// Set and save configuration parameters
    SaveConfig = 4002,
    /// Load configuration parameters
    ReloadConfig = 4003,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum KernelApi {
    /// Turn off the robot, the robot will lose power and lose control
    Shutdown = 5000,
    /// Restart the robot, the connection will be disconnected during the restart
    Reboot = 5003,
    /// Reset the robot firmware
    ResetFirmware = 5005,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum MiscApi {
    Speaker = 6000,
}

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
    ($req_type:ident, $api_variant:expr, $(req: $req_body:expr,)? res: $res_type:ty) => {
        #[derive(Debug, Clone)]
        pub struct $req_type {
            $(
                pub req_body: String,
            )?
        }

        impl $req_type {
            pub fn new($(req_body: impl ToString,)?) -> Self {
                Self {
                    $(
                        req_body: req_body.to_string(),
                    )?
                }
            }
        }

        impl $crate::api::ToRequestBody for $req_type {
            fn to_request_body(&self) -> String {
                $(
                    self.req_body.clone()
                )?
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

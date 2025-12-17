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
/// - Push APIs (9000+): Push configuration and push data
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
    Peripheral(PeripheralApi),
    /// Push module APIs (9000+)
    Push(PushApi),
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
            ApiRequest::Peripheral(api) => api as u16,
            ApiRequest::Push(api) => api as u16,
        }
    }
}

/// Macro to generate request DTO types for RBK robot APIs
///
/// This macro creates a request type with associated traits for serialization and response handling.
///
/// # Patterns
///
/// 1. Request without payload (returns empty string):
/// ```ignore
/// impl_api_request!(RequestTypeName, ApiRequest::Module(ModuleApi::Variant), res: ResponseType);
/// ```
///
/// 2. Request with payload (serializes payload to JSON):
/// ```ignore
/// impl_api_request!(RequestTypeName, ApiRequest::Module(ModuleApi::Variant), req: PayloadType, res: ResponseType);
/// ```
///
/// # Arguments
///
/// * `$req_type` - Name of the request type to generate
/// * `$api_variant` - The API variant expression (e.g., `ApiRequest::State(StateApi::RobotInfo)`)
/// * `$req_body_type` - (Optional) Type of the request payload for requests that need a body
/// * `$res_type` - Type of the response that will be returned
/// * `$docs` - (Optional) Documentation string for the generated request type
macro_rules! impl_api_request {
    // Pattern for requests without payload
    ($req_type:ident, $api_variant:expr, res: $res_type:ty $(, $docs:literal)?) => {
        $(#[doc = $docs])?
        #[derive(Debug, Clone, Default)]
        pub struct $req_type;

        impl $req_type {
            pub fn new() -> Self {
                Self
            }
        }

        impl $crate::api::ToRequestBody for $req_type {
            fn to_request_body(&self) -> Result<String, serde_json::Error> {
                Ok(String::new())
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
    ($req_type:ident, $api_variant:expr, req: $req_body_type:ty, res: $res_type:ty $(, $docs:literal)?) => {
        $(#[doc = $docs])?
        #[derive(Debug, Clone)]
        pub struct $req_type {
            pub req_body: $req_body_type,
        }

        impl $req_type {
            pub fn new(req_body: $req_body_type) -> Self {
                Self { req_body }
            }
        }

        impl $req_body_type {
            pub fn into_request(self) -> $req_type {
                $req_type { req_body: self }
            }
        }

        impl $crate::api::ToRequestBody for $req_type {
            fn to_request_body(&self) -> Result<String, serde_json::Error> {
                serde_json::to_string(&self.req_body)
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

// State API requests
impl_api_request!(CommonInfoRequest, ApiRequest::State(StateApi::Info), res: CommonInfo);
impl_api_request!(OperationInfoRequest, ApiRequest::State(StateApi::Run), res: OperationInfo);
impl_api_request!(RobotPoseRequest, ApiRequest::State(StateApi::Loc), res: RobotPose);
impl_api_request!(RobotSpeedRequest, ApiRequest::State(StateApi::Speed), res: StatusMessage);
impl_api_request!(BlockStatusRequest, ApiRequest::State(StateApi::Block), res: BlockStatus);
impl_api_request!(BatteryStatusRequest, ApiRequest::State(StateApi::Battery), res: BatteryStatus);
impl_api_request!(RobotLidarDataRequest, ApiRequest::State(StateApi::Laser), res: StatusMessage);
impl_api_request!(RobotCurrentAreaRequest, ApiRequest::State(StateApi::Area), res: StatusMessage);
impl_api_request!(RobotEmergencyStatusRequest, ApiRequest::State(StateApi::Emergency), res: StatusMessage);
impl_api_request!(RobotIODataRequest, ApiRequest::State(StateApi::Io), res: StatusMessage);
impl_api_request!(RobotTaskStatusRequest, ApiRequest::State(StateApi::Task), res: StatusMessage);
impl_api_request!(RobotRelocationStatusRequest, ApiRequest::State(StateApi::Reloc), res: StatusMessage);
impl_api_request!(RobotLoadMapStatusRequest, ApiRequest::State(StateApi::Loadmap), res: StatusMessage);
impl_api_request!(RobotSlamStatusRequest, ApiRequest::State(StateApi::Slam), res: StatusMessage);
impl_api_request!(JackStatusRequest, ApiRequest::State(StateApi::Jack), res: StatusMessage);
impl_api_request!(RobotAlarmStatusRequest, ApiRequest::State(StateApi::Alarm), res: StatusMessage);
impl_api_request!(RobotAllStatus1Request, ApiRequest::State(StateApi::All1), res: StatusMessage);
impl_api_request!(RobotAllStatus2Request, ApiRequest::State(StateApi::All2), res: StatusMessage);
impl_api_request!(RobotAllStatus3Request, ApiRequest::State(StateApi::All3), res: StatusMessage);
impl_api_request!(RobotMapInfoRequest, ApiRequest::State(StateApi::Map), res: StatusMessage);
impl_api_request!(RobotParamsRequest, ApiRequest::State(StateApi::Params), res: StatusMessage);

// Control API requests
impl_api_request!(StopExerciseRequest, ApiRequest::Control(ControlApi::Stop), res: StatusMessage);
impl_api_request!(RelocateRequest, ApiRequest::Control(ControlApi::Reloc), res: StatusMessage);
impl_api_request!(ConfirmLocationRequest, ApiRequest::Control(ControlApi::Comfirmloc), res: StatusMessage);
impl_api_request!(OpenLoopMotionRequest, ApiRequest::Control(ControlApi::Motion), res: StatusMessage);
impl_api_request!(SwitchMapRequest, ApiRequest::Control(ControlApi::Loadmap), res: StatusMessage);

// Navigation API requests
impl_api_request!(PauseTaskRequest, ApiRequest::Nav(NavApi::Pause), res: StatusMessage);
impl_api_request!(ResumeTaskRequest, ApiRequest::Nav(NavApi::Resume), res: StatusMessage);
impl_api_request!(CancelTaskRequest, ApiRequest::Nav(NavApi::Cancel), res: StatusMessage);
impl_api_request!(MoveToTargetRequest, ApiRequest::Nav(NavApi::MoveToTarget), req: MoveToTarget, res: StatusMessage);
impl_api_request!(TranslateRequest, ApiRequest::Nav(NavApi::Translate), res: StatusMessage);
impl_api_request!(TurnRequest, ApiRequest::Nav(NavApi::Turn), res: StatusMessage);

// Peripheral API requests
impl_api_request!(LoadJackRequest, ApiRequest::Peripheral(PeripheralApi::JackLoad), res: StatusMessage);
impl_api_request!(UnloadJackRequest, ApiRequest::Peripheral(PeripheralApi::JackUnload), res: StatusMessage);
impl_api_request!(StopJackRequest, ApiRequest::Peripheral(PeripheralApi::JackStop), res: StatusMessage);
impl_api_request!(SetJackHeightRequest, ApiRequest::Peripheral(PeripheralApi::JackSetHeight), req: SetJackHeight, res: StatusMessage);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum StateApi {
    /// Query Robot Information
    Info = 1000,
    /// Query Robot Running Information
    Run = 1002,
    /// Query Robot Location
    Loc = 1004,
    /// Query Robot Speed
    Speed = 1005,
    /// Query Robot Blocked Status
    Block = 1006,
    /// Query Robot Battery Status
    Battery = 1007,
    /// Query Robot Laser Status
    Laser = 1009,
    /// Query Robot Area Status
    Area = 1011,
    /// Query Robot Estop Status
    Emergency = 1012,
    /// Query Robot I/O Status
    Io = 1013,
    /// Query Robot IMU Data
    Imu = 1014,
    /// Query Robot RFID Data
    Rfid = 1015,
    /// Query Robot Ultrasonic Status
    Ultrasonic = 1016,
    /// Query Robot PGV Data
    Pgv = 1017,
    /// Query Robot Encoder Status
    Encoder = 1018,
    /// Query Robot Navigation Status
    Task = 1020,
    /// Query Robot Localization Status
    Reloc = 1021,
    /// Query Robot Map Loading Status
    Loadmap = 1022,
    /// Query Scanning Status of Robot
    Slam = 1025,
    /// Query Robot Jacking Status
    Jack = 1027,
    /// Query Robot Fork Status
    Fork = 1028,
    /// Query Robot Roller Status
    Roller = 1029,
    /// Query Robot Motor Status
    Motor = 1040,
    /// Query Robot Alarm Status
    Alarm = 1050,
    /// Query Robot Current Lock
    CurrentLock = 1060,
    /// Query Modbus Data
    Modbus = 1071,
    /// Query Batch Data 1
    All1 = 1100,
    /// Query Batch Data 2
    All2 = 1101,
    /// Query Batch Data 3
    All3 = 1102,
    /// Query Robot Task Status Package
    TaskStatusPackage = 1110,
    /// Query Loaded Map and Stored Map
    Map = 1300,
    /// Query Station Information of Currently Loaded Map
    Station = 1301,
    /// Query MD5 Value of Specified Map List
    Mapmd5 = 1302,
    /// Query the Path between Any Two Points
    GetPath = 1303,
    /// Query Robot Parameters
    Params = 1400,
    /// Download the Robot Model File
    Model = 1500,
    /// Query List of Robot Scripts
    ScriptInfo = 1506,
    /// Query List of Robot Script Details
    ScriptDetailslist = 1507,
    /// Query Default Parameters of Robot Script
    ScriptArgs = 1508,
    /// Query Robot Support Calibration List
    CalibSupportList = 1509,
    /// Query Robot Calibration Status
    CalibStatus = 1510,
    /// Query Robot Calibration File
    CalibData = 1511,
    /// Query 3D QR Code During Mapping
    Tag3d = 1665,
    /// Query Status of Robotic Arm
    Armstatus = 1669,
    /// Calculate Coordinate Transformation of Robotic Arms
    Armcalculate = 1670,
    /// Robotic Arm binTask
    Armtask = 1671,
    /// Robotic Arm Motion Control
    Armmove = 1673,
    /// Robotic Arm Teaching Panel Control
    Armoperation = 1674,
    /// Query the Point Cloud Image of the Currently Recognized Camera
    Cloudprojection = 1675,
    /// Emulation from File Recognition
    Recofiles = 1676,
    /// Query Driver Params
    Canframe = 1750,
    /// Query GNSS Connection Status
    Gnsscheck = 1760,
    /// Query List of GNSS Devices
    GnssList = 1761,
    /// Query List of Robot Files
    Listfile = 1798,
    /// Upload the Robot File
    Uploadfile = 1799,
    /// Download the Robot File
    Downloadfile = 1800,
    /// Query Storage Bin Information Seen by Robot
    Bins = 1803,
    /// Query Robot Sound Status
    Sound = 1850,
    /// Download Handle Custom Binding Event
    JoystickKeymap = 1852,
    /// Query Transparent Data
    TransparentData = 1900,
    /// Run Start Battery Script
    Startbatteryscript = 1901,
    /// Stop Robot Battery Script
    Stopbatteryscript = 1902,
    /// Start Ambient Lamp Script
    Startdmxscript = 1903,
    /// Stop Ambient Lamp Script
    Stopdmxscript = 1904,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ControlApi {
    /// Stop Open Loop Motion
    Stop = 2000,
    /// Relocation
    Reloc = 2002,
    /// Confirm Correct Location
    Comfirmloc = 2003,
    /// Cancel Relocation
    Cancelreloc = 2004,
    /// Open Loop Motion
    Motion = 2010,
    /// Switch Map
    Loadmap = 2022,
    /// Clear Motor Encoder
    Clearmotorencoder = 2024,
    /// Upload and Load Map
    UploadAndLoadmap = 2025,
    /// Clear Weight Sensor Value
    ClearWeightdevvalue = 2026,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum NavApi {
    /// Pause Navigation
    Pause = 3001,
    /// Resume Navigation
    Resume = 3002,
    /// Cancel Navigation
    Cancel = 3003,
    /// Path Navigation
    MoveToTarget = 3051,
    /// Get Navigation Path
    TargetPath = 3053,
    /// Translation
    Translate = 3055,
    /// Rotation
    Turn = 3056,
    /// Tray Rotation
    Spin = 3057,
    /// Circular Motion
    Circular = 3058,
    /// Enable and Disable Paths
    Path = 3059,
    /// Designated Path Navigation
    MoveToTargetlist = 3066,
    /// Clear Specified Path Navigation
    Cleartargetlist = 3067,
    /// Clear Specified Navigation Path with Task ID
    Safeclearmovements = 3068,
    /// Query Task Chain
    TasklistStatus = 3101,
    /// Execute Pre-Stored Tasks
    TasklistName = 3106,
    /// Query Robot Task Chain List
    TasklistList = 3115,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ConfigApi {
    /// Preempt Control
    Lock = 4005,
    /// Release Control
    Unlock = 4006,
    /// Clear Robot's All Errors
    Clearallerrors = 4009,
    /// Load Map to Robot
    Uploadmap = 4010,
    /// Download Maps from Robots
    Downloadmap = 4011,
    /// Delete Map in Robot
    Removemap = 4012,
    /// Upload Robot Script
    Uploadscript = 4021,
    /// Download Robot Script
    Downloadscript = 4022,
    /// Delete Robot Script
    Removescript = 4023,
    /// Configure Robot Push Port
    Push = 4091,
    /// Set Robot Params Temporarily
    Setparams = 4100,
    /// Set Robot Params Permanently
    Saveparams = 4101,
    /// Restore Robot Params
    Reloadparams = 4102,
    /// Configure Ultrasonic
    Ultrasonic = 4130,
    /// Configure DI
    Di = 4140,
    /// Motor Calibration
    MotorCalib = 4150,
    /// Motor Clear Fault
    MotorClearFault = 4151,
    /// Upload the Model File to Robot
    Model = 4200,
    /// Set up Calibration Process Data
    CalibPushData = 4201,
    /// Confirmation of Calibration Data
    CalibConfirm = 4202,
    /// Clear Calibration Data according to Calibration Type
    CalibClear = 4203,
    /// Clear the Robot.cp File
    CalibClearAll = 4209,
    /// Add Dynamic Obstacles (Robot Coordinate System)
    Addobstacle = 4350,
    /// Add Dynamic Obstacles (World Coordinate System)
    Addgobstacle = 4351,
    /// Remove Dynamic Obstacles
    Removeobstacle = 4352,
    /// 3D QR Code Mapping
    Tag3dmapping = 4353,
    /// Clear Goods Shape
    ClearGoodsshape = 4356,
    /// Set Shelf Description File
    SetShelfshape = 4357,
    /// Set Driver Params
    SendCanframe = 4400,
    /// Reset Running Info
    ClearOdo = 4450,
    /// Reset GNSS Configuration
    ResetGnss = 4460,
    /// Set GNSS Baudrate
    SetGnssBaudrate = 4461,
    /// Set GNSS to Rover mode
    SetGnssRover = 4462,
    /// Upload Handle Custom Binding Event
    JoystickBindKeymap = 4470,
    /// Set Third-Party Error
    Seterror = 4800,
    /// Clear Third-Party Error
    Clearerror = 4801,
    /// Set Third-Party Warning
    Setwarning = 4802,
    /// Clear Third-Party Warning
    Clearwarning = 4803,
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
pub enum PeripheralApi {
    /// Play Audio
    PlayAudio = 6000,
    /// Set DO
    Setdo = 6001,
    /// Batch Set DO
    Setdos = 6002,
    /// Set Relay
    Setrelay = 6003,
    /// Soft Estop
    Softemc = 6004,
    /// Set Charging Relay
    Setchargingrelay = 6005,
    /// Pause Audio
    PauseAudio = 6010,
    /// Resume Audio
    ResumeAudio = 6011,
    /// Stop Playing Audio
    StopAudio = 6012,
    /// Set Virtual DI
    Setvdi = 6020,
    /// Upload Audio Files
    UploadAudio = 6030,
    /// Download Audio Files
    DownloadAudio = 6031,
    /// Get Audio File List
    AudioList = 6033,
    /// Set Fork Height
    SetForkHeight = 6040,
    /// Stop Fork Motion
    StopFork = 6041,
    /// Write Peripheral User-defined Data
    WritePeripheralData = 6049,
    /// Roller (belt) Front Roll
    RollerFrontRoll = 6051,
    /// Roller (belt) Back Roll
    RollerBackRoll = 6052,
    /// Roller (belt) Left Roll
    RollerLeftRoll = 6053,
    /// Roller (belt) Right Roll
    RollerRightRoll = 6054,
    /// Roller (belt) Front Load
    RollerFrontLoad = 6055,
    /// Roller (belt) Front Unload
    RollerFrontUnload = 6056,
    /// Roller (belt) Front Pre-Load
    RollerFrontPreLoad = 6057,
    /// Roller (belt) Back Load
    RollerBackLoad = 6058,
    /// Roller (belt) Back Unload
    RollerBackUnload = 6059,
    /// Roller (belt) Back Pre-Load
    RollerBackPreLoad = 6060,
    /// Roller (belt) Left Load
    RollerLeftLoad = 6061,
    /// Roller (belt) Left Unload
    RollerLeftUnload = 6062,
    /// Roller (belt) Right Load
    RollerRightLoad = 6063,
    /// Roller (belt) Right Unload
    RollerRightUnload = 6064,
    /// Roller (belt) Left Pre-Load
    RollerLeftPreLoad = 6065,
    /// Roller (belt) Right Pre-Load
    RollerRightPreLoad = 6066,
    /// Roller (belt) Stop
    RollerStop = 6067,
    /// Roller (belt) Inverse Left and Right
    RollerLeftRightInverse = 6068,
    /// Roller (belt) Inverse Front and Back
    RollerFrontBackInverse = 6069,
    /// Jacking Load
    JackLoad = 6070,
    /// Jacking Unload
    JackUnload = 6071,
    /// Jacking Stop
    JackStop = 6072,
    /// Jacking Height
    JackSetHeight = 6073,
    /// Clear Cargo Status
    ResetCargo = 6080,
    /// Hook Load
    HookLoad = 6082,
    /// Hook Unload
    HookUnload = 6083,
    /// Write modbus Data
    SetModbus = 6086,
    /// Start Map Scanning
    Slam = 6100,
    /// End SLAM
    Endslam = 6101,
    /// Start Calibration
    Calibrate = 6110,
    /// Cancel Calibration
    Endcalibrate = 6111,
    /// Get the Current Calibration Result
    CalibResult = 6112,
    /// Get the Current Calibration Result
    CalibAllinone2 = 6115,
    /// Motor Enabling and Disabling
    SetMotorEnable = 6201,
    /// Unbind designate goods
    ClearGoods = 6801,
    /// Unbind Goods from Designated Containers
    ClearContainer = 6802,
    /// Unbind Goods from all Containers
    ClearAllContainersGoods = 6803,
    /// Bind Goods to Containers
    SetContainerGoods = 6804,
    /// Update transparent data
    UpdateTransparentData = 6900,
    /// Storage Bin Detection
    BinDetect = 6901,
    /// Replay
    Replay = 6910,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum PushApi {
    /// Set the Robot Push Port
    Config = 9300,
    /// Robot Push
    Push = 19301,
}

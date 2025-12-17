use crate::{PointId, TaskId};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatusMessage {
    #[serde(rename = "ret_code")]
    pub code: StatusCode,
    #[serde(rename = "err_msg", default)]
    pub message: String,
    #[serde(rename = "create_on", default)]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::FromPrimitive)]
#[repr(u32)]
pub enum StatusCode {
    /// Success
    Success = 0,
    Unavailable = 40000,
    /// The request parameter is missing
    ParamMissing = 40001,
    /// The request parameter type is incorrect
    ParamTypeError = 40002,
    /// The request parameter is not legal
    ParamIllegal = 40003,
    /// Operating mode error
    ModeError = 40004,
    /// Illegal map name
    IllegalMapName = 40005,
    /// Programming firmware
    ProgrammingDsp = 40006,
    /// Programming firmware error
    ProgramDspError = 40007,
    /// An error occurred in the shutdown command
    ShutdownError = 40010,
    /// An error occurred in the restart command
    RebootError = 40011,
    /// Map analysis error
    MapParseError = 40050,
    /// The map does not exist
    MapNotExists = 40051,
    /// Loading map error
    LoadMapError = 40052,
    /// Overload map error
    LoadMapobjError = 40053,
    /// Open map
    EmptyMap = 40054,
    /// Request execution timeout
    ReqTimeout = 40100,
    /// Request is prohibited
    ReqForbidden = 40101,
    /// The robot is busy
    RobotBusy = 40102,
    /// Internal error
    RobotInternalError = 40199,
    /// Initialization status error
    InitStatusError = 41000,
    /// Map loading status error
    LoadmapStatusError = 41001,
    /// Relocation status error
    RelocStatusError = 41002,

    /// Unknown error code
    #[num_enum(default)]
    Custom,
}

/// Assumed that the enum is represented as u32
macro_rules! impl_serde_for_num_enum {
    ($enum_type:ty) => {
        impl<'de> serde::Deserialize<'de> for $enum_type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let code = u32::deserialize(deserializer)?;
                Ok(<$enum_type>::from(code))
            }
        }

        impl serde::Serialize for $enum_type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_u32(*self as u32)
            }
        }
    };
}

impl_serde_for_num_enum!(StatusCode);
impl_serde_for_num_enum!(JackOperation);

pub trait FromResponseBody: Sized {
    type Response: serde::de::DeserializeOwned;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommonInfo {
    pub id: String,
    pub version: String,
    pub model: String,
    #[serde(rename = "ret_code", default)]
    pub code: Option<StatusCode>,
    #[serde(rename = "err_msg", default)]
    pub message: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperationInfo {
    #[serde(rename = "odo")]
    pub mileage: f64,
    #[serde(rename = "total")]
    pub session_time_ms: f64,
    #[serde(rename = "total_time")]
    pub total_time_ms: f64,
    /// Controller temperature in Celsius
    pub controller_temp: f64,
    /// Controller humidity in percentage
    #[serde(default)]
    pub controller_humi: f64,
    /// Controller voltage in Volts
    #[serde(default)]
    pub controller_voltage: f64,

    #[serde(rename = "ret_code", default)]
    pub code: Option<StatusCode>,
    #[serde(rename = "err_msg", default)]
    pub message: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RobotPose {
    /// X coordinate in meters
    pub x: f64,
    /// Y coordinate in meters
    pub y: f64,
    /// Angle in radians
    pub angle: f64,

    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,

    #[serde(rename = "ret_code", default)]
    pub code: Option<StatusCode>,
    #[serde(rename = "err_msg", default)]
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::FromPrimitive)]
#[repr(u8)]
pub enum BlockReason {
    Laser = 1,
    Fallingdown = 2,
    Collision = 3,
    Infrared = 4,

    #[num_enum(default)]
    Custom,
}

impl<'de> serde::Deserialize<'de> for BlockReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let code = u8::deserialize(deserializer)?;
        Ok(BlockReason::from(code))
    }
}

impl serde::Serialize for BlockReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BlockStatus {
    #[serde(rename = "blocked")]
    pub is_blocked: bool,
    #[serde(rename = "block_reason", default)]
    pub reason: Option<BlockReason>,
    #[serde(rename = "block_x", default)]
    pub x: Option<f64>,
    #[serde(rename = "block_y", default)]
    pub y: Option<f64>,

    #[serde(rename = "ret_code", default)]
    pub code: Option<StatusCode>,
    #[serde(rename = "err_msg", default)]
    pub message: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BatteryStatus {
    /// Level in range 0.0 to 1.0
    pub battery_level: f64,
    /// Temperature in Celsius
    pub battery_temp: f64,
    /// Is the robot currently charging
    pub charging: bool,
    /// Voltage in Volts
    pub voltage: f64,
    /// Current in Amperes
    pub current: f64,

    #[serde(rename = "ret_code", default)]
    pub code: Option<StatusCode>,
    #[serde(rename = "err_msg", default)]
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::FromPrimitive)]
#[repr(u32)]
pub enum JackOperation {
    Rising = 0x0,
    RisingInPlace = 0x1,
    Lowering = 0x2,
    LoweringInPlace = 0x3,
    Stop = 0x4,
    #[num_enum(default)]
    Failed = 0xFF,
}

/// Status of the robot's jack
/// ### Example
/// ```
/// use seersdk_rs::{JackStatus, JackOperation};
/// let raw_json = r#"
///  {
///   "jack_emc": false,
///   "jack_enable": false,
///   "jack_error_code": 0,
///   "jack_height": 0,
///   "jack_isFull": false,
///   "jack_mode": false,
///   "jack_speed": 0,
///   "jack_state": 0,
///   "peripheral_data": [],
///   "ret_code": 0
/// }"#;
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JackStatus {
    /// Current mode is automatic or manual
    #[serde(rename = "jack_mode")]
    pub automatic_mode: bool,

    #[serde(rename = "jack_enable")]
    pub enabled: bool,

    #[serde(rename = "jack_error_code")]
    pub error_code: u32,

    /// Current jack operation
    #[serde(rename = "jack_state")]
    pub operation: JackOperation,

    #[serde(rename = "jack_isFull")]
    pub has_payload: bool,
    /// Jacking speed in mm/s
    #[serde(rename = "jack_speed")]
    pub speed: u32,
    /// Is emergency stop activated
    #[serde(rename = "jack_emc")]
    pub emergency_stop: bool,
    /// Current height in meters
    #[serde(rename = "jack_height")]
    pub height: f64,
    /// User defined peripheral data
    #[serde(rename = "peripheral_data")]
    pub peripheral_data: Vec<u8>,

    #[serde(rename = "ret_code", default)]
    pub code: Option<StatusCode>,
    #[serde(rename = "err_msg", default)]
    pub message: String,

    /// API Upload timestamp
    #[serde(rename = "create_on", default)]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NavStatus {
    pub state: TaskState,
    #[serde(rename = "task_type")]
    pub ty: TaskType,
    pub target_id: PointId,
    /// Target point coordinates (x, y, angle)
    pub target_point: [f64; 3],

    /// Stations already passed on the current navigation path,
    /// an array of stations, this field is only valid when task_type is 3.
    /// All intermediate points already passed will be listed here
    pub finished_path: Vec<PointId>,

    /// Stations on the current navigation path that have not yet been passed,
    /// represented as an array of stations, are only valid when task_type is 3.
    /// All intermediate points that have not yet been passed will be listed here.
    pub unfinished_path: Vec<PointId>,

    /// Navigation Task Additional Information
    pub move_status_info: String,

    /// API Error Code
    pub code: Option<StatusCode>,
    /// API Upload Timestamp
    pub create_on: Option<String>,
    /// Error Message
    #[serde(rename = "err_msg", default)]
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::FromPrimitive)]
#[repr(u32)]
pub enum TaskType {
    NoNav = 0,
    FreeNavToPoint = 1,
    FreeNavToSite = 2,
    PathNavToSite = 3,
    Manual = 7,
    #[num_enum(default)]
    Other = 100,
}

impl_serde_for_num_enum!(TaskType);

#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::FromPrimitive)]
#[repr(u32)]
pub enum TaskState {
    #[num_enum(default)]
    None = 0,
    Waiting = 1,
    Running = 2,
    Suspended = 3,
    Completed = 4,
    Failed = 5,
    Canceled = 6,
    OverTime = 7,
    NotFound = 404,
}

impl_serde_for_num_enum!(TaskState);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskStateItem {
    pub task_id: TaskId,
    pub state: TaskState,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskStatus {
    /// The station closest to the robot within a certain linear distance (this distance is a
    pub closest_target: PointId,
    /// The "source_id" in the navigation task currently being executed by the robot
    pub source_name: TaskId,
    /// The "id" of the navigation task currently being executed by the robot
    pub target_name: TaskId,
    /// In the navigation task currently being executed by the robot, for the corresponding path,
    /// the proportion of the part that the robot has completed to the entire path
    pub percentage: f64,
    /// Projection distance of the robot to the "path corresponding to the currently executing
    /// navigation task
    pub distance: f64,

    #[serde(rename = "task_status_list")]
    pub tasks: Vec<TaskStateItem>,
    /// During the navigation process, some prompts from the robot to the user can be output to the
    /// front end. This field does not participate in actual logical judgment
    pub info: String,

    #[serde(rename = "ret_code", default)]
    pub code: Option<StatusCode>,
    #[serde(rename = "err_msg", default)]
    pub message: String,
    pub create_on: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::StatusCode;

    #[test]
    fn test_error_code_serialization() {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        struct TestStruct {
            code: StatusCode,
        }

        let test_instance = TestStruct {
            code: StatusCode::ParamMissing,
        };

        let serialized = serde_json::to_string(&test_instance).unwrap();
        assert_eq!(serialized, r#"{"code":40001}"#);

        let deserialized: TestStruct =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.code, StatusCode::ParamMissing);

        let custom_code = r#"{"code":99999}"#;
        let deserialized_custom: TestStruct =
            serde_json::from_str(custom_code).unwrap();
        assert_eq!(deserialized_custom.code, StatusCode::Custom);
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_robot_pose_serialization_deserializatio() {
        let with_error_code = r#"
        {
            "x": 1.0,
            "y": 2.0,
            "angle": 0.7854,
            "confidence": 0.95,
            "ret_code": 40000,
            "err_msg": "msg"
        }"#;

        let pose: super::RobotPose =
            serde_json::from_str(with_error_code).unwrap();
        assert_eq!(pose.x, 1.0);
        assert_eq!(pose.y, 2.0);
        assert!((pose.angle - 0.7854).abs() < 0.0001);
        assert_eq!(pose.confidence, 0.95);
        assert_eq!(pose.code, Some(StatusCode::Unavailable));
        assert_eq!(pose.message, "msg");

        let without_error_code = r#"
        {
            "x": 3.0,
            "y": 4.0,
            "angle": 1.5708,
            "confidence": 0.9
        }"#;
        let pose_no_code: super::RobotPose =
            serde_json::from_str(without_error_code).unwrap();
        assert_eq!(pose_no_code.x, 3.0);
        assert_eq!(pose_no_code.y, 4.0);
        assert!((pose_no_code.angle - 1.5708).abs() < 0.0001);
        assert_eq!(pose_no_code.confidence, 0.9);
        assert_eq!(pose_no_code.code, None);
        assert_eq!(pose_no_code.message, "");
    }

    #[test]
    fn test_block_status_serialization_deserialization() {
        let with_error_code = r#"
        {
            "blocked": true,
            "block_reason": 2,
            "block_x": 1.5,
            "block_y": 2.5,
            "ret_code": 40002,
            "err_msg": "Parameter type error"
        }"#;

        let status: super::BlockStatus =
            serde_json::from_str(with_error_code).unwrap();
        assert!(status.is_blocked);
        assert_eq!(status.reason, Some(super::BlockReason::Fallingdown));
        assert_eq!(status.x, Some(1.5));
        assert_eq!(status.y, Some(2.5));
        assert_eq!(status.code, Some(StatusCode::ParamTypeError));
        assert_eq!(status.message, "Parameter type error");

        let without_error_code = r#"
        {
            "blocked": false
        }"#;
        let status_no_code: super::BlockStatus =
            serde_json::from_str(without_error_code).unwrap();
        assert!(!status_no_code.is_blocked);
        assert_eq!(status_no_code.reason, None);
        assert_eq!(status_no_code.x, None);
        assert_eq!(status_no_code.y, None);
        assert_eq!(status_no_code.code, None);
        assert_eq!(status_no_code.message, "");
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatusMessage {
    #[serde(rename = "ret_code")]
    pub code: ErrorCode,
    #[serde(rename = "err_msg", default)]
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::FromPrimitive)]
#[repr(u32)]
pub enum ErrorCode {
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

impl<'de> serde::Deserialize<'de> for ErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let code = u32::deserialize(deserializer)?;
        Ok(ErrorCode::from(code))
    }
}

impl serde::Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(*self as u32)
    }
}

pub trait FromResponseBody: Sized {
    type Response: serde::de::DeserializeOwned;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommonInfo {
    pub id: String,
    pub version: String,
    pub model: String,
    #[serde(rename = "ret_code", default)]
    pub code: Option<ErrorCode>,
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
    pub code: Option<ErrorCode>,
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
    pub code: Option<ErrorCode>,
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
    pub code: Option<ErrorCode>,
    #[serde(rename = "err_msg", default)]
    pub message: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RobotBattery {
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
    pub code: Option<ErrorCode>,
    #[serde(rename = "err_msg", default)]
    pub message: String,
}

#[cfg(test)]
mod tests {
    use crate::ErrorCode;

    #[test]
    fn test_error_code_serialization() {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        struct TestStruct {
            code: ErrorCode,
        }

        let test_instance = TestStruct {
            code: ErrorCode::ParamMissing,
        };

        let serialized = serde_json::to_string(&test_instance).unwrap();
        assert_eq!(serialized, r#"{"code":40001}"#);

        let deserialized: TestStruct =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.code, ErrorCode::ParamMissing);

        let custom_code = r#"{"code":99999}"#;
        let deserialized_custom: TestStruct =
            serde_json::from_str(custom_code).unwrap();
        assert_eq!(deserialized_custom.code, ErrorCode::Custom);
    }

    #[test]
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
        assert_eq!(pose.angle, 0.7854);
        assert_eq!(pose.confidence, 0.95);
        assert_eq!(pose.code, Some(ErrorCode::Unavailable));
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
        assert_eq!(pose_no_code.angle, 1.5708);
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
        assert_eq!(status.is_blocked, true);
        assert_eq!(status.reason, Some(super::BlockReason::Fallingdown));
        assert_eq!(status.x, Some(1.5));
        assert_eq!(status.y, Some(2.5));
        assert_eq!(status.code, Some(ErrorCode::ParamTypeError));
        assert_eq!(status.message, "Parameter type error");

        let without_error_code = r#"
        {
            "blocked": false
        }"#;
        let status_no_code: super::BlockStatus =
            serde_json::from_str(without_error_code).unwrap();
        assert_eq!(status_no_code.is_blocked, false);
        assert_eq!(status_no_code.reason, None);
        assert_eq!(status_no_code.x, None);
        assert_eq!(status_no_code.y, None);
        assert_eq!(status_no_code.code, None);
        assert_eq!(status_no_code.message, "");
    }
}

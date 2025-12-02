#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatusMessage {
    #[serde(
        rename = "ret_code",
        serialize_with = "error_code_serialize",
        deserialize_with = "error_code_deserialize",
    )]
    pub code: ErrorCode,
    #[serde(rename = "err_msg", default)]
    pub message: String,
}

fn error_code_serialize<S>(
    code: &ErrorCode,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u32(*code as u32)
}

fn error_code_deserialize<'de, D>(
    deserializer: D,
) -> Result<ErrorCode, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserialize;

    let code = u32::deserialize(deserializer)?;

    ErrorCode::try_from(code).map_err(serde::de::Error::custom)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, num_enum::TryFromPrimitive)]
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
}

pub trait 

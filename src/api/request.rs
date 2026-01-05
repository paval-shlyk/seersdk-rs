use crate::{ApiRequest, PointId, TaskId};

pub trait ToRequestBody {
    /// Convert the request to a JSON string body
    fn to_request_body(&self) -> Result<String, serde_json::Error>;
    fn to_api_request(&self) -> ApiRequest;
}

pub const SELF_POSITION: &str = "SELF_POSITION";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct MoveToPoint {
    pub id: PointId,
    pub x: Option<f64>,
    pub y: Option<f64>,

    pub angle: Option<f64>,
    pub max_speed: Option<f64>,
    pub max_wspeed: Option<f64>,
    pub max_acc: Option<f64>,
    pub max_wacc: Option<f64>,
}

impl MoveToPoint {
    /// Move to the origin (0,0)
    pub fn zeros() -> Self {
        Self::default()
    }

    pub fn with_id<T: Into<String>>(id: T) -> Self {
        Self {
            id: id.into(),
            ..Default::default()
        }
    }

    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: Some(x),
            y: Some(y),
            ..Default::default()
        }
    }
}

//id
//
//string
//
//Target Site Name
//When the robot executes an operation in place, its fixed value is "SELF_POSITION"
//source_id
//
//string
//
//Starting Station Name
//When the starting position is not at a station but the current position of the robot, its value is fixed as "SELF_POSITION"
//task_id
//string
//Task Number
//angle
//number
//Angle value of the target site (world coordinate system), unit: rad
//method
//
//string
//
//The movement mode can only be "forward" (walking forward) or "backward" (walking backward), and if omitted, the mode configured on the path will be used
//max_speed
//number
//Maximum speed, unit m/s
//max_wspeed
//number
//Maximum angular velocity, unit rad/s
//max_acc
//number
//Maximum acceleration, unit m/s^2
//max_wacc
//number
//Maximum angular acceleration, unit rad/s^2
//duration
//number
//Waiting time of the robot after navigation ends, unit: ms
//orientation
//number
//Angle maintained by the omnidirectional vehicle (not currently in use)
//spin
//boolean
//Follow-up or not
//delay
//number
//Delays the time to end the navigation state, with the unit in ms, defaulting to 0
//start_rot_dir
//(>=3.4.6.1802)
//number
//
//Starting in-place rotation direction-1: Clockwise rotation
//0: Rotate towards the nearest direction
//1: Counterclockwise rotation
//end_rot_dir
//
//(>=3.4.6.1802)
//number
//Rotation direction at the point
//-1: Clockwise rotation
//0: Rotate towards the nearest direction
//1: Counterclockwise rotation
//reach_dist
//number
//Position Accuracy (m)
//reach_angle
//number
//Angle accAngle Accuracy (rad)uracy to point, unit: rad
//skill_name
//string
//"Action" indicates performing an action at the current point, or "GotoSpecifiedPose" represents path navigation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MoveMethod {
    Forward,
    Backward,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetNavStatus {
    /// Whether to return only simple data, true = yes, false = no, default is no
    pub simple: Option<bool>,
}

impl GetNavStatus {
    pub fn new() -> Self {
        Self { simple: None }
    }

    pub fn with_simple(mut self, simple: bool) -> Self {
        self.simple = Some(simple);
        self
    }
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq,
)]
pub struct MoveToTarget {
    /// Target Station Name
    /// When the robot executes an operation in place, its fixed value is "SELF_POSITION"
    #[serde(rename = "id")]
    pub target: PointId,
    /// Starting Station Name
    /// When the starting position is not at a station but the current position of the robot, its
    /// value is fixed as "SELF_POSITION"
    #[serde(rename = "source_id")]
    pub start: PointId,

    /// Unique Task ID
    /// Can be ommitted when only a single MoveToTarget is used
    /// But required when used in a list of MoveToTargets
    pub task_id: Option<TaskId>,

    /// When ommitted, the mode configured on the path will be used
    pub method: Option<MoveMethod>,

    /// Follow-up or not
    pub spin: Option<bool>,

    /// Delays the time to end the navigation state, with the unit in ms, defaulting to 0
    #[serde(default)]
    pub delay: u64,

    /// Starting in-place rotation direction
    /// -1: Clockwise rotation
    /// 0: Rotate towards the nearest direction
    /// 1: Counterclockwise rotation
    pub start_rot_dir: Option<i8>,

    /// Rotation direction at the point
    /// -1: Clockwise rotation
    /// 0: Rotate towards the nearest direction
    /// 1: Counterclockwise rotation
    pub end_rot_dir: Option<i8>,

    /// Position Accuracy (m)
    pub reach_dist: Option<f64>,

    /// Angle Accuracy (rad)
    pub reach_angle: Option<f64>,

    pub angle: Option<f64>,
    pub max_speed: Option<f64>,
    pub max_wspeed: Option<f64>,
    pub max_acc: Option<f64>,
    pub max_wacc: Option<f64>,

    #[serde(flatten)]
    pub jack_operation: Option<JackOperation>,
}

/// Macro to implement builder methods for MoveToTarget
/// 3 argumens: method_name, field_name, field_type
macro_rules! impl_move_to_target_builder {
    ( $($method_name:ident : $field_name:ident = $field_type:ty),* $(,)? ) => {
        $(
            pub fn $method_name(mut self, value: $field_type) -> Self {
                self.$field_name = value.into();
                self
            }
        )*
    };
}

impl MoveToTarget {
    pub fn new<T: Into<String>>(target: T) -> Self {
        Self {
            target: target.into(),
            start: SELF_POSITION.to_string(),
            ..Default::default()
        }
    }

    impl_move_to_target_builder! {
        with_task_id: task_id = TaskId,
        with_start: start = PointId,
        with_method: method = MoveMethod,
        with_operation: jack_operation = JackOperation,
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(tag = "operation")]
pub enum JackOperation {
    JackLoad,
    JackUnload,
    JackHeight { jack_height: f64 },
    Wait,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetJackHeight {
    pub height: f64,
}

impl SetJackHeight {
    pub fn new(height: f64) -> Self {
        Self { height }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MoveDesignedPath {
    #[serde(rename = "move_task_list")]
    pub path: Vec<MoveToTarget>,
}

impl MoveDesignedPath {
    pub fn new(path: impl IntoIterator<Item = MoveToTarget>) -> Self {
        Self {
            path: path.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetTaskStatus {
    ///Specify the task_id of the task to be queried in the array.
    ///If the array is empty, the response will also be empty;
    ///If this field is omitted, the status of the most recently completed task and the status of all incomplete tasks of the robot will be returned.
    pub task_ids: Vec<String>,
}

impl FromIterator<String> for GetTaskStatus {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            task_ids: iter.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move_to_point_serialization() {
        let raw_json = r#"
            {
                  "id": "AP1",
                  "source_id": "LM2",
                  "task_id": "12344321",
                  "operation": "JackHeight",
                  "jack_height": 0.2
            }
        "#;

        let m1: MoveToTarget = serde_json::from_str(raw_json).unwrap();

        let m2 = MoveToTarget {
            target: "AP1".to_string(),
            start: "LM2".to_string(),
            task_id: Some("12344321".to_string()),
            jack_operation: Some(JackOperation::JackHeight {
                jack_height: 0.2,
            }),
            ..Default::default()
        };

        let serialized = serde_json::to_string_pretty(&m2).unwrap();
        let m2 = serde_json::from_str::<MoveToTarget>(&serialized).unwrap();

        eprintln!("Serialized: {}", serialized);
        assert_eq!(m1, m2);

        let raw_json = r#"
            {
                  "id": "AP1",
                  "source_id": "LM2",
                  "task_id": "12344321",
                  "operation": "JackLoad"
            }
        "#;
        let m1: MoveToTarget = serde_json::from_str(raw_json).unwrap();
        let m2 = MoveToTarget {
            target: "AP1".to_string(),
            start: "LM2".to_string(),
            task_id: Some("12344321".to_string()),
            jack_operation: Some(JackOperation::JackLoad),
            ..Default::default()
        };
        let serialized = serde_json::to_string_pretty(&m2).unwrap();
        let m2 = serde_json::from_str::<MoveToTarget>(&serialized).unwrap();
        assert_eq!(m1, m2);
    }
}

use crate::ApiRequest;

pub trait ToRequestBody {
    /// Convert the request to a JSON string body
    fn to_request_body(&self) -> Result<String, serde_json::Error>;
    fn to_api_request(&self) -> ApiRequest;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct MoveToPoint {
    pub id: String,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct MoveToTarget {
    pub id: String,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub angle: Option<f64>,
    pub max_speed: Option<f64>,
    pub max_wspeed: Option<f64>,
    pub max_acc: Option<f64>,
    pub max_wacc: Option<f64>,
}

impl MoveToTarget {
    pub fn new<T: Into<String>>(id: T) -> Self {
        Self {
            id: id.into(),
            ..Default::default()
        }
    }
}

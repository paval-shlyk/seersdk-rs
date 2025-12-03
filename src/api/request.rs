use crate::ApiRequest;

pub trait ToRequestBody {
    /// Convert the request to a JSON string body
    fn to_request_body(&self) -> Result<String, serde_json::Error>;
    fn to_api_request(&self) -> ApiRequest;
}

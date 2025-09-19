use serde_json::Value;

pub struct HttpResponseDto {
    pub status_code: u16,
    pub body: Option<Value>,
}

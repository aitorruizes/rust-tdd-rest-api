use serde_json::Value;

pub struct HttpRequestDto {
    pub body: Option<Value>,
    pub method: String,
    pub url: String,
}

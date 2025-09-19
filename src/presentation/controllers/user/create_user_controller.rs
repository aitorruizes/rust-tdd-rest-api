use std::pin::Pin;

use crate::presentation::{
    dtos::http::{http_request_dto::HttpRequestDto, http_response_dto::HttpResponseDto},
    ports::controller::controller_port::ControllerPort,
};

#[derive(Clone)]
pub struct CreateUserController;

impl CreateUserController {
    pub fn new() -> Self {
        CreateUserController {}
    }
}

impl ControllerPort for CreateUserController {
    fn handle(
        &self,
        _http_request_dto: HttpRequestDto,
    ) -> Pin<Box<dyn Future<Output = HttpResponseDto> + Send>> {
        Box::pin(async move {
            HttpResponseDto {
                status_code: 201,
                body: Some(serde_json::json!({ "message": "OK!" })),
            }
        })
    }
}

impl Default for CreateUserController {
    fn default() -> Self {
        Self::new()
    }
}

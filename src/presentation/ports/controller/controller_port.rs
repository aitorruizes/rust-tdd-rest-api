use std::pin::Pin;

use crate::presentation::dtos::http::{
    http_request_dto::HttpRequestDto, http_response_dto::HttpResponseDto,
};

pub trait ControllerPort: Send + Sync {
    fn handle(
        &self,
        request: HttpRequestDto,
    ) -> Pin<Box<dyn Future<Output = HttpResponseDto> + Send + '_>>;
}

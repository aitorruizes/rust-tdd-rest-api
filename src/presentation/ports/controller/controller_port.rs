use std::pin::Pin;

use crate::presentation::dtos::http::{
    http_request_dto::HttpRequestDto, http_response_dto::HttpResponseDto,
};

pub type ControllerFuture<'a> = Pin<Box<dyn Future<Output = HttpResponseDto> + Send + 'a>>;

pub trait ControllerPort: Send + Sync {
    fn handle(&self, request: HttpRequestDto) -> ControllerFuture<'_>;
}

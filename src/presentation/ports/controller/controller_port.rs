use std::pin::Pin;

use crate::presentation::dtos::http::{
    http_request_dto::HttpRequestDto, http_response_dto::HttpResponseDto,
};

pub trait ControllerPort: ControllerPortClone + Send + Sync {
    fn handle(
        &self,
        request: HttpRequestDto,
    ) -> Pin<Box<dyn Future<Output = HttpResponseDto> + Send + '_>>;
}

pub trait ControllerPortClone {
    fn clone_box(&self) -> Box<dyn ControllerPort + Send + Sync>;
}

impl<T> ControllerPortClone for T
where
    T: ControllerPort + Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn ControllerPort + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ControllerPort + Send + Sync> {
    fn clone(&self) -> Box<dyn ControllerPort + Send + Sync> {
        self.as_ref().clone_box()
    }
}

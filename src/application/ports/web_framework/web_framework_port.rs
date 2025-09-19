use std::{any::Any, pin::Pin};

use crate::presentation::ports::controller::controller_port::ControllerPort;

pub type ServeFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + 'a>>;

pub enum RouterMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

pub trait RouterWrapper: Send + Sync {
    fn into_inner(self: Box<Self>) -> Box<dyn Any + Send + Sync>;
}

pub trait WebFrameworkPort {
    fn serve(&self) -> ServeFuture<'_>;
}

pub trait WebFrameworkRoutePort {
    fn create_router(
        &self,
        router_method: RouterMethod,
        path: &str,
        controller: Box<dyn ControllerPort>,
    ) -> Box<dyn RouterWrapper>;
    fn merge_router(
        &self,
        router: Box<dyn RouterWrapper>,
        router_to_merge: Box<dyn RouterWrapper>,
    ) -> Box<dyn RouterWrapper>;
    fn create_core_router(
        &self,
        path: &str,
        router_to_merge: Box<dyn RouterWrapper>,
    ) -> Box<dyn RouterWrapper>;
}

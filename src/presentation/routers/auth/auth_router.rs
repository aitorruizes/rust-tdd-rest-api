use std::collections::HashMap;

use axum::{
    Router,
    body::Body,
    extract::{Path, Request},
    routing::post,
};

use crate::{
    infrastructure::adapters::axum::axum_handler_adapter::AxumHandlerAdapter,
    presentation::ports::{
        controller::controller_port::ControllerPort, router::router_port::RouterPort,
    },
};

pub struct AuthRouter<SignUpController, SignInController> {
    sign_up_controller: SignUpController,
    sign_in_controller: SignInController,
}

impl<SignUpController, SignInController> AuthRouter<SignUpController, SignInController>
where
    SignUpController: ControllerPort + Clone + Send + Sync,
    SignInController: ControllerPort + Clone + Send + Sync,
{
    #[must_use]
    pub const fn new(
        sign_up_controller: SignUpController,
        sign_in_controller: SignInController,
    ) -> Self {
        Self {
            sign_up_controller,
            sign_in_controller,
        }
    }
}

impl<SignUpController, SignInController> RouterPort
    for AuthRouter<SignUpController, SignInController>
where
    SignUpController: ControllerPort + Clone + Send + Sync + 'static,
    SignInController: ControllerPort + Clone + Send + Sync + 'static,
{
    fn register_routes(self) -> Router {
        let sign_up_controller_adapter = AxumHandlerAdapter::new(self.sign_up_controller);
        let sign_in_controller_adapter = AxumHandlerAdapter::new(self.sign_in_controller);

        Router::new()
            .route(
                "/auth/sign-up",
                post({
                    move |path: Path<HashMap<String, String>>, req: Request<Body>| async move {
                        sign_up_controller_adapter.adapt_handler(path, req).await
                    }
                }),
            )
            .route(
                "/auth/sign-in",
                post({
                    move |path: Path<HashMap<String, String>>, req: Request<Body>| async move {
                        sign_in_controller_adapter.adapt_handler(path, req).await
                    }
                }),
            )
    }
}

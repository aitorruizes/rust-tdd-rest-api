use std::collections::HashMap;

use axum::{
    Router,
    body::Body,
    extract::{Path, Request},
    routing::{get, post},
};

use crate::{
    infrastructure::adapters::axum::axum_adapter::AxumAdapter,
    presentation::{
        controllers::auth::{
            sign_in_controller::SignInController, sign_up_controller::SignUpController,
        },
        ports::router::router_port::RouterPort,
    },
};

pub struct AuthRouter {
    sign_up_controller: SignUpController,
    sign_in_controller: SignInController,
}

impl AuthRouter {
    pub fn new(sign_up_controller: SignUpController, sign_in_controller: SignInController) -> Self {
        AuthRouter {
            sign_up_controller,
            sign_in_controller,
        }
    }
}

impl RouterPort for AuthRouter {
    fn register_routes(self) -> Router {
        let sign_up_controller_adapter: AxumAdapter =
            AxumAdapter::new(Box::new(self.sign_up_controller));
        let sign_in_controller_adapter: AxumAdapter =
            AxumAdapter::new(Box::new(self.sign_in_controller));

        Router::new()
            .route(
                "/auth/sign-up",
                post({
                    move |path: Path<HashMap<String, String>>, req: Request<Body>| async move {
                        sign_up_controller_adapter.adapt_controller(path, req).await
                    }
                }),
            )
            .route(
                "/auth/sign-in/{email}",
                get({
                    move |path: Path<HashMap<String, String>>, req: Request<Body>| async move {
                        sign_in_controller_adapter.adapt_controller(path, req).await
                    }
                }),
            )
    }
}

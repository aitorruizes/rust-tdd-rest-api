use std::collections::HashMap;

use axum::{
    Router,
    body::Body,
    extract::{Path, Request},
    routing::post,
};

use crate::{
    infrastructure::adapters::axum::axum_adapter::AxumAdapter,
    presentation::{
        controllers::auth::sign_up_controller::SignUpController,
        ports::router::router_port::RouterPort,
    },
};

pub struct AuthRouter {
    sign_up_controller: SignUpController,
}

impl AuthRouter {
    pub fn new(sign_up_controller: SignUpController) -> Self {
        AuthRouter { sign_up_controller }
    }
}

impl RouterPort for AuthRouter {
    fn register_routes(self) -> Router {
        let axum_adapter: AxumAdapter = AxumAdapter::new(Box::new(self.sign_up_controller));

        Router::new().route(
            "/auth/sign-up",
            post(
                move |path: Path<HashMap<String, String>>, req: Request<Body>| async move {
                    axum_adapter.adapt_controller(path, req).await
                },
            ),
        )
    }
}

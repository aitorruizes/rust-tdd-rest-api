use axum::{Json, Router, http::StatusCode};
use serde_json::json;
use tower_http::trace::TraceLayer;

use crate::presentation::{
    controllers::auth::{
        sign_in_controller::SignInController, sign_up_controller::SignUpController,
    },
    ports::router::router_port::RouterPort,
    routers::auth::auth_router::AuthRouter,
};

pub struct CoreRouter {
    sign_up_controller: SignUpController,
    sign_in_controller: SignInController,
}

impl CoreRouter {
    pub fn new(sign_up_controller: SignUpController, sign_in_controller: SignInController) -> Self {
        CoreRouter {
            sign_up_controller,
            sign_in_controller,
        }
    }
}

impl RouterPort for CoreRouter {
    fn register_routes(self) -> Router {
        let auth_router: AuthRouter =
            AuthRouter::new(self.sign_up_controller, self.sign_in_controller);

        Router::new()
            .nest("/api/v1", auth_router.register_routes())
            .layer(TraceLayer::new_for_http())
            .fallback(|| async {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "router not found" })),
                )
            })
    }
}

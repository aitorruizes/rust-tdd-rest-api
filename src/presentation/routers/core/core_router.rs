use axum::{Json, Router, http::StatusCode, response::IntoResponse};
use serde_json::json;
use tower::ServiceBuilder;
use tower_governor::{GovernorError, GovernorLayer, governor::GovernorConfigBuilder};
use tower_helmet::HelmetLayer;
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

        let trace_layer_middleware = TraceLayer::new_for_http();

        let governor_config = GovernorConfigBuilder::default()
            .per_second(4)
            .burst_size(2)
            .finish()
            .unwrap();

        let governor_middleware =
            GovernorLayer::new(governor_config).error_handler(|err: GovernorError| match err {
                GovernorError::TooManyRequests { .. } => (
                    StatusCode::TOO_MANY_REQUESTS,
                    Json(json!({
                        "error_code": "too_many_requests",
                        "error_message": "rate limit exceeded"
                    })),
                )
                    .into_response(),

                GovernorError::UnableToExtractKey => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error_code": "unable_to_extract_key",
                        "error_message": "a key extractor must be provided"
                    })),
                )
                    .into_response(),
                GovernorError::Other { msg, .. } => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error_code": "internal_server_error",
                        "error_message": msg
                    })),
                )
                    .into_response(),
            });

        let helmet_middleware: HelmetLayer = HelmetLayer::with_defaults();

        Router::new()
            .nest("/api/v1", auth_router.register_routes())
            .layer(
                ServiceBuilder::new()
                    .layer(trace_layer_middleware)
                    .layer(governor_middleware)
                    .layer(helmet_middleware),
            )
            .fallback(|| async {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "router not found" })),
                )
            })
    }
}

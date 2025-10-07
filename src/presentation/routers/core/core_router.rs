use axum::{Json, Router, http::StatusCode, response::IntoResponse};
use serde_json::json;
use tower::ServiceBuilder;
use tower_governor::{GovernorError, GovernorLayer, governor::GovernorConfigBuilder};
use tower_helmet::HelmetLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::presentation::{
    ports::{controller::controller_port::ControllerPort, router::router_port::RouterPort},
    routers::{auth::auth_router::AuthRouter, user::user_router::UserRouter},
};

#[allow(clippy::struct_field_names)]
pub struct CoreRouter<SignUpController, SignInController, GetUserByIdController> {
    sign_up_controller: SignUpController,
    sign_in_controller: SignInController,
    get_user_by_id_controller: GetUserByIdController,
}

impl<SignUpController, SignInController, GetUserByIdController>
    CoreRouter<SignUpController, SignInController, GetUserByIdController>
where
    SignUpController: ControllerPort + Clone + Send + Sync,
    SignInController: ControllerPort + Clone + Send + Sync,
    GetUserByIdController: ControllerPort + Clone + Send + Sync,
{
    #[must_use]
    pub const fn new(
        sign_up_controller: SignUpController,
        sign_in_controller: SignInController,
        get_user_by_id_controller: GetUserByIdController,
    ) -> Self {
        Self {
            sign_up_controller,
            sign_in_controller,
            get_user_by_id_controller,
        }
    }
}

impl<SignUpController, SignInController, GetUserByIdController> RouterPort
    for CoreRouter<SignUpController, SignInController, GetUserByIdController>
where
    SignUpController: ControllerPort + Clone + Send + Sync + 'static,
    SignInController: ControllerPort + Clone + Send + Sync + 'static,
    GetUserByIdController: ControllerPort + Clone + Send + Sync + 'static,
{
    fn register_routes(self) -> Router {
        let auth_router =
            AuthRouter::new(self.sign_up_controller, self.sign_in_controller).register_routes();

        let user_router = UserRouter::new(self.get_user_by_id_controller).register_routes();
        let cors_middleware = CorsLayer::permissive();
        let trace_layer_middleware = TraceLayer::new_for_http();

        let governor_config = GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
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

        let helmet_middleware = HelmetLayer::with_defaults();
        let merged_routers = auth_router.merge(user_router);

        Router::new()
            .nest("/api/v1", merged_routers)
            .layer(
                ServiceBuilder::new()
                    .layer(cors_middleware)
                    .layer(governor_middleware)
                    .layer(helmet_middleware),
            )
            .layer(trace_layer_middleware)
            .fallback(|| async {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "router not found" })),
                )
            })
    }
}

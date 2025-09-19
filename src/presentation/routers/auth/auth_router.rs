use axum::{Router, http::Method};

use crate::{
    infrastructure::adapters::axum::axum_route_adapter::AxumRouteAdapter,
    presentation::{
        controllers::auth::sign_up_controller::SignUpController,
        ports::router::router_port::RouterPort,
    },
};

pub struct AuthRouter {
    axum_route_adapter: AxumRouteAdapter,
    sign_up_controller: SignUpController,
}

impl AuthRouter {
    pub fn new(axum_route_adapter: AxumRouteAdapter, sign_up_controller: SignUpController) -> Self {
        AuthRouter {
            axum_route_adapter,
            sign_up_controller,
        }
    }
}

impl RouterPort for AuthRouter {
    fn register_routes(self) -> Router {
        self.axum_route_adapter.create_router(
            Method::POST,
            "/auth/sign-up",
            Box::new(self.sign_up_controller),
        )
    }
}

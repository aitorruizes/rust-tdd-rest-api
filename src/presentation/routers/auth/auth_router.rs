use axum::{Router, http::Method};

use crate::{
    infrastructure::adapters::axum::axum_route_adapter::AxumRouteAdapter,
    presentation::{
        controllers::user::create_user_controller::CreateUserController,
        ports::router::router_port::RouterPort,
    },
};

pub struct AuthRouter {
    axum_route_adapter: AxumRouteAdapter,
    create_user_controller: CreateUserController,
}

impl AuthRouter {
    pub fn new(
        axum_route_adapter: AxumRouteAdapter,
        create_user_controller: CreateUserController,
    ) -> Self {
        AuthRouter {
            axum_route_adapter,
            create_user_controller,
        }
    }
}

impl RouterPort for AuthRouter {
    fn register_routes(self) -> Router {
        self.axum_route_adapter.create_router(
            Method::POST,
            "/auth/sign-up",
            Box::new(self.create_user_controller),
        )
    }
}

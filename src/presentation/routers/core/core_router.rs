use axum::{Router, http::Method};

use crate::{
    infrastructure::adapters::axum::axum_route_adapter::AxumRouteAdapter,
    presentation::{
        controllers::user::create_user_controller::CreateUserController,
        ports::router::router_port::RouterPort,
    },
};

pub struct CoreRouter {
    axum_route_adapter: AxumRouteAdapter,
    create_user_controller: CreateUserController,
}

impl CoreRouter {
    pub fn new(
        axum_route_adapter: AxumRouteAdapter,
        create_user_controller: CreateUserController,
    ) -> Self {
        CoreRouter {
            axum_route_adapter,
            create_user_controller,
        }
    }
}

impl RouterPort for CoreRouter {
    fn register_routes(self) -> Router {
        let auth_router: Router = self.axum_route_adapter.create_router(
            Method::POST,
            "/auth/sign-up",
            Box::new(self.create_user_controller),
        );

        self.axum_route_adapter
            .create_core_router("/api/v1", auth_router)
    }
}

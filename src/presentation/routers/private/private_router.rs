use axum::{Router, middleware, routing::get};

use crate::{
    infrastructure::adapters::jsonwebtoken::jsonwebtoken_adapter::JsonWebTokenAdapter,
    presentation::{
        middlewares::auth::auth_middleware::AuthMiddleware, ports::router::router_port::RouterPort,
    },
};

pub struct PrivateRouter;

impl PrivateRouter {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl RouterPort for PrivateRouter {
    fn register_routes(self) -> Router {
        let auth_middleware = AuthMiddleware::new(JsonWebTokenAdapter);

        Router::new()
            .route("/home", get(|| async move { "Hello our dear customer!" }))
            .layer(middleware::from_fn({
                move |req, next| {
                    let auth_middleware = auth_middleware.clone();

                    async move { auth_middleware.handle(req, next).await }
                }
            }))
    }
}

impl Default for PrivateRouter {
    fn default() -> Self {
        Self::new()
    }
}

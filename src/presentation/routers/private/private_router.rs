use axum::{Router, middleware, routing::get};

use crate::{
    infrastructure::adapters::jsonwebtoken::jsonwebtoken_adapter::JsonWebTokenAdapter,
    presentation::{
        middlewares::auth::auth_middleware::AuthMiddleware, ports::router::router_port::RouterPort,
    },
};

pub struct PrivateRouter;

impl PrivateRouter {
    pub fn new() -> Self {
        PrivateRouter
    }
}

impl RouterPort for PrivateRouter {
    fn register_routes(self) -> Router {
        let auth_middleware: AuthMiddleware = AuthMiddleware::new(Box::new(JsonWebTokenAdapter));

        Router::new()
            .route("/home", get(|| async move { "Hello our dear customer!" }))
            .layer(middleware::from_fn({
                let auth_middleware = auth_middleware.clone();

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

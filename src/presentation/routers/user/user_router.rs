use std::collections::HashMap;

use axum::{
    Router,
    body::Body,
    extract::{Path, Request},
    http::StatusCode,
    middleware::{self},
    routing::get,
};

use crate::{
    infrastructure::adapters::{
        axum::axum_handler_adapter::AxumHandlerAdapter,
        jsonwebtoken::jsonwebtoken_adapter::JsonWebTokenAdapter,
    },
    presentation::{
        middlewares::auth::auth_middleware::AuthMiddleware,
        ports::{controller::controller_port::ControllerPort, router::router_port::RouterPort},
    },
};

pub struct UserRouter<C> {
    get_user_by_id_controller: C,
}

impl<C> UserRouter<C>
where
    C: ControllerPort + Clone + Send + Sync,
{
    #[must_use]
    pub const fn new(get_user_by_id_controller: C) -> Self {
        Self {
            get_user_by_id_controller,
        }
    }
}

impl<C> RouterPort for UserRouter<C>
where
    C: ControllerPort + Clone + Send + Sync + 'static,
{
    fn register_routes(self) -> Router {
        let auth_middleware = AuthMiddleware::new(JsonWebTokenAdapter);
        let axum_handler_adapter = AxumHandlerAdapter::new(self.get_user_by_id_controller);

        Router::new().route(
            "/users/{id}",
            get({
                move |path: Path<HashMap<String, String>>, request: Request<Body>| async move {
                    axum_handler_adapter.adapt_handler(path, request).await
                }
            })
            .options(|| async { StatusCode::OK })
            .layer(middleware::from_fn({
                move |request, next| {
                    let auth_middleware = auth_middleware.clone();

                    async move { auth_middleware.process(request, next).await }
                }
            })),
        )
    }
}

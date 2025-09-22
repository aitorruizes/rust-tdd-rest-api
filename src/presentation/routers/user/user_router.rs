use std::collections::HashMap;

use axum::{
    Router,
    body::Body,
    extract::{Path, Request},
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

pub struct UserRouter<Controller> {
    get_user_by_id_controller: Controller,
}

impl<Controller> UserRouter<Controller>
where
    Controller: ControllerPort + Clone + Send + Sync,
{
    #[must_use]
    pub const fn new(get_user_by_id_controller: Controller) -> Self {
        Self {
            get_user_by_id_controller,
        }
    }
}

impl<Controller> RouterPort for UserRouter<Controller>
where
    Controller: ControllerPort + Clone + Send + Sync + 'static,
{
    fn register_routes(self) -> Router {
        let auth_middleware = AuthMiddleware::new(JsonWebTokenAdapter);
        let axum_handler_adapter = AxumHandlerAdapter::new(self.get_user_by_id_controller);

        Router::new().route(
            "/user/{id}",
            get({
                move |path: Path<HashMap<String, String>>, req: Request<Body>| async move {
                    axum_handler_adapter.adapt_handler(path, req).await
                }
            })
            .layer(middleware::from_fn({
                move |req, next| {
                    let auth_middleware = auth_middleware.clone();

                    async move { auth_middleware.process(req, next).await }
                }
            })),
        )
    }
}

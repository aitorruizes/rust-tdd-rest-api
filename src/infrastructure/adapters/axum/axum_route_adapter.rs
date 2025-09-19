use axum::{
    Router,
    body::{Body, to_bytes},
    extract::{Path, Request},
    http::{Method, Response, StatusCode},
    routing::{delete, get, patch, post, put},
};

use serde_json::Value;
use std::{any::Any, collections::HashMap};

use crate::{
    application::ports::web_framework::web_framework_port::{
        RouterMethod, RouterWrapper, WebFrameworkRoutePort,
    },
    presentation::{
        dtos::http::{http_request_dto::HttpRequestDto, http_response_dto::HttpResponseDto},
        ports::controller::controller_port::ControllerPort,
    },
};

pub struct AxumRouterWrapper {
    router: Router<()>,
}

impl RouterWrapper for AxumRouterWrapper {
    fn into_inner(self: Box<Self>) -> Box<dyn Any + Send + Sync> {
        Box::new(self.router)
    }
}

#[derive(Clone)]
pub struct ControllerWrapper {
    controller: Box<dyn ControllerPort + Send + Sync>,
}

impl ControllerWrapper {
    pub fn new(controller: Box<dyn ControllerPort + Send + Sync>) -> Self {
        Self { controller }
    }

    pub async fn adapt_controller(
        &self,
        Path(_request_params): Path<HashMap<String, String>>,
        req: Request<Body>,
    ) -> Response<Body> {
        let method: Method = match *req.method() {
            Method::GET => Method::GET,
            Method::POST => Method::POST,
            Method::PUT => Method::PUT,
            Method::PATCH => Method::PATCH,
            Method::DELETE => Method::DELETE,
            _ => {
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("Method type is invalid."))
                    .unwrap();
            }
        };

        let uri: String = req.uri().to_string();

        let content_bytes = match to_bytes(req.into_body(), usize::MAX).await {
            Ok(bytes) => bytes,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Error reading request body."))
                    .unwrap();
            }
        };

        let content: Option<Value> = if content_bytes.is_empty() {
            None
        } else {
            match serde_json::from_slice(&content_bytes) {
                Ok(json) => Some(json),
                Err(_) => {
                    return Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Invalid JSON structure."))
                        .unwrap();
                }
            }
        };

        let http_request_dto: HttpRequestDto = HttpRequestDto {
            method: method.to_string(),
            url: uri,
            body: content,
        };

        let http_response_dto: HttpResponseDto = self.controller.handle(http_request_dto).await;

        let body_string: String = http_response_dto
            .body
            .map(|b| b.to_string())
            .unwrap_or_else(|| "{}".to_string());

        Response::builder()
            .status(http_response_dto.status_code)
            .header("content-type", "application/json")
            .body(Body::from(body_string))
            .unwrap()
    }
}

pub struct AxumRouteAdapter;

impl AxumRouteAdapter {
    pub fn new() -> Self {
        AxumRouteAdapter
    }
}

impl WebFrameworkRoutePort for AxumRouteAdapter {
    fn create_router(
        &self,
        router_method: RouterMethod,
        path: &str,
        controller: Box<dyn ControllerPort>,
    ) -> Box<dyn RouterWrapper> {
        let controller_wrapper: ControllerWrapper = ControllerWrapper::new(controller);

        let router: Router<()> = match router_method {
            RouterMethod::Get => {
                let controller = controller_wrapper.clone();

                Router::new().route(
                    path,
                    get(
                        move |path: Path<HashMap<String, String>>, req: Request<Body>| async move {
                            controller.adapt_controller(path, req).await
                        },
                    ),
                )
            }
            RouterMethod::Post => {
                let controller = controller_wrapper.clone();

                Router::new().route(
                    path,
                    post(
                        move |path: Path<HashMap<String, String>>, req: Request<Body>| async move {
                            controller.adapt_controller(path, req).await
                        },
                    ),
                )
            }
            RouterMethod::Put => {
                let controller = controller_wrapper.clone();

                Router::new().route(
                    path,
                    put(
                        move |path: Path<HashMap<String, String>>, req: Request<Body>| async move {
                            controller.adapt_controller(path, req).await
                        },
                    ),
                )
            }
            RouterMethod::Patch => {
                let controller = controller_wrapper.clone();

                Router::new().route(
                    path,
                    patch(
                        move |path: Path<HashMap<String, String>>, req: Request<Body>| async move {
                            controller.adapt_controller(path, req).await
                        },
                    ),
                )
            }
            RouterMethod::Delete => {
                let controller = controller_wrapper.clone();

                Router::new().route(
                    path,
                    delete(
                        move |path: Path<HashMap<String, String>>, req: Request<Body>| async move {
                            controller.adapt_controller(path, req).await
                        },
                    ),
                )
            }
        };

        Box::new(AxumRouterWrapper { router })
    }

    fn merge_router(
        &self,
        router: Box<dyn RouterWrapper>,
        router_to_merge: Box<dyn RouterWrapper>,
    ) -> Box<dyn RouterWrapper> {
        let merged_router: Router<()> =
            router.into_inner().downcast::<Router<()>>().unwrap().merge(
                *router_to_merge
                    .into_inner()
                    .downcast::<Router<()>>()
                    .unwrap(),
            );

        Box::new(AxumRouterWrapper {
            router: merged_router,
        })
    }

    fn create_core_router(
        &self,
        path: &str,
        router_to_merge: Box<dyn RouterWrapper>,
    ) -> Box<dyn RouterWrapper> {
        let nested_router: Router = *router_to_merge
            .into_inner()
            .downcast::<Router<()>>()
            .unwrap();

        let core_router: Router = Router::new()
            .nest(path, nested_router)
            .fallback(|| async { (StatusCode::NOT_FOUND, "Route not found") });

        Box::new(AxumRouterWrapper {
            router: core_router,
        })
    }
}

impl Default for AxumRouteAdapter {
    fn default() -> Self {
        Self::new()
    }
}

use axum::{
    body::{Body, to_bytes},
    extract::{Path, Request},
    http::{Method, Response, StatusCode},
};

use std::collections::HashMap;

use crate::presentation::{
    dtos::http::http_request_dto::HttpRequestDto,
    ports::controller::controller_port::ControllerPort,
};

#[derive(Clone)]
pub struct AxumHandlerAdapter<Controller> {
    handler: Controller,
}

impl<Controller> AxumHandlerAdapter<Controller>
where
    Controller: ControllerPort + Clone + Send + Sync,
{
    #[must_use]
    pub const fn new(handler: Controller) -> Self {
        Self { handler }
    }

    /// Adapts an incoming HTTP request into the internal handler and returns an HTTP response.
    ///
    /// This function acts as a bridge between the raw HTTP request and the application's core
    /// handler logic.
    ///
    /// # Parameters
    ///
    /// - `Path(request_params)`: a map of path parameters extracted from the request URL.
    /// - `req`: the raw HTTP request of type `Request<Body>`.
    ///
    /// # Returns
    ///
    /// Returns an `Response<Body>` containing the status code, headers, and body as produced
    /// by the internal handler.
    ///
    /// # Panics
    ///
    /// This function may panic if:
    /// - Building the response fails (via `.unwrap()` on `Response::builder().body(...)`)
    ///   which can occur if the body type is invalid.
    pub async fn adapt_handler(
        &self,
        Path(request_params): Path<HashMap<String, String>>,
        req: Request<Body>,
    ) -> Response<Body> {
        let method = match *req.method() {
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

        let uri = req.uri().to_string();

        let Ok(body_bytes) = to_bytes(req.into_body(), usize::MAX).await else {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Error reading request body."))
                .unwrap();
        };

        let body_content = if body_bytes.is_empty() {
            None
        } else {
            match serde_json::from_slice(&body_bytes) {
                Ok(json) => Some(json),
                Err(_) => {
                    return Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Invalid JSON structurerror."))
                        .unwrap();
                }
            }
        };

        let http_request_dto = HttpRequestDto {
            method: method.to_string(),
            url: uri,
            body: body_content,
            params: Some(request_params),
        };

        let http_response_dto = self.handler.handle(http_request_dto).await;

        let body_string = http_response_dto
            .body
            .map_or_else(|| "{}".to_string(), |body| body.to_string());

        Response::builder()
            .status(http_response_dto.status_code)
            .header("content-type", "application/json")
            .body(Body::from(body_string))
            .unwrap()
    }
}

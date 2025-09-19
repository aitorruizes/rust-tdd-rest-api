use axum::{
    body::{Body, to_bytes},
    extract::{Path, Request},
    http::{Method, Response, StatusCode},
};

use serde_json::Value;
use std::collections::HashMap;

use crate::presentation::{
    dtos::http::{http_request_dto::HttpRequestDto, http_response_dto::HttpResponseDto},
    ports::controller::controller_port::ControllerPort,
};

#[derive(Clone)]
pub struct AxumAdapter {
    controller: Box<dyn ControllerPort + Send + Sync>,
}

impl AxumAdapter {
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

impl Default for AxumRouteAdapter {
    fn default() -> Self {
        Self::new()
    }
}

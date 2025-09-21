use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
};
use serde_json::json;

use crate::application::ports::auth::auth_port::AuthPort;

#[derive(Clone)]
pub struct AuthMiddleware<AuthAdapter> {
    auth_port: AuthAdapter,
}

impl<AuthAdapter> AuthMiddleware<AuthAdapter>
where
    AuthAdapter: AuthPort + Clone + Send + Sync,
{
    pub fn new(auth_port: AuthAdapter) -> Self {
        Self { auth_port }
    }

    pub async fn handle(&self, req: Request<Body>, next: Next) -> Response<Body> {
        let token = match req.headers().get("Authorization") {
            Some(authorization_header) => match authorization_header.to_str() {
                Ok(value) if !value.is_empty() => value,
                _ => {
                    let body = serde_json::to_string(&json!({
                        "error_code": "missing_authorization_header",
                        "error_message": "authorization header is empty"
                    }))
                    .unwrap();

                    return Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .header("content-type", "application/json")
                        .body(Body::from(body))
                        .unwrap();
                }
            },
            None => {
                let body = serde_json::to_string(&json!({
                    "error_code": "missing_authorization_header",
                    "error_message": "authorization header is missing"
                }))
                .unwrap();

                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap();
            }
        };

        match self.auth_port.verify_auth_token(token) {
            Ok(_) => next.run(req).await,
            Err(err) => {
                let body = serde_json::to_string(&json!({
                    "error_code": "authorization_middleware",
                    "error_message": err.to_string()
                }))
                .unwrap();

                Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap()
            }
        }
    }
}

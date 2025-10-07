use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
};
use serde_json::json;

use crate::application::ports::auth::auth_port::AuthPort;

#[derive(Clone)]
pub struct AuthMiddleware<A> {
    auth_port: A,
}

impl<A> AuthMiddleware<A>
where
    A: AuthPort + Clone + Send + Sync,
{
    pub const fn new(auth_port: A) -> Self {
        Self { auth_port }
    }

    /// Process an incoming HTTP request, validating the `Authorization` header.
    ///
    /// This middleware checks for the presence of the `Authorization` header and verifies
    /// its token using the `auth_port`. If the header is missing, empty, or the token
    /// verification fails, it returns a `401 Unauthorized` response with a JSON error.
    ///
    /// # Parameters
    /// - `request`: the incoming HTTP request.
    /// - `next`: the next middleware or handler in the chain.
    ///
    /// # Returns
    /// - A `Response<Body>` representing either:
    ///   - The result of the next handler if the token is valid.
    ///   - A `401 Unauthorized` JSON response if the token is missing, empty, or invalid.
    ///
    /// # Panics
    /// - Panics if `serde_json::to_string` fails (should not happen with valid JSON literals).
    /// - Panics if building the response with `Response::builder().body(...)` fails.
    pub async fn process(&self, request: Request<Body>, next: Next) -> Response<Body> {
        let authorization_token =
            if let Some(authorization_header) = request.headers().get("Authorization") {
                match authorization_header.to_str() {
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
                }
            } else {
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
            };

        let splitted_token = authorization_token.trim_start_matches("Bearer ").trim();

        match self.auth_port.verify_auth_token(splitted_token) {
            Ok(()) => next.run(request).await,
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

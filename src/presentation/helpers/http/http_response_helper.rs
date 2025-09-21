use serde_json::{Value, json};

use crate::presentation::dtos::http::http_response_dto::HttpResponseDto;

#[derive(Clone)]
pub struct HttpResponseHelper;

impl HttpResponseHelper {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn ok(&self, body: Option<Value>) -> HttpResponseDto {
        body.map_or(
            HttpResponseDto {
                status_code: 200,
                body: None,
            },
            |value| HttpResponseDto {
                status_code: 200,
                body: Some(json!(value)),
            },
        )
    }

    #[must_use]
    pub fn bad_request(&self, body: Option<Value>) -> HttpResponseDto {
        body.map_or(
            HttpResponseDto {
                status_code: 400,
                body: None,
            },
            |value| HttpResponseDto {
                status_code: 400,
                body: Some(json!(value)),
            },
        )
    }

    #[must_use]
    pub fn too_many_requests(&self, body: Option<Value>) -> HttpResponseDto {
        body.map_or(
            HttpResponseDto {
                status_code: 429,
                body: None,
            },
            |value| HttpResponseDto {
                status_code: 429,
                body: Some(json!(value)),
            },
        )
    }

    #[must_use]
    pub fn unauthorized(&self, body: Option<Value>) -> HttpResponseDto {
        body.map_or(
            HttpResponseDto {
                status_code: 401,
                body: None,
            },
            |value| HttpResponseDto {
                status_code: 401,
                body: Some(json!(value)),
            },
        )
    }

    #[must_use]
    pub fn created(&self, body: Option<Value>) -> HttpResponseDto {
        body.map_or(
            HttpResponseDto {
                status_code: 201,
                body: None,
            },
            |value| HttpResponseDto {
                status_code: 201,
                body: Some(json!(value)),
            },
        )
    }

    #[must_use]
    pub fn no_content(&self, body: Option<Value>) -> HttpResponseDto {
        body.map_or(
            HttpResponseDto {
                status_code: 204,
                body: None,
            },
            |value| HttpResponseDto {
                status_code: 204,
                body: Some(json!(value)),
            },
        )
    }

    #[must_use]
    pub fn internal_server_error(&self, body: Option<Value>) -> HttpResponseDto {
        body.map_or(
            HttpResponseDto {
                status_code: 500,
                body: None,
            },
            |value| HttpResponseDto {
                status_code: 500,
                body: Some(json!(value)),
            },
        )
    }
}

impl Default for HttpResponseHelper {
    fn default() -> Self {
        Self::new()
    }
}

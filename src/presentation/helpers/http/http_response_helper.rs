use std::collections::HashMap;

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
                headers: None,
            },
            |value| HttpResponseDto {
                status_code: 200,
                body: Some(json!(value)),
                headers: None,
            },
        )
    }

    #[must_use]
    pub fn bad_request(&self, body: Option<Value>) -> HttpResponseDto {
        body.map_or(
            HttpResponseDto {
                status_code: 400,
                body: None,
                headers: None,
            },
            |value| HttpResponseDto {
                status_code: 400,
                body: Some(json!(value)),
                headers: None,
            },
        )
    }

    #[must_use]
    pub fn too_many_requests(&self, body: Option<Value>) -> HttpResponseDto {
        body.map_or(
            HttpResponseDto {
                status_code: 429,
                body: None,
                headers: None,
            },
            |value| HttpResponseDto {
                status_code: 429,
                body: Some(json!(value)),
                headers: None,
            },
        )
    }

    #[must_use]
    pub fn unauthorized(&self, body: Option<Value>) -> HttpResponseDto {
        body.map_or(
            HttpResponseDto {
                status_code: 401,
                body: None,
                headers: None,
            },
            |value| HttpResponseDto {
                status_code: 401,
                body: Some(json!(value)),
                headers: None,
            },
        )
    }

    #[must_use]
    pub fn created(&self, body: Value, location: &str) -> HttpResponseDto {
        let mut headers: HashMap<String, String> = HashMap::new();

        headers.insert("Location".to_string(), location.to_string());

        HttpResponseDto {
            status_code: 201,
            body: Some(body),
            headers: Some(headers),
        }
    }

    #[must_use]
    pub fn no_content(&self, body: Option<Value>) -> HttpResponseDto {
        body.map_or(
            HttpResponseDto {
                status_code: 204,
                body: None,
                headers: None,
            },
            |value| HttpResponseDto {
                status_code: 204,
                body: Some(json!(value)),
                headers: None,
            },
        )
    }

    #[must_use]
    pub fn internal_server_error(&self, body: Option<Value>) -> HttpResponseDto {
        body.map_or(
            HttpResponseDto {
                status_code: 500,
                body: None,
                headers: None,
            },
            |value| HttpResponseDto {
                status_code: 500,
                body: Some(json!(value)),
                headers: None,
            },
        )
    }

    #[must_use]
    pub fn not_found(&self, body: Option<Value>) -> HttpResponseDto {
        body.map_or(
            HttpResponseDto {
                status_code: 404,
                body: None,
                headers: None,
            },
            |value| HttpResponseDto {
                status_code: 404,
                body: Some(json!(value)),
                headers: None,
            },
        )
    }

    #[must_use]
    pub fn conflict(&self, body: Option<Value>) -> HttpResponseDto {
        body.map_or(
            HttpResponseDto {
                status_code: 409,
                body: None,
                headers: None,
            },
            |value| HttpResponseDto {
                status_code: 409,
                body: Some(json!(value)),
                headers: None,
            },
        )
    }
}

impl Default for HttpResponseHelper {
    fn default() -> Self {
        Self::new()
    }
}

use serde_json::{Value, json};

use crate::{
    application::ports::pattern_matching::pattern_matching_port::PatternMatchingError,
    presentation::{
        dtos::http::http_response_dto::HttpResponseDto,
        helpers::http::http_response_helper::HttpResponseHelper,
        ports::validator::validator_port::ValidatorPort,
    },
};

#[derive(Clone)]
pub struct HttpBodyHelper<V> {
    validator: V,
    http_response_helper: HttpResponseHelper,
}

impl<V> HttpBodyHelper<V>
where
    V: ValidatorPort + Clone + Send + Sync,
{
    #[must_use]
    pub const fn new(validator: V, http_response_helper: HttpResponseHelper) -> Self {
        Self {
            validator,
            http_response_helper,
        }
    }

    pub fn validate_request_body(&self, body_opt: Option<Value>) -> Option<HttpResponseDto> {
        let Some(body) = body_opt else {
            let body = serde_json::json!({
                "error_code": "missing_request_body",
                "error_message": "a request body was not provided"
            });

            return Some(self.http_response_helper.bad_request(Some(body)));
        };

        if let Err(errors) = self.validator.validate(&body) {
            let body = json!({
                "error_code": "invalid_request_body",
                "error_message": "some fields are not valid",
                "details": errors
            });

            return Some(self.http_response_helper.bad_request(Some(body)));
        }

        None
    }

    pub fn validate_regex<F>(
        &self,
        value: &str,
        validator: F,
        error_code: &str,
        error_message: &PatternMatchingError,
    ) -> Option<HttpResponseDto>
    where
        F: Fn(&str) -> Result<bool, PatternMatchingError>,
    {
        match validator(value) {
            Ok(true) => None,
            Ok(false) => {
                let body = json!({
                    "error_code": error_code,
                    "error_message": error_message.to_string(),
                });

                Some(self.http_response_helper.bad_request(Some(body)))
            }
            Err(err) => {
                let body = json!({
                    "error_code": "invalid_regex",
                    "error_message": err.to_string(),
                });

                Some(self.http_response_helper.internal_server_error(Some(body)))
            }
        }
    }
}

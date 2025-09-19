use std::pin::Pin;

use serde_json::{Value, json};

use crate::{
    application::{
        dtos::auth::sign_up_dto::SignUpDto,
        use_cases::auth::sign_up_use_case::{SignUpUseCaseError, SignUpUseCasePort},
    },
    infrastructure::adapters::regex::regex_adapter::{RegexAdapter, RegexError},
    presentation::{
        controllers::auth::sign_up_validator::SignUpValidator,
        dtos::http::{http_request_dto::HttpRequestDto, http_response_dto::HttpResponseDto},
        ports::controller::controller_port::ControllerPort,
    },
};

pub struct SignUpController {
    sign_up_validator: SignUpValidator,
    regex_adapter: RegexAdapter,
    sign_up_use_case: Box<dyn SignUpUseCasePort>,
}

impl SignUpController {
    pub fn new(
        sign_up_validator: SignUpValidator,
        regex_adapter: RegexAdapter,
        sign_up_use_case: Box<dyn SignUpUseCasePort>,
    ) -> Self {
        SignUpController {
            regex_adapter,
            sign_up_validator,
            sign_up_use_case,
        }
    }
}

impl ControllerPort for SignUpController {
    fn handle(
        &self,
        http_request_dto: HttpRequestDto,
    ) -> Pin<Box<dyn Future<Output = HttpResponseDto> + Send + '_>> {
        Box::pin(async move {
            let body: Value = match http_request_dto.body {
                Some(body) => body,
                None => {
                    return HttpResponseDto {
                        status_code: 400,
                        body: Some(serde_json::json!({
                            "error_code": "missing_request_body",
                            "error_message": "a request body was not provided"
                        })),
                    };
                }
            };

            if let Err(errors) = self.sign_up_validator.validate(body.clone()) {
                return HttpResponseDto {
                    status_code: 400,
                    body: Some(json!({
                        "error_code": "invalid_request_body",
                        "error_message": "some fields are not valid",
                        "details": errors
                    })),
                };
            }

            let is_valid_email: Result<bool, RegexError> = self
                .regex_adapter
                .is_valid_email(body["email"].as_str().unwrap());

            match is_valid_email {
                Ok(result) => match result {
                    true => {}
                    false => {
                        return HttpResponseDto {
                            status_code: 400,
                            body: Some(json!({
                                "error_code": "invalid_email",
                                "error_message": RegexError::InvalidEmail.to_string(),
                            })),
                        };
                    }
                },
                Err(err) => {
                    return HttpResponseDto {
                        status_code: 400,
                        body: Some(json!({
                            "error_code": "invalid_regex",
                            "error_message": err.to_string(),
                        })),
                    };
                }
            }

            let is_valid_email_domain: Result<bool, RegexError> = self
                .regex_adapter
                .is_valid_email_domain(body["email"].as_str().unwrap());

            match is_valid_email_domain {
                Ok(result) => match result {
                    true => {}
                    false => {
                        return HttpResponseDto {
                            status_code: 400,
                            body: Some(json!({
                                "error_code": "invalid_email_domain",
                                "error_message": RegexError::InvalidEmailDomain.to_string(),
                            })),
                        };
                    }
                },
                Err(err) => {
                    return HttpResponseDto {
                        status_code: 400,
                        body: Some(json!({
                            "error_code": "invalid_regex",
                            "error_message": err.to_string(),
                        })),
                    };
                }
            }

            let is_valid_password: Result<bool, RegexError> = self
                .regex_adapter
                .is_valid_password(body["password"].as_str().unwrap());

            match is_valid_password {
                Ok(result) => match result {
                    true => {}
                    false => {
                        return HttpResponseDto {
                            status_code: 400,
                            body: Some(json!({
                                "error_code": "invalid_password",
                                "error_message": RegexError::InvalidPassword.to_string(),
                            })),
                        };
                    }
                },
                Err(err) => {
                    return HttpResponseDto {
                        status_code: 400,
                        body: Some(json!({
                            "error_code": "invalid_regex",
                            "error_message": err.to_string(),
                        })),
                    };
                }
            }

            let sign_up_dto: SignUpDto = SignUpDto::new(
                body["first_name"].as_str().unwrap().to_string(),
                body["last_name"].as_str().unwrap().to_string(),
                body["email"].as_str().unwrap().to_string(),
                body["password"].as_str().unwrap().to_string(),
            );

            if let Err(err) = self.sign_up_use_case.perform(sign_up_dto).await {
                let (error_code, error_message) = match err {
                    SignUpUseCaseError::HasherError(e) => ("use_case_error", e.to_string()),
                    SignUpUseCaseError::UserError(e) => ("use_case_error", e.to_string()),
                    SignUpUseCaseError::DatabaseError(e) => ("repository_error", e.to_string()),
                };

                return HttpResponseDto {
                    status_code: 400,
                    body: Some(serde_json::json!({
                        "error_code": error_code,
                        "error_message": error_message
                    })),
                };
            }

            HttpResponseDto {
                status_code: 204,
                body: None,
            }
        })
    }
}

impl Clone for SignUpController {
    fn clone(&self) -> Self {
        Self {
            sign_up_validator: self.sign_up_validator.clone(),
            regex_adapter: self.regex_adapter.clone(),
            sign_up_use_case: self.sign_up_use_case.clone_box(),
        }
    }
}

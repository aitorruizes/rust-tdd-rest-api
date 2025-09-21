use std::pin::Pin;

use serde_json::json;

use crate::{
    application::{
        dtos::auth::sign_up_dto::SignUpDto,
        ports::pattern_matching::pattern_matching_port::{PatternMatchingPort, RegexError},
        use_cases::auth::sign_up_use_case::{SignUpUseCaseError, SignUpUseCasePort},
    },
    presentation::{
        controllers::auth::sign_up_validator::SignUpValidator,
        dtos::http::{http_request_dto::HttpRequestDto, http_response_dto::HttpResponseDto},
        ports::controller::controller_port::ControllerPort,
    },
};

#[derive(Clone)]
pub struct SignUpController<UseCase, PatternMatchingAdapter>
where
    UseCase: SignUpUseCasePort + Send + Sync + Clone + 'static,
    PatternMatchingAdapter: PatternMatchingPort + Send + Sync + Clone + 'static,
{
    sign_up_validator: SignUpValidator,
    pattern_matching_adapter: PatternMatchingAdapter,
    sign_up_use_case: UseCase,
}

impl<UseCase, PatternMatchingAdapter> SignUpController<UseCase, PatternMatchingAdapter>
where
    UseCase: SignUpUseCasePort + Send + Sync + Clone + 'static,
    PatternMatchingAdapter: PatternMatchingPort + Send + Sync + Clone + 'static,
{
    pub fn new(
        sign_up_validator: SignUpValidator,
        pattern_matching_adapter: PatternMatchingAdapter,
        sign_up_use_case: UseCase,
    ) -> Self {
        SignUpController {
            sign_up_validator,
            pattern_matching_adapter,
            sign_up_use_case,
        }
    }
}

impl<UseCase, PatternMatchingAdapter> ControllerPort
    for SignUpController<UseCase, PatternMatchingAdapter>
where
    UseCase: SignUpUseCasePort + Send + Sync + Clone + 'static,
    PatternMatchingAdapter: PatternMatchingPort + Send + Sync + Clone + 'static,
{
    fn handle(
        &self,
        http_request_dto: HttpRequestDto,
    ) -> Pin<Box<dyn Future<Output = HttpResponseDto> + Send + '_>> {
        Box::pin(async move {
            let body = match http_request_dto.body {
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

            let is_valid_email = self
                .pattern_matching_adapter
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

            let is_valid_email_domain = self
                .pattern_matching_adapter
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

            let is_valid_password = self
                .pattern_matching_adapter
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

            let sign_up_dto = SignUpDto::new(
                body["first_name"].as_str().unwrap().to_string(),
                body["last_name"].as_str().unwrap().to_string(),
                body["email"].as_str().unwrap().to_string(),
                body["password"].as_str().unwrap().to_string(),
            );

            if let Err(err) = self.sign_up_use_case.perform(sign_up_dto).await {
                let (error_code, error_message) = match err {
                    SignUpUseCaseError::HasherError(error) => ("use_case_error", error.to_string()),
                    SignUpUseCaseError::UserError(error) => ("use_case_error", error.to_string()),
                    SignUpUseCaseError::DatabaseError(error) => {
                        ("repository_error", error.to_string())
                    }
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

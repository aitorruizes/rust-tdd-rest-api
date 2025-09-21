use std::pin::Pin;

use serde_json::json;

use crate::{
    application::{
        dtos::auth::sign_in_dto::SignInDto,
        ports::pattern_matching::pattern_matching_port::{PatternMatchingPort, RegexError},
        use_cases::auth::sign_in_use_case::{SignInUseCaseError, SignInUseCasePort},
    },
    presentation::{
        controllers::auth::sign_in_validator::SignInValidator,
        dtos::http::{http_request_dto::HttpRequestDto, http_response_dto::HttpResponseDto},
        ports::controller::controller_port::ControllerPort,
    },
};

#[derive(Clone)]
pub struct SignInController<P, U> {
    sign_in_validator: SignInValidator,
    pattern_matching_adapter: P,
    sign_in_use_case: U,
}

impl<P, U> SignInController<P, U>
where
    P: PatternMatchingPort + Clone + Send + Sync,
    U: SignInUseCasePort + Clone + Send + Sync,
{
    pub fn new(
        sign_in_validator: SignInValidator,
        pattern_matching_adapter: P,
        sign_in_use_case: U,
    ) -> Self {
        SignInController {
            sign_in_validator,
            pattern_matching_adapter,
            sign_in_use_case,
        }
    }
}

impl<P, U> ControllerPort for SignInController<P, U>
where
    P: PatternMatchingPort + Clone + Send + Sync + 'static,
    U: SignInUseCasePort + Clone + Send + Sync + 'static,
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

            if let Err(errors) = self.sign_in_validator.validate(body.clone()) {
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

            let sign_in_dto = SignInDto::new(
                body["email"].as_str().unwrap().to_string(),
                body["password"].as_str().unwrap().to_string(),
            );

            match self.sign_in_use_case.perform(sign_in_dto).await {
                Ok(result) => match result {
                    Some(generated_auth_token) => HttpResponseDto {
                        status_code: 200,
                        body: Some(
                            serde_json::json!({ "authorization_token": generated_auth_token }),
                        ),
                    },
                    None => HttpResponseDto {
                        status_code: 401,
                        body: Some(
                            serde_json::json!({ "error_code": "invalid credentials", "error_message": "the provided credentials are invalid" }),
                        ),
                    },
                },
                Err(err) => {
                    let (error_code, error_message) = match err {
                        SignInUseCaseError::HasherError(error) => {
                            ("use_case_error", error.to_string())
                        }
                        SignInUseCaseError::AuthError(error) => {
                            ("use_case_error", error.to_string())
                        }
                        SignInUseCaseError::DatabaseError(error) => {
                            ("repository_error", error.to_string())
                        }
                    };

                    HttpResponseDto {
                        status_code: 400,
                        body: Some(serde_json::json!({
                            "error_code": error_code,
                            "error_message": error_message
                        })),
                    }
                }
            }
        })
    }
}

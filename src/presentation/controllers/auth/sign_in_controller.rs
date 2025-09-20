use std::{collections::HashMap, pin::Pin};

use crate::{
    application::{
        dtos::auth::sign_in_dto::SignInDto,
        use_cases::auth::sign_in_use_case::{SignInUseCaseError, SignInUseCasePort},
    },
    presentation::{
        dtos::http::{http_request_dto::HttpRequestDto, http_response_dto::HttpResponseDto},
        ports::controller::controller_port::ControllerPort,
    },
};

pub struct SignInController {
    sign_in_use_case: Box<dyn SignInUseCasePort>,
}

impl SignInController {
    pub fn new(sign_in_use_case: Box<dyn SignInUseCasePort>) -> Self {
        SignInController { sign_in_use_case }
    }
}

impl ControllerPort for SignInController {
    fn handle(
        &self,
        http_request_dto: HttpRequestDto,
    ) -> Pin<Box<dyn Future<Output = HttpResponseDto> + Send + '_>> {
        Box::pin(async move {
            let params: &HashMap<String, String> = match &http_request_dto.params {
                Some(p) => p,
                None => {
                    return HttpResponseDto {
                        status_code: 400,
                        body: Some(serde_json::json!({
                            "error_code": "missing_request_params",
                            "error_message": "request params were not provided"
                        })),
                    };
                }
            };

            let email: String = match params.get("email") {
                Some(email) => email.clone(),
                None => {
                    return HttpResponseDto {
                        status_code: 400,
                        body: Some(serde_json::json!({
                            "error_code": "missing_email_param",
                            "error_message": "the email parameter is required"
                        })),
                    };
                }
            };

            let sign_in_dto: SignInDto = SignInDto::new(email);

            match self.sign_in_use_case.perform(sign_in_dto).await {
                Ok(result) => match result {
                    Some(generated_auth_token) => HttpResponseDto {
                        status_code: 200,
                        body: Some(serde_json::json!({ "token": generated_auth_token })),
                    },
                    None => HttpResponseDto {
                        status_code: 404,
                        body: Some(serde_json::json!({ "error": "user not found" })),
                    },
                },
                Err(err) => {
                    let (error_code, error_message) = match err {
                        SignInUseCaseError::AuthError(e) => ("use_case_error", e.to_string()),
                        SignInUseCaseError::DatabaseError(e) => ("repository_error", e.to_string()),
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

impl Clone for SignInController {
    fn clone(&self) -> Self {
        Self {
            sign_in_use_case: self.sign_in_use_case.clone_box(),
        }
    }
}

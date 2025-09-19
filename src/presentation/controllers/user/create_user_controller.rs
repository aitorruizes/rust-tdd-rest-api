use std::pin::Pin;

use serde_json::{Value, json};

use crate::{
    application::use_cases::auth::sign_up_use_case::{SignUpUseCaseError, SignUpUseCasePort},
    infrastructure::adapters::regex::regex_adapter::{RegexAdapter, RegexError},
    presentation::{
        controllers::user::create_user_validator::CreateUserValidator,
        dtos::{
            http::{http_request_dto::HttpRequestDto, http_response_dto::HttpResponseDto},
            user::create_user_dto::CreateUserDto,
        },
        ports::controller::controller_port::ControllerPort,
    },
};

pub struct CreateUserController {
    create_user_validator: CreateUserValidator,
    regex_adapter: RegexAdapter,
    sign_up_use_case: Box<dyn SignUpUseCasePort>,
}

impl CreateUserController {
    pub fn new(
        create_user_validator: CreateUserValidator,
        regex_adapter: RegexAdapter,
        sign_up_use_case: Box<dyn SignUpUseCasePort>,
    ) -> Self {
        CreateUserController {
            regex_adapter,
            create_user_validator,
            sign_up_use_case,
        }
    }
}

impl ControllerPort for CreateUserController {
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

            if let Err(errors) = self.create_user_validator.validate(body.clone()) {
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

            let create_user_dto: CreateUserDto = CreateUserDto::new(
                body["first_name"].as_str().unwrap().to_string(),
                body["last_name"].as_str().unwrap().to_string(),
                body["email"].as_str().unwrap().to_string(),
                body["password"].as_str().unwrap().to_string(),
                body["password_confirmation"].as_str().unwrap().to_string(),
            );

            if let Err(err) = self.sign_up_use_case.perform(create_user_dto).await {
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

impl Clone for CreateUserController {
    fn clone(&self) -> Self {
        Self {
            create_user_validator: self.create_user_validator.clone(),
            regex_adapter: self.regex_adapter.clone(),
            sign_up_use_case: self.sign_up_use_case.clone_box(),
        }
    }
}

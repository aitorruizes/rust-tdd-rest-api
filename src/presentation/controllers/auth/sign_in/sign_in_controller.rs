use serde_json::json;

use crate::{
    application::{
        dtos::auth::sign_in_dto::SignInDto,
        ports::pattern_matching::pattern_matching_port::{
            PatternMatchingError, PatternMatchingPort,
        },
        use_cases::auth::sign_in_use_case::{
            SignInUseCaseError, SignInUseCasePort,
        },
    },
    presentation::{
        dtos::http::{http_request_dto::HttpRequestDto, http_response_dto::HttpResponseDto},
        helpers::http::{
            http_body_helper::HttpBodyHelper, http_response_helper::HttpResponseHelper,
        },
        ports::{
            controller::controller_port::{ControllerFuture, ControllerPort},
            validator::validator_port::ValidatorPort,
        },
    },
};

#[derive(Clone)]
pub struct SignInController<Validator, PatternMatchingAdapter, UseCase> {
    http_body_helper: HttpBodyHelper<Validator>,
    pattern_matching_adapter: PatternMatchingAdapter,
    sign_in_use_case: UseCase,
    http_response_helper: HttpResponseHelper,
}

impl<Validator, PatternMatchingAdapter, UseCase>
    SignInController<Validator, PatternMatchingAdapter, UseCase>
where
    Validator: ValidatorPort + Clone + Send + Sync,
    PatternMatchingAdapter: PatternMatchingPort + Clone + Send + Sync,
    UseCase: SignInUseCasePort + Clone + Send + Sync,
{
    pub const fn new(
        http_body_helper: HttpBodyHelper<Validator>,
        pattern_matching_adapter: PatternMatchingAdapter,
        sign_in_use_case: UseCase,
        http_response_helper: HttpResponseHelper,
    ) -> Self {
        Self {
            http_body_helper,
            pattern_matching_adapter,
            sign_in_use_case,
            http_response_helper,
        }
    }
}

impl<Validator, PatternMatchingAdapter, UseCase> ControllerPort
    for SignInController<Validator, PatternMatchingAdapter, UseCase>
where
    Validator: ValidatorPort + Clone + Send + Sync,
    PatternMatchingAdapter: PatternMatchingPort + Clone + Send + Sync,
    UseCase: SignInUseCasePort + Clone + Send + Sync,
{
    fn handle(&self, http_request_dto: HttpRequestDto) -> ControllerFuture<'_> {
        Box::pin(async move {
            self.http_body_helper
                .validate_request_body(http_request_dto.body.clone());

            let extracted_body = http_request_dto.body.unwrap();

            let is_valid_email = self
                .pattern_matching_adapter
                .is_valid_email(extracted_body["email"].as_str().unwrap());

            match is_valid_email {
                Ok(result) => {
                    if !result {
                        let body = json!({
                            "error_code": "invalid_email",
                            "error_message": PatternMatchingError::InvalidEmail.to_string(),
                        });

                        return self.http_response_helper.bad_request(Some(body));
                    }
                }
                Err(err) => {
                    let body = json!({
                        "error_code": "invalid_regex",
                        "error_message": err.to_string(),
                    });

                    return self.http_response_helper.internal_server_error(Some(body));
                }
            }

            let is_valid_email_domain = self
                .pattern_matching_adapter
                .is_valid_email_domain(extracted_body["email"].as_str().unwrap());

            match is_valid_email_domain {
                Ok(result) => {
                    if !result {
                        let body = Some(json!({
                            "error_code": "invalid_email_domain",
                            "error_message": PatternMatchingError::InvalidEmailDomain.to_string(),
                        }));

                        return self.http_response_helper.bad_request(body);
                    }
                }
                Err(err) => {
                    let body = json!({
                        "error_code": "invalid_regex",
                        "error_message": err.to_string(),
                    });

                    return self.http_response_helper.internal_server_error(Some(body));
                }
            }

            let sign_in_dto = SignInDto::new(
                extracted_body["email"].as_str().unwrap().to_string(),
                extracted_body["password"].as_str().unwrap().to_string(),
            );

            match self.sign_in_use_case.perform(sign_in_dto).await {
                Ok(result) => result.map_or_else(
                    || {
                        let body = json!({
                            "error_code": "invalid credentials",
                            "error_message": "the provided credentials are invalid"
                        });

                        self.http_response_helper.unauthorized(Some(body))
                    },
                    |generated_auth_token| {
                        let body = json!({ "authorization_token": generated_auth_token });

                        self.http_response_helper.ok(Some(body))
                    },
                ),
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
                        body: Some(json!({
                            "error_code": error_code,
                            "error_message": error_message
                        })),
                    }
                }
            }
        })
    }
}

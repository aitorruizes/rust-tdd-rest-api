use std::pin::Pin;

use serde_json::json;

use crate::{
    application::{
        dtos::auth::sign_up_dto::SignUpDto,
        ports::pattern_matching::pattern_matching_port::{PatternMatchingPort, RegexError},
        use_cases::auth::sign_up_use_case::{SignUpUseCaseError, SignUpUseCasePort},
    },
    presentation::{
        dtos::http::{http_request_dto::HttpRequestDto, http_response_dto::HttpResponseDto},
        helpers::http::{
            http_body_helper::HttpBodyHelper, http_response_helper::HttpResponseHelper,
        },
        ports::{
            controller::controller_port::ControllerPort, validator::validator_port::ValidatorPort,
        },
    },
};

#[derive(Clone)]
pub struct SignUpController<Validator, UseCase, PatternMatchingAdapter>
where
    Validator: ValidatorPort + Clone + Send + Sync,
    UseCase: SignUpUseCasePort + Send + Sync + Clone + 'static,
    PatternMatchingAdapter: PatternMatchingPort + Send + Sync + Clone + 'static,
{
    http_body_helper: HttpBodyHelper<Validator>,
    pattern_matching_adapter: PatternMatchingAdapter,
    sign_up_use_case: UseCase,
    http_response_helper: HttpResponseHelper,
}

impl<Validator, UseCase, PatternMatchingAdapter>
    SignUpController<Validator, UseCase, PatternMatchingAdapter>
where
    Validator: ValidatorPort + Clone + Send + Sync,
    UseCase: SignUpUseCasePort + Send + Sync + Clone + 'static,
    PatternMatchingAdapter: PatternMatchingPort + Send + Sync + Clone + 'static,
{
    pub const fn new(
        http_body_helper: HttpBodyHelper<Validator>,
        pattern_matching_adapter: PatternMatchingAdapter,
        sign_up_use_case: UseCase,
        http_response_helper: HttpResponseHelper,
    ) -> Self {
        Self {
            http_body_helper,
            pattern_matching_adapter,
            sign_up_use_case,
            http_response_helper,
        }
    }
}

impl<Validator, UseCase, PatternMatchingAdapter> ControllerPort
    for SignUpController<Validator, UseCase, PatternMatchingAdapter>
where
    Validator: ValidatorPort + Clone + Send + Sync,
    UseCase: SignUpUseCasePort + Send + Sync + Clone + 'static,
    PatternMatchingAdapter: PatternMatchingPort + Send + Sync + Clone + 'static,
{
    fn handle(
        &self,
        http_request_dto: HttpRequestDto,
    ) -> Pin<Box<dyn Future<Output = HttpResponseDto> + Send + '_>> {
        Box::pin(async move {
            self.http_body_helper
                .validate_request_body(http_request_dto.body.clone());

            let extracted_body = http_request_dto.body.unwrap();

            if let Some(http_response_dto) = self.http_body_helper.validate_regex(
                extracted_body["email"].as_str().unwrap(),
                |v| self.pattern_matching_adapter.is_valid_email(v),
                "invalid_email",
                &RegexError::InvalidEmail,
            ) {
                return http_response_dto;
            }

            if let Some(http_response_dto) = self.http_body_helper.validate_regex(
                extracted_body["email"].as_str().unwrap(),
                |v| self.pattern_matching_adapter.is_valid_email_domain(v),
                "invalid_email_domain",
                &RegexError::InvalidEmailDomain,
            ) {
                return http_response_dto;
            }

            if let Some(http_response_dto) = self.http_body_helper.validate_regex(
                extracted_body["password"].as_str().unwrap(),
                |v| self.pattern_matching_adapter.is_valid_password(v),
                "invalid_password",
                &RegexError::InvalidPassword,
            ) {
                return http_response_dto;
            }

            let sign_up_dto = SignUpDto::new(
                extracted_body["first_name"].as_str().unwrap().to_string(),
                extracted_body["last_name"].as_str().unwrap().to_string(),
                extracted_body["email"].as_str().unwrap().to_string(),
                extracted_body["password"].as_str().unwrap().to_string(),
            );

            if let Err(err) = self.sign_up_use_case.perform(sign_up_dto).await {
                let body = match &err {
                    SignUpUseCaseError::HasherError(error) => {
                        json!({
                            "error_code": "internal_server_error",
                            "error_message": error.to_string()
                        })
                    }
                    SignUpUseCaseError::DatabaseError(error) => {
                        json!({
                            "error_code": "internal_server_error",
                            "error_message": error.to_string()
                        })
                    }
                    SignUpUseCaseError::UserError(error) => {
                        json!({
                            "error_code": "use_case_error",
                            "error_message": error.to_string()
                        })
                    }
                };

                return match err {
                    SignUpUseCaseError::UserError(_) => {
                        self.http_response_helper.bad_request(Some(body))
                    }
                    SignUpUseCaseError::HasherError(_) | SignUpUseCaseError::DatabaseError(_) => {
                        self.http_response_helper.internal_server_error(Some(body))
                    }
                };
            }

            self.http_response_helper.no_content(None)
        })
    }
}

use serde_json::json;

use crate::{
    application::{
        dtos::auth::sign_up_dto::SignUpDto,
        ports::pattern_matching::pattern_matching_port::{
            PatternMatchingError, PatternMatchingPort,
        },
        use_cases::auth::sign_up_use_case::{SignUpUseCaseError, SignUpUseCasePort},
    },
    domain::errors::user::user_errors::UserError,
    infrastructure::mappers::response::user::user_response::UserResponse,
    presentation::{
        dtos::http::http_request_dto::HttpRequestDto,
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
pub struct SignUpController<V, P, U> {
    http_body_helper: HttpBodyHelper<V>,
    pattern_matching_adapter: P,
    sign_up_use_case: U,
    http_response_helper: HttpResponseHelper,
}

impl<V, P, U> SignUpController<V, P, U>
where
    V: ValidatorPort + Send + Sync + Clone,
    P: PatternMatchingPort + Send + Sync + Clone + 'static,
    U: SignUpUseCasePort + Send + Sync + Clone + 'static,
{
    pub const fn new(
        http_body_helper: HttpBodyHelper<V>,
        pattern_matching_adapter: P,
        sign_up_use_case: U,
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

impl<V, P, U> ControllerPort for SignUpController<V, P, U>
where
    V: ValidatorPort + Clone + Send + Sync,
    P: PatternMatchingPort + Send + Sync + Clone + 'static,
    U: SignUpUseCasePort + Send + Sync + Clone + 'static,
{
    fn handle(&self, http_request_dto: HttpRequestDto) -> ControllerFuture<'_> {
        Box::pin(async move {
            if let Some(http_response_dto) = self
                .http_body_helper
                .validate_request_body(http_request_dto.body.clone())
            {
                return http_response_dto;
            }

            let extracted_body = http_request_dto.body.unwrap();

            if let Some(http_response_dto) = self.http_body_helper.validate_regex(
                extracted_body["email"].as_str().unwrap(),
                |v| self.pattern_matching_adapter.is_valid_email(v),
                "invalid_email",
                &PatternMatchingError::InvalidEmail,
            ) {
                return http_response_dto;
            }

            if let Some(http_response_dto) = self.http_body_helper.validate_regex(
                extracted_body["email"].as_str().unwrap(),
                |v| self.pattern_matching_adapter.is_valid_email_domain(v),
                "invalid_email_domain",
                &PatternMatchingError::InvalidEmailDomain,
            ) {
                return http_response_dto;
            }

            if let Some(http_response_dto) = self.http_body_helper.validate_regex(
                extracted_body["password"].as_str().unwrap(),
                |v| self.pattern_matching_adapter.is_valid_password(v),
                "invalid_password",
                &PatternMatchingError::InvalidPassword,
            ) {
                return http_response_dto;
            }

            let sign_up_dto = SignUpDto::new(
                extracted_body["firstName"].as_str().unwrap().to_string(),
                extracted_body["lastName"].as_str().unwrap().to_string(),
                extracted_body["email"].as_str().unwrap().to_string(),
                extracted_body["password"].as_str().unwrap().to_string(),
                extracted_body["passwordConfirmation"]
                    .as_str()
                    .unwrap()
                    .to_string(),
            );

            match self.sign_up_use_case.perform(sign_up_dto).await {
                Ok(user_entity) => {
                    let user_response = UserResponse::from(user_entity);
                    let location = format!("/users/{}", user_response.id);

                    self.http_response_helper
                        .created(json!({ "user": user_response }), &location)
                }
                Err(err) => {
                    let body = match &err {
                        SignUpUseCaseError::HasherError(error) => {
                            json!({
                                "error_code": "internal_server_error",
                                "error_message": error.to_string()
                            })
                        }
                        SignUpUseCaseError::RepositoryError(error) => {
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

                    match err {
                        SignUpUseCaseError::UserError(error) => match error {
                            UserError::PasswordsDoNotMatch => {
                                self.http_response_helper.bad_request(Some(body))
                            }
                            UserError::UserAlreadyExists => {
                                self.http_response_helper.conflict(Some(body))
                            }
                        },
                        SignUpUseCaseError::HasherError(_)
                        | SignUpUseCaseError::RepositoryError(_) => {
                            self.http_response_helper.internal_server_error(Some(body))
                        }
                    }
                }
            }
        })
    }
}

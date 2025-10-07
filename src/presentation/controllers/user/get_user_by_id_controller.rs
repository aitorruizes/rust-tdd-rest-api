use serde_json::json;

use crate::{
    application::{
        ports::pattern_matching::pattern_matching_port::{
            PatternMatchingError, PatternMatchingPort,
        },
        use_cases::user::get_user_by_id_use_case::GetUserByIdUseCasePort,
    },
    presentation::{
        dtos::http::http_request_dto::HttpRequestDto,
        helpers::http::http_response_helper::HttpResponseHelper,
        ports::controller::controller_port::{ControllerFuture, ControllerPort},
    },
};

#[derive(Clone)]
pub struct GetUserByIdController<PatternMatchingAdapter, UseCase> {
    pattern_matching_adapter: PatternMatchingAdapter,
    get_user_by_id_use_case: UseCase,
    http_response_helper: HttpResponseHelper,
}

impl<PatternMatchingAdapter, UseCase> GetUserByIdController<PatternMatchingAdapter, UseCase>
where
    PatternMatchingAdapter: PatternMatchingPort + Clone + Send + Sync,
    UseCase: GetUserByIdUseCasePort + Clone + Send + Sync,
{
    pub const fn new(
        pattern_matching_adapter: PatternMatchingAdapter,
        get_user_by_id_use_case: UseCase,
        http_response_helper: HttpResponseHelper,
    ) -> Self {
        Self {
            pattern_matching_adapter,
            get_user_by_id_use_case,
            http_response_helper,
        }
    }
}

impl<PatternMatchingAdapter, UseCase> ControllerPort
    for GetUserByIdController<PatternMatchingAdapter, UseCase>
where
    PatternMatchingAdapter: PatternMatchingPort + Clone + Send + Sync,
    UseCase: GetUserByIdUseCasePort + Clone + Send + Sync,
{
    fn handle(&self, http_request_dto: HttpRequestDto) -> ControllerFuture<'_> {
        Box::pin(async move {
            let params = http_request_dto.params.unwrap();
            let id = params.get("id").unwrap();
            let is_valid_uuid = self.pattern_matching_adapter.is_valid_uuid(id);

            match is_valid_uuid {
                Ok(result) => {
                    if !result {
                        let body = json!({
                            "error_code": "invalid_uuid",
                            "error_message": PatternMatchingError::InvalidUuid.to_string(),
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

            match self.get_user_by_id_use_case.perform(id.to_string()).await {
                Ok(result) => result.map_or_else(
                    || {
                        let body = json!({
                            "error_code": "user_not_found",
                            "error_message": "no user with the provided id was found"
                        });

                        self.http_response_helper.not_found(Some(body))
                    },
                    |user| {
                        let body = json!({ "data": user });

                        self.http_response_helper.ok(Some(body))
                    },
                ),
                Err(err) => {
                    let body = json!({
                        "error_code": "internal_server_error",
                        "error_message": err.to_string()
                    });

                    self.http_response_helper.internal_server_error(Some(body))
                }
            }
        })
    }
}

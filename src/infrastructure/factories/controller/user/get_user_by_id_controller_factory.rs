use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{
    application::use_cases::user::get_user_by_id_use_case::GetUserByIdUseCase,
    infrastructure::{
        adapters::regex::regex_adapter::RegexAdapter,
        repositories::user::get_user_by_id_repository::GetUserByIdRepository,
    },
    presentation::{
        controllers::user::get_user_by_id_controller::GetUserByIdController,
        helpers::http::http_response_helper::HttpResponseHelper,
    },
};

pub struct GetUserByIdControllerFactory {
    database_pool: Arc<Pool<Postgres>>,
}

impl GetUserByIdControllerFactory {
    #[must_use]
    pub const fn new(database_pool: Arc<Pool<Postgres>>) -> Self {
        Self { database_pool }
    }

    #[must_use]
    pub fn build(
        &self,
    ) -> GetUserByIdController<RegexAdapter, GetUserByIdUseCase<GetUserByIdRepository>> {
        let pattern_matching_adapter = RegexAdapter;
        let get_user_by_id_repository = GetUserByIdRepository::new(self.database_pool.clone());
        let get_user_by_id_use_case = GetUserByIdUseCase::new(get_user_by_id_repository);
        let http_response_helper = HttpResponseHelper::new();

        GetUserByIdController::new(
            pattern_matching_adapter,
            get_user_by_id_use_case,
            http_response_helper,
        )
    }
}

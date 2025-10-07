use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{
    application::use_cases::auth::sign_up_use_case::SignUpUseCase,
    infrastructure::{
        adapters::{
            bcrypt::bcrypt_adapter::BcryptAdapter, regex::regex_adapter::RegexAdapter,
            uuid::uuid_adapter::UuidAdapter,
        },
        repositories::user::{
            create_user_repository::CreateUserRepository,
            get_user_by_email_repository::GetUserByEmailRepository,
        },
    },
    presentation::{
        controllers::auth::sign_up::{
            sign_up_controller::SignUpController, sign_up_validator::SignUpValidator,
        },
        helpers::http::{
            http_body_helper::HttpBodyHelper, http_response_helper::HttpResponseHelper,
        },
    },
};

pub struct SignUpControllerFactory {
    database_pool: Arc<Pool<Postgres>>,
}

impl SignUpControllerFactory {
    #[must_use]
    pub const fn new(database_pool: Arc<Pool<Postgres>>) -> Self {
        Self { database_pool }
    }

    #[must_use]
    pub fn build(
        &self,
    ) -> SignUpController<
        SignUpValidator,
        RegexAdapter,
        SignUpUseCase<BcryptAdapter, UuidAdapter, CreateUserRepository, GetUserByEmailRepository>,
    > {
        let sign_up_validator = SignUpValidator;
        let pattern_matching_adapter = RegexAdapter;
        let hasher_adapter = BcryptAdapter;
        let id_generator_adapter = UuidAdapter;
        let create_user_repository = CreateUserRepository::new(self.database_pool.clone());

        let get_user_by_email_repository =
            GetUserByEmailRepository::new(self.database_pool.clone());

        let http_response_helper = HttpResponseHelper::new();
        let http_body_helper = HttpBodyHelper::new(sign_up_validator, http_response_helper.clone());

        let sign_up_use_case = SignUpUseCase::new(
            hasher_adapter,
            id_generator_adapter,
            create_user_repository,
            get_user_by_email_repository,
        );

        SignUpController::new(
            http_body_helper,
            pattern_matching_adapter,
            sign_up_use_case,
            http_response_helper,
        )
    }
}

use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{
    application::use_cases::user::get_user_by_email_use_case::GetUserByEmailUseCase,
    infrastructure::{
        adapters::{
            bcrypt::bcrypt_adapter::BcryptAdapter,
            jsonwebtoken::jsonwebtoken_adapter::JsonWebTokenAdapter,
            regex::regex_adapter::RegexAdapter,
        },
        repositories::user::get_user_by_email_repository::GetUserByEmailRepository,
    },
    presentation::{
        controllers::auth::{
            sign_in::sign_in_controller::SignInController,
            sign_in::sign_in_validator::SignInValidator,
        },
        helpers::http::{
            http_body_helper::HttpBodyHelper, http_response_helper::HttpResponseHelper,
        },
    },
};

pub struct SignInControllerFactory {
    database_pool: Arc<Pool<Postgres>>,
}

impl SignInControllerFactory {
    #[must_use]
    pub const fn new(database_pool: Arc<Pool<Postgres>>) -> Self {
        Self { database_pool }
    }

    #[must_use]
    pub fn build(
        &self,
    ) -> SignInController<
        SignInValidator,
        RegexAdapter,
        GetUserByEmailUseCase<BcryptAdapter, JsonWebTokenAdapter, GetUserByEmailRepository>,
    > {
        let hasher_adapter = BcryptAdapter;
        let auth_adapter = JsonWebTokenAdapter;
        let pattern_matching_adapter = RegexAdapter;

        let get_user_by_email_repository =
            GetUserByEmailRepository::new(self.database_pool.clone());

        let get_user_by_email_use_case =
            GetUserByEmailUseCase::new(hasher_adapter, auth_adapter, get_user_by_email_repository);

        let sign_in_validator = SignInValidator;
        let http_response_helper = HttpResponseHelper::new();
        let http_body_helper = HttpBodyHelper::new(sign_in_validator, http_response_helper.clone());

        SignInController::new(
            http_body_helper,
            pattern_matching_adapter,
            get_user_by_email_use_case,
            http_response_helper,
        )
    }
}

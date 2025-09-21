use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{
    application::use_cases::auth::sign_in_use_case::SignInUseCase,
    infrastructure::{
        adapters::{
            bcrypt::bcrypt_adapter::BcryptAdapter,
            jsonwebtoken::jsonwebtoken_adapter::JsonWebTokenAdapter,
            regex::regex_adapter::RegexAdapter,
        },
        repositories::auth::sign_in_repository::SignInRepository,
    },
    presentation::controllers::auth::{
        sign_in_controller::SignInController, sign_in_validator::SignInValidator,
    },
};

pub struct SignInControllerFactory {
    database_pool: Arc<Pool<Postgres>>,
}

impl SignInControllerFactory {
    pub fn new(database_pool: Arc<Pool<Postgres>>) -> Self {
        SignInControllerFactory { database_pool }
    }

    pub fn build(
        &self,
    ) -> SignInController<
        RegexAdapter,
        SignInUseCase<BcryptAdapter, JsonWebTokenAdapter, SignInRepository>,
    > {
        let hasher_adapter = BcryptAdapter;
        let auth_adapter = JsonWebTokenAdapter;
        let sign_in_validator = SignInValidator;
        let pattern_matching_adapter = RegexAdapter;
        let sign_in_repository = SignInRepository::new(self.database_pool.clone());
        let sign_in_use_case = SignInUseCase::new(hasher_adapter, auth_adapter, sign_in_repository);

        SignInController::new(
            sign_in_validator,
            pattern_matching_adapter,
            sign_in_use_case,
        )
    }
}

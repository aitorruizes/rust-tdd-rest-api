use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{
    application::use_cases::auth::sign_up_use_case::SignUpUseCase,
    infrastructure::{
        adapters::{
            bcrypt::bcrypt_adapter::BcryptAdapter, regex::regex_adapter::RegexAdapter,
            uuid::uuid_adapter::UuidAdapter,
        },
        repositories::auth::sign_up_repository::SignUpRepository,
    },
    presentation::controllers::auth::{
        sign_up_controller::SignUpController, sign_up_validator::SignUpValidator,
    },
};

pub struct SignUpControllerFactory {
    database_pool: Arc<Pool<Postgres>>,
}

impl SignUpControllerFactory {
    pub fn new(database_pool: Arc<Pool<Postgres>>) -> Self {
        SignUpControllerFactory { database_pool }
    }

    pub fn build(
        &self,
    ) -> SignUpController<SignUpUseCase<BcryptAdapter, UuidAdapter, SignUpRepository>, RegexAdapter>
    {
        let sign_up_validator = SignUpValidator;
        let pattern_matching_adapter = RegexAdapter;
        let hasher_adapter = BcryptAdapter;
        let id_generator_adapter = UuidAdapter;
        let sign_up_repository = SignUpRepository::new(self.database_pool.clone());

        let sign_up_use_case =
            SignUpUseCase::new(hasher_adapter, id_generator_adapter, sign_up_repository);

        SignUpController::new(
            sign_up_validator,
            pattern_matching_adapter,
            sign_up_use_case,
        )
    }
}

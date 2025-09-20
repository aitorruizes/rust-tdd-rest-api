use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{
    application::{
        ports::auth::sign_up_repository_port::SignUpRepositoryPort,
        use_cases::auth::sign_up_use_case::SignUpUseCase,
    },
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

    pub fn build(&self) -> SignUpController {
        let bcrypt_adapter: BcryptAdapter = BcryptAdapter;
        let uuid_adapter: UuidAdapter = UuidAdapter;

        let sign_up_repository: Box<dyn SignUpRepositoryPort> =
            Box::new(SignUpRepository::new(self.database_pool.clone()));

        let sign_up_use_case: SignUpUseCase = SignUpUseCase::new(
            Box::new(bcrypt_adapter),
            Box::new(uuid_adapter),
            sign_up_repository,
        );

        let sign_up_validator: SignUpValidator = SignUpValidator;
        let regex_adapter: RegexAdapter = RegexAdapter;

        SignUpController::new(sign_up_validator, regex_adapter, Box::new(sign_up_use_case))
    }
}

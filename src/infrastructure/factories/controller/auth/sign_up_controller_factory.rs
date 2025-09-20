use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{
    application::{
        ports::{
            hasher::hasher_port::HasherPort, id_generator::id_generator_port::IdGeneratorPort,
            pattern_matching::pattern_matching_port::PatternMatchingPort,
            repositories::sign_up_repository_port::SignUpRepositoryPort,
        },
        use_cases::auth::sign_up_use_case::{SignUpUseCase, SignUpUseCasePort},
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
        let hasher_adapter: Box<dyn HasherPort> = Box::new(BcryptAdapter);
        let id_generator_adapter: Box<dyn IdGeneratorPort> = Box::new(UuidAdapter);

        let sign_up_repository: Box<dyn SignUpRepositoryPort> =
            Box::new(SignUpRepository::new(self.database_pool.clone()));

        let sign_up_use_case: Box<dyn SignUpUseCasePort> = Box::new(SignUpUseCase::new(
            hasher_adapter,
            id_generator_adapter,
            sign_up_repository,
        ));

        let sign_up_validator: SignUpValidator = SignUpValidator;
        let pattern_matching_adapter: Box<dyn PatternMatchingPort> = Box::new(RegexAdapter);

        SignUpController::new(
            sign_up_validator,
            pattern_matching_adapter,
            sign_up_use_case,
        )
    }
}

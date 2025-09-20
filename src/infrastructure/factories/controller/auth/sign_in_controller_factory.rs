use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{
    application::{
        ports::{
            auth::auth_port::AuthPort, hasher::hasher_port::HasherPort,
            pattern_matching::pattern_matching_port::PatternMatchingPort,
            repositories::sign_in_repository_port::SignInRepositoryPort,
        },
        use_cases::auth::sign_in_use_case::{SignInUseCase, SignInUseCasePort},
    },
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

    pub fn build(&self) -> SignInController {
        let hasher_adapter: Box<dyn HasherPort> = Box::new(BcryptAdapter);
        let auth_adapter: Box<dyn AuthPort> = Box::new(JsonWebTokenAdapter);
        let sign_in_validator: SignInValidator = SignInValidator;
        let pattern_matching_adapter: Box<dyn PatternMatchingPort> = Box::new(RegexAdapter);

        let sign_in_repository: Box<dyn SignInRepositoryPort> =
            Box::new(SignInRepository::new(self.database_pool.clone()));

        let sign_in_use_case: Box<dyn SignInUseCasePort> = Box::new(SignInUseCase::new(
            hasher_adapter,
            auth_adapter,
            sign_in_repository,
        ));

        SignInController::new(
            sign_in_validator,
            pattern_matching_adapter,
            sign_in_use_case,
        )
    }
}

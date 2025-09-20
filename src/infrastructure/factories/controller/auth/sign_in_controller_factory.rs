use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{
    application::use_cases::auth::sign_in_use_case::SignInUseCase,
    infrastructure::{
        adapters::jsonwebtoken::jsonwebtoken_adapter::JsonWebTokenAdapter,
        repositories::auth::sign_in_repository::SignInRepository,
    },
    presentation::controllers::auth::sign_in_controller::SignInController,
};

pub struct SignInControllerFactory {
    database_pool: Arc<Pool<Postgres>>,
}

impl SignInControllerFactory {
    pub fn new(database_pool: Arc<Pool<Postgres>>) -> Self {
        SignInControllerFactory { database_pool }
    }

    pub fn build(&self) -> SignInController {
        let auth_adapter: JsonWebTokenAdapter = JsonWebTokenAdapter;

        let sign_in_repository: SignInRepository =
            SignInRepository::new(self.database_pool.clone());

        let sign_in_use_case: SignInUseCase =
            SignInUseCase::new(Box::new(auth_adapter), Box::new(sign_in_repository));

        SignInController::new(Box::new(sign_in_use_case))
    }
}

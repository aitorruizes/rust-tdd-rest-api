use std::{pin::Pin, sync::Arc};

use sqlx::{Pool, Postgres};

use crate::{
    application::ports::repositories::sign_in_repository_port::{
        SignInRepositoryError, SignInRepositoryPort,
    },
    domain::entities::user::user_entity::UserEntity,
};

#[derive(Clone)]
pub struct SignInRepository {
    database_pool: Arc<Pool<Postgres>>,
}

impl SignInRepository {
    pub fn new(database_pool: Arc<Pool<Postgres>>) -> Self {
        SignInRepository { database_pool }
    }
}

impl SignInRepositoryPort for SignInRepository {
    fn execute(
        &self,
        email: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<UserEntity>, SignInRepositoryError>> + Send + '_>>
    {
        Box::pin(async move {
            let user_entity =
                sqlx::query_as!(UserEntity, "SELECT * FROM users WHERE email = $1", email)
                    .fetch_optional(&*self.database_pool)
                    .await
                    .map_err(|err| SignInRepositoryError::FindByEmailError {
                        message: err.to_string(),
                    })?;

            Ok(user_entity)
        })
    }
}

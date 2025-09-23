use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{
    application::ports::repositories::user::get_user_by_email_repository_port::{
        GetUserByEmailRepositoryError, GetUserByEmailRepositoryFuture, GetUserByEmailRepositoryPort,
    },
    domain::entities::user::user_entity::UserEntity,
};

#[derive(Clone)]
pub struct GetUserByEmailRepository {
    database_pool: Arc<Pool<Postgres>>,
}

impl GetUserByEmailRepository {
    #[must_use]
    pub const fn new(database_pool: Arc<Pool<Postgres>>) -> Self {
        Self { database_pool }
    }
}

impl GetUserByEmailRepositoryPort for GetUserByEmailRepository {
    fn execute(&self, email: String) -> GetUserByEmailRepositoryFuture<'_> {
        Box::pin(async move {
            let user_entity =
                sqlx::query_as!(UserEntity, "SELECT * FROM users WHERE email = $1", email)
                    .fetch_optional(&*self.database_pool)
                    .await
                    .map_err(|err| GetUserByEmailRepositoryError::FindByEmailError {
                        message: err.to_string(),
                    })?;

            Ok(user_entity)
        })
    }
}

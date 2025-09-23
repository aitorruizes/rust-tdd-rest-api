use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{
    application::ports::repositories::user::create_user_repository_port::{
        CreateUserRepositoryError, CreateUserRepositoryFuture, CreateUserRepositoryPort,
    },
    domain::entities::user::user_entity::UserEntity,
};

#[derive(Clone)]
pub struct CreateUserRepository {
    database_pool: Arc<Pool<Postgres>>,
}

impl CreateUserRepository {
    #[must_use]
    pub const fn new(database_pool: Arc<Pool<Postgres>>) -> Self {
        Self { database_pool }
    }
}

impl CreateUserRepositoryPort for CreateUserRepository {
    fn execute(&self, user_entity: UserEntity) -> CreateUserRepositoryFuture<'_> {
        Box::pin(async move {
            sqlx::query!(
                r#"
                INSERT INTO users (id, first_name, last_name, email, password)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                user_entity.id,
                user_entity.first_name,
                user_entity.last_name,
                user_entity.email,
                user_entity.password
            )
            .execute(&*self.database_pool)
            .await
            .map_err(|err| CreateUserRepositoryError::InsertError {
                message: err.to_string(),
            })?;

            Ok(())
        })
    }
}

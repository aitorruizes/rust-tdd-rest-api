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
            let created_user = sqlx::query_as!(
                UserEntity,
                r#"
                INSERT INTO users (id, first_name, last_name, email, password, is_admin, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING *
                "#,
                user_entity.id,
                user_entity.first_name,
                user_entity.last_name,
                user_entity.email,
                user_entity.password,
                user_entity.is_admin,
                user_entity.created_at,
                user_entity.updated_at,
            )
            .fetch_one(&*self.database_pool)
            .await
            .map_err(|err| CreateUserRepositoryError::InsertError {
                message: err.to_string(),
            })?;

            Ok(created_user)
        })
    }
}

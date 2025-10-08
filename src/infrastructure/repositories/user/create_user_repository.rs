use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{
    application::ports::repositories::user::create_user_repository_port::{
        CreateUserRepositoryError, CreateUserRepositoryFuture, CreateUserRepositoryPort,
    },
    domain::entities::user::user_entity::UserEntity,
    infrastructure::models::user::user_model::UserModel,
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
            let user_model = UserModel::from(user_entity);

            let created_user = sqlx::query_as!(
                UserModel,
                r#"
                INSERT INTO users (id, first_name, last_name, email, password, is_admin, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING *
                "#,
                user_model.id,
                user_model.first_name,
                user_model.last_name,
                user_model.email,
                user_model.password,
                user_model.is_admin,
                user_model.created_at,
                user_model.updated_at,
            )
            .fetch_one(&*self.database_pool)
            .await
            .map_err(|err| CreateUserRepositoryError::InsertError {
                message: err.to_string(),
            })?;

            let user_entity = created_user.into();

            Ok(user_entity)
        })
    }
}

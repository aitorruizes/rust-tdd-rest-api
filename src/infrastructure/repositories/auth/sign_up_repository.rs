use std::{pin::Pin, sync::Arc};

use sqlx::{Pool, Postgres};

use crate::{
    application::ports::auth::sign_up_repository_port::{
        SignUpRepositoryError, SignUpRepositoryPort,
    },
    domain::entities::user::user_entity::UserEntity,
};

#[derive(Clone)]
pub struct SignUpRepository {
    database_pool: Arc<Pool<Postgres>>,
}

impl SignUpRepository {
    pub fn new(database_pool: Arc<Pool<Postgres>>) -> Self {
        SignUpRepository { database_pool }
    }
}

impl SignUpRepositoryPort for SignUpRepository {
    fn execute(
        &self,
        user_entity: UserEntity,
    ) -> Pin<Box<dyn Future<Output = Result<(), SignUpRepositoryError>> + Send + '_>> {
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
            .map_err(|err| SignUpRepositoryError::InsertError {
                message: err.to_string(),
            })?;

            Ok(())
        })
    }
}

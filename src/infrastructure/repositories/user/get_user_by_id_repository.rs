use std::sync::Arc;

use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    application::ports::repositories::user::get_user_by_id_repository_port::{
        GetUserByIdFuture, GetUserByIdRepositoryError, GetUserByIdRepositoryPort,
    },
    infrastructure::models::user::user_model::UserModel,
};

#[derive(Clone)]
pub struct GetUserByIdRepository {
    database_pool: Arc<Pool<Postgres>>,
}

impl GetUserByIdRepository {
    #[must_use]
    pub const fn new(database_pool: Arc<Pool<Postgres>>) -> Self {
        Self { database_pool }
    }
}

impl GetUserByIdRepositoryPort for GetUserByIdRepository {
    fn execute(&self, id: String) -> GetUserByIdFuture<'_> {
        Box::pin(async move {
            let user_uuid =
                Uuid::parse_str(&id).map_err(|_| GetUserByIdRepositoryError::FindByIdError {
                    message: "Invalid UUID format".to_string(),
                })?;

            let user_model =
                sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = $1", user_uuid)
                    .fetch_optional(&*self.database_pool)
                    .await
                    .map_err(|err| GetUserByIdRepositoryError::FindByIdError {
                        message: err.to_string(),
                    })?;

            let user_entity = user_model.map(Into::into);

            Ok(user_entity)
        })
    }
}

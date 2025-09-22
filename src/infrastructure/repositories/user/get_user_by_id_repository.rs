use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    application::ports::repositories::user::get_user_by_id_repository_port::{
        GetUserByIdFUture, GetUserByIdRepositoryError, GetUserByIdRepositoryPort,
    },
    domain::entities::user::user_entity::UserEntity,
};

pub struct GetUserByIdRepository {
    database_pool: Pool<Postgres>,
}

impl GetUserByIdRepository {
    pub const fn new(database_pool: Pool<Postgres>) -> Self {
        Self { database_pool }
    }
}

impl GetUserByIdRepositoryPort for GetUserByIdRepository {
    fn execute(&self, id: String) -> GetUserByIdFUture<'_> {
        Box::pin(async move {
            let user_entity = sqlx::query_as!(
                UserEntity,
                "SELECT * FROM users WHERE id = $1",
                Uuid::parse_str(&id).unwrap()
            )
            .fetch_optional(&self.database_pool)
            .await
            .map_err(|err| GetUserByIdRepositoryError::FindByIdError {
                message: err.to_string(),
            })?;

            Ok(user_entity)
        })
    }
}

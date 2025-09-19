use std::pin::Pin;

use sqlx::{Pool, Postgres};

use crate::{
    application::ports::database::user_database_port::{UserDatabaseError, UserDatabasePort},
    domain::entities::user::user_entity::UserEntity,
};

#[derive(Clone)]
pub struct UserDatabaseGateway {
    database_pool: Pool<Postgres>,
}

impl UserDatabaseGateway {
    pub fn new(database_pool: Pool<Postgres>) -> Self {
        UserDatabaseGateway { database_pool }
    }
}

impl UserDatabasePort for UserDatabaseGateway {
    fn insert_user(
        &self,
        user_entity: UserEntity,
    ) -> Pin<Box<dyn Future<Output = Result<(), UserDatabaseError>> + Send + '_>> {
        Box::pin(async move {
            sqlx::query!(
                r#"
            INSERT INTO users (id, first_name, last_name, e_mail, password)
            VALUES ($1, $2, $3, $4, $5)
            "#,
                uuid::Uuid::parse_str(&user_entity.id).unwrap(),
                user_entity.first_name,
                user_entity.last_name,
                user_entity.email,
                user_entity.password
            )
            .execute(&self.database_pool)
            .await
            .map_err(|err| UserDatabaseError::InsertError {
                message: err.to_string(),
            })?;

            Ok(())
        })
    }
}

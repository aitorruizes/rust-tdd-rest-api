use sqlx::types::Uuid;
use time::OffsetDateTime;

use crate::domain::entities::user::user_entity::UserEntity;

#[derive(sqlx::FromRow)]
pub struct UserModel {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<UserModel> for UserEntity {
    fn from(user_model: UserModel) -> Self {
        Self {
            id: user_model.id.to_string(),
            first_name: user_model.first_name,
            last_name: user_model.last_name,
            email: user_model.email,
            password: user_model.password,
            is_admin: user_model.is_admin,
            created_at: user_model.created_at.unix_timestamp(),
            updated_at: user_model.updated_at.unix_timestamp(),
        }
    }
}

impl From<UserEntity> for UserModel {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: Uuid::parse_str(&entity.id).expect("Invalid UUID in UserEntity"),
            first_name: entity.first_name,
            last_name: entity.last_name,
            email: entity.email,
            password: entity.password,
            is_admin: entity.is_admin,
            created_at: OffsetDateTime::from_unix_timestamp(entity.created_at)
                .expect("Invalid timestamp in UserEntity"),
            updated_at: OffsetDateTime::from_unix_timestamp(entity.updated_at)
                .expect("Invalid timestamp in UserEntity"),
        }
    }
}

use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::entities::user::user_entity::UserEntity;

#[derive(Serialize, Debug, Clone)]
pub struct UserResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub is_admin: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<UserEntity> for UserResponse {
    fn from(user_entity: UserEntity) -> Self {
        Self {
            id: Uuid::parse_str(&user_entity.id).expect("Invalid UUID"),
            first_name: user_entity.first_name,
            last_name: user_entity.last_name,
            email: user_entity.email,
            is_admin: user_entity.is_admin,
            created_at: OffsetDateTime::from_unix_timestamp(user_entity.created_at)
                .expect("Invalid created_at timestamp"),
            updated_at: OffsetDateTime::from_unix_timestamp(user_entity.updated_at)
                .expect("Invalid updated_at timestamp"),
        }
    }
}

use std::pin::Pin;

use crate::domain::entities::user::user_entity::UserEntity;

#[derive(Debug, PartialEq)]
pub enum UserDatabaseError {
    InsertError(String),
}

impl std::fmt::Display for UserDatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserDatabaseError::InsertError(msg) => write!(f, "Insert Error: {}", msg),
        }
    }
}

impl std::error::Error for UserDatabaseError {}

pub trait UserDatabasePort: Send + Sync {
    fn insert_user(
        &self,
        user_entity: UserEntity,
    ) -> Pin<Box<dyn Future<Output = Result<UserEntity, UserDatabaseError>> + Send + Sync>>;
}

use std::pin::Pin;

use crate::domain::entities::user::user_entity::UserEntity;

#[derive(Debug, PartialEq)]
pub enum UserDatabaseError {
    InsertError { message: String },
}

impl std::fmt::Display for UserDatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserDatabaseError::InsertError { message } => write!(f, "insert Error: {}", message),
        }
    }
}

impl std::error::Error for UserDatabaseError {}

pub trait UserDatabasePort: UserDatabasePortClone + Send + Sync {
    fn insert_user(
        &self,
        user_entity: UserEntity,
    ) -> Pin<Box<dyn Future<Output = Result<(), UserDatabaseError>> + Send + '_>>;
}

pub trait UserDatabasePortClone {
    fn clone_box(&self) -> Box<dyn UserDatabasePort + Send + Sync>;
}

impl<T> UserDatabasePortClone for T
where
    T: UserDatabasePort + Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn UserDatabasePort + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn UserDatabasePort + Send + Sync> {
    fn clone(&self) -> Box<dyn UserDatabasePort + Send + Sync> {
        self.as_ref().clone_box()
    }
}

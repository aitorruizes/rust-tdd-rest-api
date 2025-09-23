use std::pin::Pin;

use crate::domain::entities::user::user_entity::UserEntity;

#[derive(Debug, PartialEq, Eq)]
pub enum CreateUserRepositoryError {
    InsertError { message: String },
}

impl std::fmt::Display for CreateUserRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsertError { message } => {
                write!(f, "insert error: {message}")
            }
        }
    }
}

impl std::error::Error for CreateUserRepositoryError {}

pub type CreateUserRepositoryFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(), CreateUserRepositoryError>> + Send + 'a>>;

pub trait CreateUserRepositoryPort: Send + Sync {
    fn execute(&self, user_entity: UserEntity) -> CreateUserRepositoryFuture<'_>;
}

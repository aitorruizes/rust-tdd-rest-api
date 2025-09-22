use std::pin::Pin;

use crate::domain::entities::user::user_entity::UserEntity;

#[derive(Debug, PartialEq, Eq)]
pub enum SignUpRepositoryError {
    InsertError { message: String },
}

impl std::fmt::Display for SignUpRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsertError { message } => {
                write!(f, "insert error: {message}")
            }
        }
    }
}

impl std::error::Error for SignUpRepositoryError {}

pub trait SignUpRepositoryPort: Send + Sync {
    fn execute(
        &self,
        user_entity: UserEntity,
    ) -> Pin<Box<dyn Future<Output = Result<(), SignUpRepositoryError>> + Send + '_>>;
}

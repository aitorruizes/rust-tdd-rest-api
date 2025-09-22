use std::pin::Pin;

use crate::domain::entities::user::user_entity::UserEntity;

#[derive(Debug, PartialEq, Eq)]
pub enum GetUserByEmailRepositoryError {
    FindByEmailError { message: String },
}

impl std::fmt::Display for GetUserByEmailRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FindByEmailError { message } => {
                write!(f, "fetch by e-mail error: {message}")
            }
        }
    }
}

impl std::error::Error for GetUserByEmailRepositoryError {}

pub trait GetUserByEmailRepositoryPort: Send + Sync {
    fn execute(
        &self,
        email: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<UserEntity>, GetUserByEmailRepositoryError>> + Send + '_>>;
}

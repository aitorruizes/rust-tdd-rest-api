use std::pin::Pin;

use crate::domain::entities::user::user_entity::UserEntity;

#[derive(Debug, PartialEq, Eq)]
pub enum GetUserByIdRepositoryError {
    FindByIdError { message: String },
}

impl std::fmt::Display for GetUserByIdRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FindByIdError { message } => {
                write!(f, "fetch by id error: {message}")
            }
        }
    }
}

pub type GetUserByIdFUture<'a> =
    Pin<Box<dyn Future<Output = Result<Option<UserEntity>, GetUserByIdRepositoryError>> + 'a>>;

pub trait GetUserByIdRepositoryPort {
    fn execute(&self, id: String) -> GetUserByIdFUture<'_>;
}

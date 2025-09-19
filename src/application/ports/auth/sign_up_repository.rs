use std::pin::Pin;

use crate::domain::entities::user::user_entity::UserEntity;

#[derive(Debug, PartialEq)]
pub enum SignUpRepositoryError {
    InsertError { message: String },
}

impl std::fmt::Display for SignUpRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignUpRepositoryError::InsertError { message } => {
                write!(f, "insert Error: {}", message)
            }
        }
    }
}

impl std::error::Error for SignUpRepositoryError {}

pub trait SignUpRepositoryPort: UserDatabasePortClone + Send + Sync {
    fn execute(
        &self,
        user_entity: UserEntity,
    ) -> Pin<Box<dyn Future<Output = Result<(), SignUpRepositoryError>> + Send + '_>>;
}

pub trait UserDatabasePortClone {
    fn clone_box(&self) -> Box<dyn SignUpRepositoryPort + Send + Sync>;
}

impl<T> UserDatabasePortClone for T
where
    T: SignUpRepositoryPort + Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn SignUpRepositoryPort + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SignUpRepositoryPort + Send + Sync> {
    fn clone(&self) -> Box<dyn SignUpRepositoryPort + Send + Sync> {
        self.as_ref().clone_box()
    }
}

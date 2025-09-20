use std::pin::Pin;

use crate::domain::entities::user::user_entity::UserEntity;

#[derive(Debug, PartialEq)]
pub enum SignInRepositoryError {
    FindByEmailError { message: String },
}

impl std::fmt::Display for SignInRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignInRepositoryError::FindByEmailError { message } => {
                write!(f, "fetch by e-mail error: {}", message)
            }
        }
    }
}

impl std::error::Error for SignInRepositoryError {}

pub trait SignInRepositoryPort: SignInRepositoryPortClone + Send + Sync {
    fn execute(
        &self,
        email: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<UserEntity>, SignInRepositoryError>> + Send + '_>>;
}

pub trait SignInRepositoryPortClone {
    fn clone_box(&self) -> Box<dyn SignInRepositoryPort + Send + Sync>;
}

impl<T> SignInRepositoryPortClone for T
where
    T: SignInRepositoryPort + Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn SignInRepositoryPort + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SignInRepositoryPort + Send + Sync> {
    fn clone(&self) -> Box<dyn SignInRepositoryPort + Send + Sync> {
        self.as_ref().clone_box()
    }
}

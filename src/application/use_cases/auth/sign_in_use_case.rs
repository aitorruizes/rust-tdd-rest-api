use std::pin::Pin;

use crate::{
    application::{
        dtos::auth::sign_in_dto::SignInDto,
        ports::auth::sign_in_repository_port::{SignInRepositoryError, SignInRepositoryPort},
    },
    domain::entities::user::user_entity::UserEntity,
};

#[derive(Debug, PartialEq)]
pub enum SignInUseCaseError {
    DatabaseError(SignInRepositoryError),
}

impl std::fmt::Display for SignInUseCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignInUseCaseError::DatabaseError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for SignInUseCaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SignInUseCaseError::DatabaseError(e) => Some(e),
        }
    }
}

pub trait SignInUseCasePort: SignInUseCasePortClone + Send + Sync {
    fn perform(
        &self,
        sign_in_dto: SignInDto,
    ) -> Pin<Box<dyn Future<Output = Result<Option<UserEntity>, SignInUseCaseError>> + Send + '_>>;
}

pub trait SignInUseCasePortClone {
    fn clone_box(&self) -> Box<dyn SignInUseCasePort + Send + Sync>;
}

impl<T> SignInUseCasePortClone for T
where
    T: SignInUseCasePort + Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn SignInUseCasePort + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SignInUseCasePort + Send + Sync> {
    fn clone(&self) -> Box<dyn SignInUseCasePort + Send + Sync> {
        self.as_ref().clone_box()
    }
}

pub struct SignInUseCase {
    sign_in_repository: Box<dyn SignInRepositoryPort>,
}

impl SignInUseCase {
    pub fn new(sign_in_repository: Box<dyn SignInRepositoryPort>) -> Self {
        Self { sign_in_repository }
    }
}

impl SignInUseCasePort for SignInUseCase {
    fn perform(
        &self,
        sign_in_dto: SignInDto,
    ) -> Pin<Box<dyn Future<Output = Result<Option<UserEntity>, SignInUseCaseError>> + Send + '_>>
    {
        Box::pin(async move {
            let user_entity: Option<UserEntity> = self
                .sign_in_repository
                .execute(sign_in_dto.email)
                .await
                .map_err(SignInUseCaseError::DatabaseError)?;

            Ok(user_entity)
        })
    }
}

impl Clone for SignInUseCase {
    fn clone(&self) -> Self {
        Self {
            sign_in_repository: self.sign_in_repository.clone_box(),
        }
    }
}

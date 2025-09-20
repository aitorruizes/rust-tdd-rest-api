use std::pin::Pin;

use crate::application::{
    dtos::auth::sign_in_dto::SignInDto,
    ports::{
        auth::auth_port::{AuthError, AuthPort},
        repositories::sign_in_repository_port::{SignInRepositoryError, SignInRepositoryPort},
    },
};

#[derive(Debug, PartialEq)]
pub enum SignInUseCaseError {
    AuthError(AuthError),
    DatabaseError(SignInRepositoryError),
}

impl std::fmt::Display for SignInUseCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignInUseCaseError::AuthError(e) => write!(f, "{}", e),
            SignInUseCaseError::DatabaseError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for SignInUseCaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SignInUseCaseError::AuthError(e) => Some(e),
            SignInUseCaseError::DatabaseError(e) => Some(e),
        }
    }
}

pub trait SignInUseCasePort: SignInUseCasePortClone + Send + Sync {
    fn perform(
        &self,
        sign_in_dto: SignInDto,
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>, SignInUseCaseError>> + Send + '_>>;
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
    auth_adapter: Box<dyn AuthPort>,
    sign_in_repository: Box<dyn SignInRepositoryPort>,
}

impl SignInUseCase {
    pub fn new(
        auth_adapter: Box<dyn AuthPort>,
        sign_in_repository: Box<dyn SignInRepositoryPort>,
    ) -> Self {
        Self {
            auth_adapter,
            sign_in_repository,
        }
    }
}

impl SignInUseCasePort for SignInUseCase {
    fn perform(
        &self,
        sign_in_dto: SignInDto,
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>, SignInUseCaseError>> + Send + '_>> {
        Box::pin(async move {
            match self
                .sign_in_repository
                .execute(sign_in_dto.email)
                .await
                .map_err(SignInUseCaseError::DatabaseError)?
            {
                Some(user) => {
                    let generated_auth_token: String = self
                        .auth_adapter
                        .generate_auth_token(user.id)
                        .map_err(SignInUseCaseError::AuthError)?;

                    Ok(Some(generated_auth_token))
                }
                None => Ok(None),
            }
        })
    }
}

impl Clone for SignInUseCase {
    fn clone(&self) -> Self {
        Self {
            auth_adapter: self.auth_adapter.clone_box(),
            sign_in_repository: self.sign_in_repository.clone_box(),
        }
    }
}

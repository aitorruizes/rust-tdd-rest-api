use std::pin::Pin;

use crate::application::{
    dtos::auth::sign_in_dto::SignInDto,
    ports::{
        auth::auth_port::{AuthError, AuthPort},
        hasher::hasher_port::{HasherError, HasherPort},
        repositories::sign_in_repository_port::{SignInRepositoryError, SignInRepositoryPort},
    },
};

#[derive(Debug, PartialEq)]
pub enum SignInUseCaseError {
    HasherError(HasherError),
    AuthError(AuthError),
    DatabaseError(SignInRepositoryError),
}

impl std::fmt::Display for SignInUseCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignInUseCaseError::HasherError(error) => write!(f, "{}", error),
            SignInUseCaseError::AuthError(error) => write!(f, "{}", error),
            SignInUseCaseError::DatabaseError(error) => write!(f, "{}", error),
        }
    }
}

impl std::error::Error for SignInUseCaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SignInUseCaseError::HasherError(error) => Some(error),
            SignInUseCaseError::AuthError(error) => Some(error),
            SignInUseCaseError::DatabaseError(error) => Some(error),
        }
    }
}

pub trait SignInUseCasePort: Send + Sync {
    fn perform(
        &self,
        sign_in_dto: SignInDto,
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>, SignInUseCaseError>> + Send + '_>>;
}

#[derive(Clone)]
pub struct SignInUseCase<HasherAdapter, AuthAdapter, Repository>
where
    HasherAdapter: HasherPort + Send + Sync + Clone + 'static,
    AuthAdapter: AuthPort + Send + Sync + Clone + 'static,
    Repository: SignInRepositoryPort + Send + Sync + Clone + 'static,
{
    hasher_adapter: HasherAdapter,
    auth_adapter: AuthAdapter,
    sign_in_repository: Repository,
}

impl<HasherAdapter, AuthAdapter, Repository> SignInUseCase<HasherAdapter, AuthAdapter, Repository>
where
    HasherAdapter: HasherPort + Send + Sync + Clone + 'static,
    AuthAdapter: AuthPort + Send + Sync + Clone + 'static,
    Repository: SignInRepositoryPort + Send + Sync + Clone + 'static,
{
    pub fn new(
        hasher_adapter: HasherAdapter,
        auth_adapter: AuthAdapter,
        sign_in_repository: Repository,
    ) -> Self {
        Self {
            hasher_adapter,
            auth_adapter,
            sign_in_repository,
        }
    }
}

impl<HasherAdapter, AuthAdapter, Repository> SignInUseCasePort
    for SignInUseCase<HasherAdapter, AuthAdapter, Repository>
where
    HasherAdapter: HasherPort + Send + Sync + Clone + 'static,
    AuthAdapter: AuthPort + Send + Sync + Clone + 'static,
    Repository: SignInRepositoryPort + Send + Sync + Clone + 'static,
{
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
                    let has_password_matched = self
                        .hasher_adapter
                        .verify(&sign_in_dto.password, &user.password)
                        .map_err(SignInUseCaseError::HasherError)?;

                    if !has_password_matched {
                        return Ok(None);
                    }

                    let generated_auth_token = self
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

use std::pin::Pin;

use crate::application::{
    dtos::auth::sign_in_dto::SignInDto,
    ports::{
        auth::auth_port::{AuthError, AuthPort},
        hasher::hasher_port::{HasherError, HasherPort},
        repositories::user::get_user_by_email_repository_port::{
            GetUserByEmailRepositoryError, GetUserByEmailRepositoryPort,
        },
    },
};

#[derive(Debug, PartialEq, Eq)]
pub enum SignInUseCaseError {
    HasherError(HasherError),
    AuthError(AuthError),
    DatabaseError(GetUserByEmailRepositoryError),
}

impl std::fmt::Display for SignInUseCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HasherError(error) => write!(f, "{error}"),
            Self::AuthError(error) => write!(f, "{error}"),
            Self::DatabaseError(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for SignInUseCaseError {}

pub type SignInUseCaseFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Option<String>, SignInUseCaseError>> + Send + 'a>>;

pub trait SignInUseCasePort: Send + Sync {
    fn perform(&self, sign_in_dto: SignInDto) -> SignInUseCaseFuture<'_>;
}

#[derive(Clone)]
pub struct SignInUseCase<HasherAdapter, AuthAdapter, Repository>
where
    HasherAdapter: HasherPort + Send + Sync + Clone + 'static,
    AuthAdapter: AuthPort + Send + Sync + Clone + 'static,
    Repository: GetUserByEmailRepositoryPort + Send + Sync + Clone + 'static,
{
    hasher_adapter: HasherAdapter,
    auth_adapter: AuthAdapter,
    get_user_by_email_repository: Repository,
}

impl<HasherAdapter, AuthAdapter, Repository>
    SignInUseCase<HasherAdapter, AuthAdapter, Repository>
where
    HasherAdapter: HasherPort + Send + Sync + Clone + 'static,
    AuthAdapter: AuthPort + Send + Sync + Clone + 'static,
    Repository: GetUserByEmailRepositoryPort + Send + Sync + Clone + 'static,
{
    pub const fn new(
        hasher_adapter: HasherAdapter,
        auth_adapter: AuthAdapter,
        get_user_by_email_repository: Repository,
    ) -> Self {
        Self {
            hasher_adapter,
            auth_adapter,
            get_user_by_email_repository,
        }
    }
}

impl<HasherAdapter, AuthAdapter, Repository> SignInUseCasePort
    for SignInUseCase<HasherAdapter, AuthAdapter, Repository>
where
    HasherAdapter: HasherPort + Send + Sync + Clone + 'static,
    AuthAdapter: AuthPort + Send + Sync + Clone + 'static,
    Repository: GetUserByEmailRepositoryPort + Send + Sync + Clone + 'static,
{
    fn perform(&self, sign_in_dto: SignInDto) -> SignInUseCaseFuture<'_> {
        Box::pin(async move {
            match self
                .get_user_by_email_repository
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

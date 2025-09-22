use std::pin::Pin;

use crate::{
    application::ports::repositories::user::get_user_by_id_repository_port::{
        GetUserByIdRepositoryError, GetUserByIdRepositoryPort,
    },
    domain::entities::user::user_entity::UserEntity,
};

#[derive(Debug, PartialEq, Eq)]
pub enum GetUserByIdUseCaseError {
    RepositoryError(GetUserByIdRepositoryError),
}

impl std::fmt::Display for GetUserByIdUseCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RepositoryError(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for GetUserByIdUseCaseError {}

pub trait GetUserByIdUseCasePort: Send + Sync {
    fn perform(
        &self,
        id: String,
    ) -> Pin<
        Box<dyn Future<Output = Result<Option<UserEntity>, GetUserByIdUseCaseError>> + Send + '_>,
    >;
}

#[derive(Clone)]
pub struct GetUserByIdUseCase<Repository>
where
    Repository: GetUserByIdRepositoryPort + Send + Sync + Clone + 'static,
{
    get_user_by_id_repository: Repository,
}

impl<Repository> GetUserByIdUseCase<Repository>
where
    Repository: GetUserByIdRepositoryPort + Send + Sync + Clone + 'static,
{
    pub const fn new(get_user_by_id_repository: Repository) -> Self {
        Self {
            get_user_by_id_repository,
        }
    }
}

impl<Repository> GetUserByIdUseCasePort for GetUserByIdUseCase<Repository>
where
    Repository: GetUserByIdRepositoryPort + Send + Sync + Clone + 'static,
{
    fn perform(
        &self,
        id: String,
    ) -> Pin<
        Box<dyn Future<Output = Result<Option<UserEntity>, GetUserByIdUseCaseError>> + Send + '_>,
    > {
        Box::pin(async move {
            let user_entity = self
                .get_user_by_id_repository
                .execute(id)
                .await
                .map_err(GetUserByIdUseCaseError::RepositoryError)?;

            Ok(user_entity)
        })
    }
}

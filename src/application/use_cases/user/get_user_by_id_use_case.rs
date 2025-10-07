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

pub type GetUserByIdUseCaseFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Option<UserEntity>, GetUserByIdUseCaseError>> + Send + 'a>>;

pub trait GetUserByIdUseCasePort: Send + Sync {
    fn perform(&self, id: String) -> GetUserByIdUseCaseFuture<'_>;
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
    fn perform(&self, id: String) -> GetUserByIdUseCaseFuture<'_> {
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

#[cfg(test)]
mod tests {
    use mockall::mock;
    use time::{OffsetDateTime, format_description::well_known::Rfc3339};
    use uuid::Uuid;

    use crate::{
        application::{
            ports::repositories::user::get_user_by_id_repository_port::{
                GetUserByIdFuture, GetUserByIdRepositoryError, GetUserByIdRepositoryPort,
            },
            use_cases::user::get_user_by_id_use_case::{
                GetUserByIdUseCase, GetUserByIdUseCaseError, GetUserByIdUseCasePort,
            },
        },
        domain::entities::user::user_entity::UserEntityBuilder,
    };

    mock! {
        pub GetUserByIdRepository {}

        impl GetUserByIdRepositoryPort for GetUserByIdRepository {
            fn execute(&self, id: String) -> GetUserByIdFuture<'_>;
        }

        impl Clone for GetUserByIdRepository {
            fn clone(&self) -> Self {
                MockCreateUserRepository::new()
            }
        }
    }

    #[tokio::test]
    async fn should_successfully_call_get_user_by_id_repository() {
        let mut get_user_by_id_repository_mock = MockGetUserByIdRepository::default();

        get_user_by_id_repository_mock
            .expect_execute()
            .returning(|_| {
                Box::pin(async move {
                    let user_entity = UserEntityBuilder::default()
                        .id(Uuid::parse_str("dba86129-90be-4409-a5a3-396db9335a57").unwrap())
                        .first_name("John")
                        .last_name("Doe")
                        .email("johndoe@gmail.com")
                        .password("$2b$12$D/HbcVNFxNrOzRmoy4M0nu1ZUzJcTDt5UVUcxEb/vKfRZsTL0ORa.")
                        .is_admin(false)
                        .created_at(
                            OffsetDateTime::parse("2025-09-22T14:57:49.66802Z", &Rfc3339).unwrap(),
                        )
                        .updated_at(
                            OffsetDateTime::parse("2025-09-22T14:57:49.66802Z", &Rfc3339).unwrap(),
                        )
                        .build();

                    Ok(Some(user_entity))
                })
            });

        let get_user_by_id_use_case = GetUserByIdUseCase::new(get_user_by_id_repository_mock);
        let id = "dba86129-90be-4409-a5a3-396db9335a57".to_string();
        let result = get_user_by_id_use_case.perform(id).await;

        assert!(result.is_ok());

        let content = result.unwrap();

        assert!(content.is_some());

        let user = content.unwrap();

        assert_eq!(user.id.to_string(), "dba86129-90be-4409-a5a3-396db9335a57");
        assert_eq!(user.first_name, "John");
        assert_eq!(user.last_name, "Doe");
        assert_eq!(user.email, "johndoe@gmail.com");

        assert_eq!(
            user.password,
            "$2b$12$D/HbcVNFxNrOzRmoy4M0nu1ZUzJcTDt5UVUcxEb/vKfRZsTL0ORa."
        );

        assert!(!user.is_admin);

        assert_eq!(
            user.created_at,
            OffsetDateTime::parse("2025-09-22T14:57:49.66802Z", &Rfc3339).unwrap()
        );

        assert_eq!(
            user.updated_at,
            OffsetDateTime::parse("2025-09-22T14:57:49.66802Z", &Rfc3339).unwrap()
        );
    }

    #[tokio::test]
    async fn should_return_error_if_get_user_by_id_repository_fails() {
        let mut get_user_by_id_repository_mock = MockGetUserByIdRepository::default();

        get_user_by_id_repository_mock
            .expect_execute()
            .returning(|_| {
                Box::pin(async move {
                    Err(GetUserByIdRepositoryError::FindByIdError {
                        message: "find by id error".to_string(),
                    })
                })
            });

        let get_user_by_id_use_case = GetUserByIdUseCase::new(get_user_by_id_repository_mock);
        let id = "dba86129-90be-4409-a5a3-396db9335a57".to_string();
        let result = get_user_by_id_use_case.perform(id).await;

        assert!(result.is_err());

        let error = result.unwrap_err();

        assert!(matches!(
            error,
            GetUserByIdUseCaseError::RepositoryError(GetUserByIdRepositoryError::FindByIdError {
                message: _
            })
        ));
    }
}

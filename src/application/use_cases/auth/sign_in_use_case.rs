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

impl<HasherAdapter, AuthAdapter, Repository> SignInUseCase<HasherAdapter, AuthAdapter, Repository>
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

#[cfg(test)]
mod tests {
    use mockall::mock;
    use time::{OffsetDateTime, format_description::well_known::Rfc3339};
    use uuid::Uuid;

    use crate::{
        application::{
            dtos::auth::sign_in_dto::SignInDto,
            ports::{
                auth::auth_port::{AuthError, AuthPort},
                hasher::hasher_port::{HasherError, HasherPort},
                repositories::user::get_user_by_email_repository_port::{
                    GetUserByEmailRepositoryError, GetUserByEmailRepositoryFuture,
                    GetUserByEmailRepositoryPort,
                },
            },
            use_cases::auth::sign_in_use_case::{
                SignInUseCase, SignInUseCaseError, SignInUseCasePort,
            },
        },
        domain::entities::user::user_entity::UserEntityBuilder,
    };

    mock! {
        pub HasherAdapter {}

        impl HasherPort for HasherAdapter {
            fn hash(&self, password: &str) -> Result<String, HasherError>;
            fn verify(&self, password: &str, password_hash: &str) -> Result<bool, HasherError>;
        }

        impl Clone for HasherAdapter {
            fn clone(&self) -> Self {
                MockHasherAdapter::new()
            }
        }
    }

    mock! {
        pub AuthAdapter {}

        impl AuthPort for AuthAdapter {
            fn generate_auth_token(&self, user_id: Uuid) -> Result<String, AuthError>;
            fn verify_auth_token(&self, token: &str) -> Result<(), AuthError>;
        }

        impl Clone for AuthAdapter {
            fn clone(&self) -> Self {
                MockAuthAdapter::new()
            }
        }
    }

    mock! {
        pub GetUserByEmailRepository {}

        impl GetUserByEmailRepositoryPort for GetUserByEmailRepository {
            fn execute(&self, email: String) -> GetUserByEmailRepositoryFuture<'_>;
        }

        impl Clone for GetUserByEmailRepository {
            fn clone(&self) -> Self {
                MockGetUserByEmailRepository::new()
            }
        }
    }

    #[tokio::test]
    async fn should_successfully_perform_sign_in_use_case() {
        let mut hasher_adapter_mock = MockHasherAdapter::default();

        hasher_adapter_mock
            .expect_verify()
            .returning(|_, _| Ok(true));

        let mut auth_adapter_mock = MockAuthAdapter::default();

        auth_adapter_mock
            .expect_generate_auth_token()
            .returning(|_| Ok("any_token".to_string()));

        let mut get_user_by_email_repository_mock = MockGetUserByEmailRepository::default();

        get_user_by_email_repository_mock
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

        let sign_in_use_case = SignInUseCase::new(
            hasher_adapter_mock,
            auth_adapter_mock,
            get_user_by_email_repository_mock,
        );

        let sign_in_dto =
            SignInDto::new("johndoe@gmail.com".to_string(), "Password123!".to_string());

        let result = sign_in_use_case.perform(sign_in_dto).await;

        assert!(result.is_ok());

        let authorization_token = result.unwrap();

        assert!(authorization_token.is_some());
    }

    #[tokio::test]
    async fn should_return_none_if_passwords_do_not_match() {
        let mut hasher_adapter_mock = MockHasherAdapter::default();

        hasher_adapter_mock
            .expect_verify()
            .returning(|_, _| Ok(false));

        let auth_adapter_mock = MockAuthAdapter::default();

        let mut get_user_by_email_repository_mock = MockGetUserByEmailRepository::default();

        get_user_by_email_repository_mock
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

        let sign_in_use_case = SignInUseCase::new(
            hasher_adapter_mock,
            auth_adapter_mock,
            get_user_by_email_repository_mock,
        );

        let sign_in_dto =
            SignInDto::new("johndoe@gmail.com".to_string(), "Password123!".to_string());

        let result = sign_in_use_case.perform(sign_in_dto).await;

        assert!(result.is_ok());

        let authorization_token = result.unwrap();

        assert!(authorization_token.is_none());
    }

    #[tokio::test]
    async fn should_return_none_if_no_user_is_found() {
        let hasher_adapter_mock = MockHasherAdapter::default();
        let auth_adapter_mock = MockAuthAdapter::default();

        let mut get_user_by_email_repository_mock = MockGetUserByEmailRepository::default();

        get_user_by_email_repository_mock
            .expect_execute()
            .returning(|_| Box::pin(async move { Ok(None) }));

        let sign_in_use_case = SignInUseCase::new(
            hasher_adapter_mock,
            auth_adapter_mock,
            get_user_by_email_repository_mock,
        );

        let sign_in_dto =
            SignInDto::new("johndoe@gmail.com".to_string(), "Password123!".to_string());

        let result = sign_in_use_case.perform(sign_in_dto).await;

        assert!(result.is_ok());

        let authorization_token = result.unwrap();

        assert!(authorization_token.is_none());
    }

    #[tokio::test]
    async fn should_return_error_if_hasher_verify_fails() {
        let mut hasher_adapter_mock = MockHasherAdapter::default();

        hasher_adapter_mock.expect_verify().returning(|_, _| {
            Err(HasherError::VerificationError {
                message: "verify fails".to_string(),
            })
        });

        let auth_adapter_mock = MockAuthAdapter::default();

        let mut get_user_by_email_repository_mock = MockGetUserByEmailRepository::default();

        get_user_by_email_repository_mock
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

        let sign_in_use_case = SignInUseCase::new(
            hasher_adapter_mock,
            auth_adapter_mock,
            get_user_by_email_repository_mock,
        );

        let sign_in_dto =
            SignInDto::new("johndoe@gmail.com".to_string(), "Password123!".to_string());

        let result = sign_in_use_case.perform(sign_in_dto).await;

        assert!(result.is_err());

        let error = result.unwrap_err();

        assert!(matches!(
            error,
            SignInUseCaseError::HasherError(HasherError::VerificationError { message: _ })
        ));
    }

    #[tokio::test]
    async fn should_return_error_if_get_user_by_email_fails() {
        let hasher_adapter_mock = MockHasherAdapter::default();

        let auth_adapter_mock = MockAuthAdapter::default();

        let mut get_user_by_email_repository_mock = MockGetUserByEmailRepository::default();

        get_user_by_email_repository_mock
            .expect_execute()
            .returning(|_| {
                Box::pin(async move {
                    Err(GetUserByEmailRepositoryError::FindByEmailError {
                        message: "get user by e-mail fail".to_string(),
                    })
                })
            });

        let sign_in_use_case = SignInUseCase::new(
            hasher_adapter_mock,
            auth_adapter_mock,
            get_user_by_email_repository_mock,
        );

        let sign_in_dto =
            SignInDto::new("johndoe@gmail.com".to_string(), "Password123!".to_string());

        let result = sign_in_use_case.perform(sign_in_dto).await;

        assert!(result.is_err());

        let error = result.unwrap_err();

        assert!(matches!(
            error,
            SignInUseCaseError::DatabaseError(GetUserByEmailRepositoryError::FindByEmailError {
                message: _
            })
        ));
    }

    #[tokio::test]
    async fn should_return_error_if_authentication_token_generation_fails() {
        let mut hasher_adapter_mock = MockHasherAdapter::default();

        hasher_adapter_mock
            .expect_verify()
            .returning(|_, _| Ok(true));

        let mut auth_adapter_mock = MockAuthAdapter::default();

        auth_adapter_mock
            .expect_generate_auth_token()
            .returning(|_| {
                Err(AuthError::GenerateTokenError {
                    message: "token generation fails".to_string(),
                })
            });

        let mut get_user_by_email_repository_mock = MockGetUserByEmailRepository::default();

        get_user_by_email_repository_mock
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

        let sign_in_use_case = SignInUseCase::new(
            hasher_adapter_mock,
            auth_adapter_mock,
            get_user_by_email_repository_mock,
        );

        let sign_in_dto =
            SignInDto::new("johndoe@gmail.com".to_string(), "Password123!".to_string());

        let result = sign_in_use_case.perform(sign_in_dto).await;

        assert!(result.is_err());

        let error = result.unwrap_err();

        assert!(matches!(
            error,
            SignInUseCaseError::AuthError(AuthError::GenerateTokenError { message: _ })
        ));
    }
}

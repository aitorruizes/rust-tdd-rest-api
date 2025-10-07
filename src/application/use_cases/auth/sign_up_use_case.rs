use std::pin::Pin;

use crate::{
    application::{
        dtos::auth::sign_up_dto::SignUpDto,
        ports::{
            hasher::hasher_port::{HasherError, HasherPort},
            id_generator::id_generator_port::IdGeneratorPort,
            repositories::user::create_user_repository_port::{
                CreateUserRepositoryError, CreateUserRepositoryPort,
            },
        },
    },
    domain::{
        entities::user::user_entity::{UserEntity, UserEntityBuilder},
        errors::user::user_errors::UserError,
    },
};

#[derive(Debug, PartialEq, Eq)]
pub enum SignUpUseCaseError {
    HasherError(HasherError),
    UserError(UserError),
    RepositoryError(CreateUserRepositoryError),
}

impl std::fmt::Display for SignUpUseCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HasherError(error) => write!(f, "{error}"),
            Self::UserError(error) => write!(f, "{error}"),
            Self::RepositoryError(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for SignUpUseCaseError {}

pub type SignUpUseCaseFuture<'a> =
    Pin<Box<dyn Future<Output = Result<UserEntity, SignUpUseCaseError>> + Send + 'a>>;

pub trait SignUpUseCasePort: Send + Sync {
    fn perform(&self, sign_up_dto: SignUpDto) -> SignUpUseCaseFuture<'_>;
}

#[derive(Clone)]
pub struct SignUpUseCase<HasherAdapter, IdGeneratorAdapter, Repository>
where
    HasherAdapter: HasherPort + Send + Sync + Clone + 'static,
    IdGeneratorAdapter: IdGeneratorPort + Send + Sync + Clone + 'static,
    Repository: CreateUserRepositoryPort + Send + Sync + Clone + 'static,
{
    hasher_adapter: HasherAdapter,
    id_generator_adapter: IdGeneratorAdapter,
    sign_up_repository: Repository,
}

impl<HasherAdapter, IdGeneratorAdapter, Repository>
    SignUpUseCase<HasherAdapter, IdGeneratorAdapter, Repository>
where
    HasherAdapter: HasherPort + Send + Sync + Clone + 'static,
    IdGeneratorAdapter: IdGeneratorPort + Send + Sync + Clone + 'static,
    Repository: CreateUserRepositoryPort + Send + Sync + Clone + 'static,
{
    pub const fn new(
        hasher_adapter: HasherAdapter,
        id_generator_adapter: IdGeneratorAdapter,
        sign_up_repository: Repository,
    ) -> Self {
        Self {
            hasher_adapter,
            id_generator_adapter,
            sign_up_repository,
        }
    }
}

impl<HasherAdapter, IdGeneratorAdapter, Repository> SignUpUseCasePort
    for SignUpUseCase<HasherAdapter, IdGeneratorAdapter, Repository>
where
    HasherAdapter: HasherPort + Send + Sync + Clone + 'static,
    IdGeneratorAdapter: IdGeneratorPort + Send + Sync + Clone + 'static,
    Repository: CreateUserRepositoryPort + Send + Sync + Clone + 'static,
{
    fn perform(&self, sign_up_dto: SignUpDto) -> SignUpUseCaseFuture<'_> {
        Box::pin(async move {
            if sign_up_dto.password != sign_up_dto.password_confirmation {
                return Err(SignUpUseCaseError::UserError(
                    UserError::PasswordsDoNotMatch,
                ));
            }

            let hashed_password = self
                .hasher_adapter
                .hash(sign_up_dto.password.as_str())
                .map_err(SignUpUseCaseError::HasherError)?;

            let generated_id = self.id_generator_adapter.generate_id();

            let user_entity = UserEntityBuilder::default()
                .id(generated_id)
                .first_name(sign_up_dto.first_name)
                .last_name(sign_up_dto.last_name)
                .email(sign_up_dto.email)
                .password(hashed_password)
                .build();

            let created_user = self
                .sign_up_repository
                .execute(user_entity)
                .await
                .map_err(SignUpUseCaseError::RepositoryError)?;

            Ok(created_user)
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
            dtos::auth::sign_up_dto::SignUpDto,
            ports::{
                hasher::hasher_port::{HasherError, HasherPort},
                id_generator::id_generator_port::IdGeneratorPort,
                repositories::user::create_user_repository_port::{
                    CreateUserRepositoryError, CreateUserRepositoryFuture, CreateUserRepositoryPort,
                },
            },
            use_cases::auth::sign_up_use_case::{
                SignUpUseCase, SignUpUseCaseError, SignUpUseCasePort,
            },
        },
        domain::{
            entities::user::user_entity::{UserEntity, UserEntityBuilder},
            errors::user::user_errors::UserError,
        },
    };

    mock! {
        pub CreateUserRepository {}

        impl CreateUserRepositoryPort for CreateUserRepository {
            fn execute(
                &self,
                user_entity: UserEntity,
            ) -> CreateUserRepositoryFuture<'_>;
        }

        impl Clone for CreateUserRepository {
            fn clone(&self) -> Self {
                MockCreateUserRepository::new()
            }
        }
    }

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
        pub IdGeneratorAdapter {}

        impl IdGeneratorPort for IdGeneratorAdapter {
            fn generate_id(&self) -> Uuid;
        }

        impl Clone for IdGeneratorAdapter {
            fn clone(&self) -> Self {
                MockIdGeneratorAdapter::new()
            }
        }
    }

    #[tokio::test]
    async fn should_succecssfully_execute_sign_up_repository() {
        let mut create_user_repository_mock = MockCreateUserRepository::default();

        create_user_repository_mock
            .expect_execute()
            .times(1)
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

                    Ok(user_entity)
                })
            });

        let mut hasher_adapter_mock = MockHasherAdapter::default();

        hasher_adapter_mock
            .expect_hash()
            .times(1)
            .returning(|_| Ok("hashed_password".to_string()));

        let mut id_generator_adapter_mock = MockIdGeneratorAdapter::default();

        id_generator_adapter_mock
            .expect_generate_id()
            .times(1)
            .returning(|| Uuid::parse_str("d836bc7f-014e-4818-a97f-dd1bb1987b66").unwrap());

        let sign_up_use_case = SignUpUseCase::new(
            hasher_adapter_mock,
            id_generator_adapter_mock,
            create_user_repository_mock,
        );

        let sign_up_dto = SignUpDto::new(
            "John".to_string(),
            "Doe".to_string(),
            "johndoe@gmail.com".to_string(),
            "Password123!".to_string(),
            "Password123!".to_string(),
        );

        let result = sign_up_use_case.perform(sign_up_dto).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_return_error_if_sign_up_repository_fails() {
        let mut create_user_repository_mock = MockCreateUserRepository::default();

        create_user_repository_mock
            .expect_execute()
            .times(1)
            .returning(|_| {
                Box::pin(async move {
                    Err(CreateUserRepositoryError::InsertError {
                        message: "database error".to_string(),
                    })
                })
            });

        let mut hasher_adapter_mock = MockHasherAdapter::default();

        hasher_adapter_mock
            .expect_hash()
            .times(1)
            .returning(|_| Ok("hashed_password".to_string()));

        let mut id_generator_adapter_mock = MockIdGeneratorAdapter::default();

        id_generator_adapter_mock
            .expect_generate_id()
            .times(1)
            .returning(|| Uuid::parse_str("d836bc7f-014e-4818-a97f-dd1bb1987b66").unwrap());

        let sign_up_use_case = SignUpUseCase::new(
            hasher_adapter_mock,
            id_generator_adapter_mock,
            create_user_repository_mock,
        );

        let sign_up_dto = SignUpDto::new(
            "John".to_string(),
            "Doe".to_string(),
            "johndoe@gmail.com".to_string(),
            "Password123!".to_string(),
            "Password123!".to_string(),
        );

        let result = sign_up_use_case.perform(sign_up_dto).await;

        assert!(result.is_err());

        let error = result.unwrap_err();

        assert!(matches!(
            error,
            SignUpUseCaseError::RepositoryError(CreateUserRepositoryError::InsertError {
                message: _
            })
        ));
    }

    #[tokio::test]
    async fn should_return_error_if_password_hash_fails() {
        let create_user_repository_mock = MockCreateUserRepository::default();
        let mut hasher_adapter_mock = MockHasherAdapter::default();

        hasher_adapter_mock.expect_hash().times(1).returning(|_| {
            Err(HasherError::HashingError {
                message: "hashing error".to_string(),
            })
        });

        let id_generator_adapter_mock = MockIdGeneratorAdapter::default();

        let sign_up_use_case = SignUpUseCase::new(
            hasher_adapter_mock,
            id_generator_adapter_mock,
            create_user_repository_mock,
        );

        let sign_up_dto = SignUpDto::new(
            "John".to_string(),
            "Doe".to_string(),
            "johndoe@gmail.com".to_string(),
            "Password123!".to_string(),
            "Password123!".to_string(),
        );

        let result = sign_up_use_case.perform(sign_up_dto).await;

        assert!(result.is_err());

        let error = result.unwrap_err();

        assert!(matches!(
            error,
            SignUpUseCaseError::HasherError(HasherError::HashingError { message: _ })
        ));
    }

    #[tokio::test]
    async fn should_return_error_if_passwords_do_not_match() {
        let create_user_repository_mock = MockCreateUserRepository::default();
        let hasher_adapter_mock = MockHasherAdapter::default();
        let id_generator_adapter_mock = MockIdGeneratorAdapter::default();

        let sign_up_use_case = SignUpUseCase::new(
            hasher_adapter_mock,
            id_generator_adapter_mock,
            create_user_repository_mock,
        );

        let sign_up_dto = SignUpDto::new(
            "John".to_string(),
            "Doe".to_string(),
            "johndoe@gmail.com".to_string(),
            "Password123!".to_string(),
            "Password1234!".to_string(),
        );

        let result = sign_up_use_case.perform(sign_up_dto).await;

        assert!(result.is_err());

        let error = result.unwrap_err();

        assert!(matches!(
            error,
            SignUpUseCaseError::UserError(UserError::PasswordsDoNotMatch)
        ));
    }
}

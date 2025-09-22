use std::pin::Pin;

use crate::{
    application::{
        dtos::auth::sign_up_dto::SignUpDto,
        ports::{
            hasher::hasher_port::{HasherError, HasherPort},
            id_generator::id_generator_port::IdGeneratorPort,
            repositories::auth::sign_up_repository_port::{
                SignUpRepositoryError, SignUpRepositoryPort,
            },
        },
    },
    domain::{
        entities::user::user_entity::UserEntityBuilder, errors::user::user_errors::UserError,
    },
};

#[derive(Debug, PartialEq, Eq)]
pub enum SignUpUseCaseError {
    HasherError(HasherError),
    UserError(UserError),
    RepositoryError(SignUpRepositoryError),
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

pub trait SignUpUseCasePort: Send + Sync {
    fn perform(
        &self,
        sign_up_dto: SignUpDto,
    ) -> Pin<Box<dyn Future<Output = Result<(), SignUpUseCaseError>> + Send + '_>>;
}

#[derive(Clone)]
pub struct SignUpUseCase<HasherAdapter, IdGeneratorAdapter, Repository>
where
    HasherAdapter: HasherPort + Send + Sync + Clone + 'static,
    IdGeneratorAdapter: IdGeneratorPort + Send + Sync + Clone + 'static,
    Repository: SignUpRepositoryPort + Send + Sync + Clone + 'static,
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
    Repository: SignUpRepositoryPort + Send + Sync + Clone + 'static,
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
    Repository: SignUpRepositoryPort + Send + Sync + Clone + 'static,
{
    fn perform(
        &self,
        sign_up_dto: SignUpDto,
    ) -> Pin<Box<dyn Future<Output = Result<(), SignUpUseCaseError>> + Send + '_>> {
        Box::pin(async move {
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

            self.sign_up_repository
                .execute(user_entity)
                .await
                .map_err(SignUpUseCaseError::RepositoryError)?;

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;

    use mockall::mock;
    use uuid::Uuid;

    use crate::{
        application::{
            dtos::auth::sign_up_dto::SignUpDto,
            ports::{
                hasher::hasher_port::{HasherError, HasherPort},
                id_generator::id_generator_port::IdGeneratorPort,
                repositories::auth::sign_up_repository_port::{
                    SignUpRepositoryError, SignUpRepositoryPort,
                },
            },
            use_cases::auth::sign_up_use_case::{
                SignUpUseCase, SignUpUseCaseError, SignUpUseCasePort,
            },
        },
        domain::{entities::user::user_entity::UserEntity, errors::user::user_errors::UserError},
    };

    mock! {
        pub SignUpRepository {}

        impl SignUpRepositoryPort for SignUpRepository {
            fn execute(
                &self,
                user_entity: UserEntity,
            ) -> Pin<Box<dyn Future<Output = Result<(), SignUpRepositoryError>> + Send + 'static>>;
        }

        impl Clone for SignUpRepository {
            fn clone(&self) -> Self {
                MockSignUpRepository::new()
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
        let mut sign_up_repository_mock = MockSignUpRepository::default();

        sign_up_repository_mock
            .expect_execute()
            .times(1)
            .returning(|_| Box::pin(async move { Ok(()) }));

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
            sign_up_repository_mock,
        );

        let sign_up_dto = SignUpDto::new(
            "John".to_string(),
            "Doe".to_string(),
            "johndoe@gmail.com".to_string(),
            "Password123!".to_string(),
        );

        let result = sign_up_use_case.perform(sign_up_dto).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_return_error_if_sign_up_repository_fails() {
        let mut sign_up_repository_mock = MockSignUpRepository::default();

        sign_up_repository_mock
            .expect_execute()
            .times(1)
            .returning(|_| {
                Box::pin(async move {
                    Err(SignUpRepositoryError::InsertError {
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
            sign_up_repository_mock,
        );

        let sign_up_dto = SignUpDto::new(
            "John".to_string(),
            "Doe".to_string(),
            "johndoe@gmail.com".to_string(),
            "Password123!".to_string(),
        );

        let result = sign_up_use_case.perform(sign_up_dto).await;

        assert!(result.is_err());

        let error = result.unwrap_err();
        let formatted_error = format!("{}", error);

        assert_eq!(formatted_error, "insert error: database error");
    }

    #[tokio::test]
    async fn should_return_error_if_password_hash_fails() {
        let sign_up_repository_mock = MockSignUpRepository::default();
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
            sign_up_repository_mock,
        );

        let sign_up_dto = SignUpDto::new(
            "John".to_string(),
            "Doe".to_string(),
            "johndoe@gmail.com".to_string(),
            "Password123!".to_string(),
        );

        let result = sign_up_use_case.perform(sign_up_dto).await;

        assert!(result.is_err());

        let error = result.unwrap_err();

        assert_eq!(
            error.to_string(),
            "an error occurred while hashing password: hashing error"
        );
    }

    #[test]
    fn test_signup_use_case_error_display() {
        let first_error = SignUpUseCaseError::HasherError(HasherError::HashingError {
            message: "hasher error".to_string(),
        });

        let second_error = SignUpUseCaseError::HasherError(HasherError::VerificationError {
            message: "verification error".to_string(),
        });

        let third_error = SignUpUseCaseError::UserError(UserError::PasswordsDoNotMatch);

        let fourth_error =
            SignUpUseCaseError::RepositoryError(SignUpRepositoryError::InsertError {
                message: "repository error".to_string(),
            });

        assert_eq!(
            first_error.to_string(),
            "an error occurred while hashing password: hasher error"
        );
        assert_eq!(
            second_error.to_string(),
            "an error occurred while verifying password: verification error"
        );

        assert_eq!(
            third_error.to_string(),
            "the provided passwords do not match"
        );

        assert_eq!(fourth_error.to_string(), "insert error: repository error");
    }
}

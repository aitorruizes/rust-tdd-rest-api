use std::pin::Pin;

use crate::{
    application::ports::{
        auth::sign_up_repository::{SignUpRepositoryError, SignUpRepositoryPort},
        hasher::hasher_port::{HasherError, HasherPort},
        id_generator::id_generator_port::IdGeneratorPort,
    },
    domain::{
        entities::user::user_entity::UserEntityBuilder, errors::user::user_errors::UserError,
    },
    presentation::dtos::user::create_user_dto::CreateUserDto,
};

#[derive(Debug, PartialEq)]
pub enum SignUpUseCaseError {
    HasherError(HasherError),
    UserError(UserError),
    DatabaseError(SignUpRepositoryError),
}

impl std::fmt::Display for SignUpUseCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignUpUseCaseError::HasherError(e) => write!(f, "{}", e),
            SignUpUseCaseError::UserError(e) => write!(f, "{}", e),
            SignUpUseCaseError::DatabaseError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for SignUpUseCaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SignUpUseCaseError::HasherError(e) => Some(e),
            SignUpUseCaseError::UserError(e) => Some(e),
            SignUpUseCaseError::DatabaseError(e) => Some(e),
        }
    }
}

pub trait SignUpUseCasePort: SignUpUseCasePortClone + Send + Sync {
    fn perform(
        &self,
        create_user_dto: CreateUserDto,
    ) -> Pin<Box<dyn Future<Output = Result<(), SignUpUseCaseError>> + Send + '_>>;
}

pub trait SignUpUseCasePortClone {
    fn clone_box(&self) -> Box<dyn SignUpUseCasePort + Send + Sync>;
}

impl<T> SignUpUseCasePortClone for T
where
    T: SignUpUseCasePort + Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn SignUpUseCasePort + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SignUpUseCasePort + Send + Sync> {
    fn clone(&self) -> Box<dyn SignUpUseCasePort + Send + Sync> {
        self.as_ref().clone_box()
    }
}

pub struct SignUpUseCase {
    hasher_adapter: Box<dyn HasherPort>,
    id_generator_adapter: Box<dyn IdGeneratorPort>,
    sign_up_repository: Box<dyn SignUpRepositoryPort>,
}

impl SignUpUseCase {
    pub fn new(
        hasher_adapter: Box<dyn HasherPort>,
        id_generator_adapter: Box<dyn IdGeneratorPort>,
        sign_up_repository: Box<dyn SignUpRepositoryPort>,
    ) -> Self {
        Self {
            hasher_adapter,
            id_generator_adapter,
            sign_up_repository,
        }
    }
}

impl SignUpUseCasePort for SignUpUseCase {
    fn perform(
        &self,
        create_user_dto: CreateUserDto,
    ) -> Pin<Box<dyn Future<Output = Result<(), SignUpUseCaseError>> + Send + '_>> {
        Box::pin(async move {
            if create_user_dto.password != create_user_dto.password_confirmation {
                return Err(SignUpUseCaseError::UserError(
                    UserError::PasswordsDoNotMatch,
                ));
            }

            let hashed_password = self
                .hasher_adapter
                .hash(create_user_dto.password.as_str())
                .map_err(SignUpUseCaseError::HasherError)?;

            let generated_id: String = self.id_generator_adapter.generate_id();

            let user_entity = UserEntityBuilder::default()
                .id(generated_id)
                .first_name(create_user_dto.first_name)
                .last_name(create_user_dto.last_name)
                .email(create_user_dto.email)
                .password(hashed_password)
                .build();

            self.sign_up_repository
                .as_ref()
                .execute(user_entity)
                .await
                .map_err(SignUpUseCaseError::DatabaseError)?;

            Ok(())
        })
    }
}

impl Clone for SignUpUseCase {
    fn clone(&self) -> Self {
        Self {
            hasher_adapter: self.hasher_adapter.clone_box(),
            id_generator_adapter: self.id_generator_adapter.clone_box(),
            sign_up_repository: self.sign_up_repository.clone_box(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;

    use mockall::mock;

    use crate::{
        application::{
            ports::{
                auth::sign_up_repository::{SignUpRepositoryError, SignUpRepositoryPort},
                hasher::hasher_port::{HasherError, HasherPort},
                id_generator::id_generator_port::IdGeneratorPort,
            },
            use_cases::auth::sign_up_use_case::{
                SignUpUseCase, SignUpUseCaseError, SignUpUseCasePort,
            },
        },
        domain::{entities::user::user_entity::UserEntity, errors::user::user_errors::UserError},
        presentation::dtos::user::create_user_dto::CreateUserDto,
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
            fn generate_id(&self) -> String;
        }

        impl Clone for IdGeneratorAdapter {
            fn clone(&self) -> Self {
                MockIdGeneratorAdapter::new()
            }
        }
    }

    #[tokio::test]
    async fn should_succecssfully_execute_create_user_repository() {
        let mut create_user_repository_mock: MockSignUpRepository = MockSignUpRepository::default();

        create_user_repository_mock
            .expect_execute()
            .times(1)
            .returning(|_| Box::pin(async move { Ok(()) }));

        let mut hasher_adapter_mock: MockHasherAdapter = MockHasherAdapter::default();

        hasher_adapter_mock
            .expect_hash()
            .times(1)
            .returning(|_| Ok("hashed_password".to_string()));

        let mut id_generator_adapter_mock: MockIdGeneratorAdapter =
            MockIdGeneratorAdapter::default();

        id_generator_adapter_mock
            .expect_generate_id()
            .times(1)
            .returning(|| "generated_id".to_string());

        let create_user_use_case: SignUpUseCase = SignUpUseCase::new(
            Box::new(hasher_adapter_mock),
            Box::new(id_generator_adapter_mock),
            Box::new(create_user_repository_mock),
        );

        let create_user_dto: CreateUserDto = CreateUserDto::new(
            "John".to_string(),
            "Doe".to_string(),
            "johndoe@gmail.com".to_string(),
            "Password123!".to_string(),
            "Password123!".to_string(),
        );

        let result: Result<(), SignUpUseCaseError> =
            create_user_use_case.perform(create_user_dto).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_return_error_if_create_user_repository_fails() {
        let mut create_user_repository_mock: MockSignUpRepository = MockSignUpRepository::default();

        create_user_repository_mock
            .expect_execute()
            .times(1)
            .returning(|_| {
                Box::pin(async move {
                    Err(SignUpRepositoryError::InsertError {
                        message: "Database error".to_string(),
                    })
                })
            });

        let mut hasher_adapter_mock: MockHasherAdapter = MockHasherAdapter::default();

        hasher_adapter_mock
            .expect_hash()
            .times(1)
            .returning(|_| Ok("hashed_password".to_string()));

        let mut id_generator_adapter_mock: MockIdGeneratorAdapter =
            MockIdGeneratorAdapter::default();

        id_generator_adapter_mock
            .expect_generate_id()
            .times(1)
            .returning(|| "generated_id".to_string());

        let create_user_use_case: SignUpUseCase = SignUpUseCase::new(
            Box::new(hasher_adapter_mock),
            Box::new(id_generator_adapter_mock),
            Box::new(create_user_repository_mock),
        );

        let create_user_dto: CreateUserDto = CreateUserDto::new(
            "John".to_string(),
            "Doe".to_string(),
            "johndoe@gmail.com".to_string(),
            "Password123!".to_string(),
            "Password123!".to_string(),
        );

        let result: Result<(), SignUpUseCaseError> =
            create_user_use_case.perform(create_user_dto).await;

        assert!(result.is_err());

        let error: SignUpUseCaseError = result.unwrap_err();

        assert_eq!(
            error,
            SignUpUseCaseError::DatabaseError(SignUpRepositoryError::InsertError {
                message: "Database error".to_string()
            })
        );
    }

    #[tokio::test]
    async fn should_return_error_if_passwords_do_not_match() {
        let create_user_repository_mock: MockSignUpRepository = MockSignUpRepository::default();

        let hasher_adapter_mock: MockHasherAdapter = MockHasherAdapter::default();

        let id_generator_adapter_mock: MockIdGeneratorAdapter = MockIdGeneratorAdapter::default();

        let create_user_use_case: SignUpUseCase = SignUpUseCase::new(
            Box::new(hasher_adapter_mock),
            Box::new(id_generator_adapter_mock),
            Box::new(create_user_repository_mock),
        );

        let create_user_dto: CreateUserDto = CreateUserDto::new(
            "John".to_string(),
            "Doe".to_string(),
            "johndoe@gmail.com".to_string(),
            "Password123!".to_string(),
            "Password1234!".to_string(),
        );

        let result: Result<(), SignUpUseCaseError> =
            create_user_use_case.perform(create_user_dto).await;

        assert!(result.is_err());

        let error: SignUpUseCaseError = result.unwrap_err();

        assert_eq!(
            error,
            SignUpUseCaseError::UserError(UserError::PasswordsDoNotMatch)
        );
    }

    #[tokio::test]
    async fn should_return_error_if_password_hash_fails() {
        let create_user_repository_mock: MockSignUpRepository = MockSignUpRepository::default();

        let mut hasher_adapter_mock: MockHasherAdapter = MockHasherAdapter::default();

        hasher_adapter_mock.expect_hash().times(1).returning(|_| {
            Err(HasherError::HashingError {
                message: "Hashing error".to_string(),
            })
        });

        let id_generator_adapter_mock: MockIdGeneratorAdapter = MockIdGeneratorAdapter::default();

        let create_user_use_case: SignUpUseCase = SignUpUseCase::new(
            Box::new(hasher_adapter_mock),
            Box::new(id_generator_adapter_mock),
            Box::new(create_user_repository_mock),
        );

        let create_user_dto: CreateUserDto = CreateUserDto::new(
            "John".to_string(),
            "Doe".to_string(),
            "johndoe@gmail.com".to_string(),
            "Password123!".to_string(),
            "Password123!".to_string(),
        );

        let result: Result<(), SignUpUseCaseError> =
            create_user_use_case.perform(create_user_dto).await;

        assert!(result.is_err());

        let error: SignUpUseCaseError = result.unwrap_err();

        assert_eq!(
            error,
            SignUpUseCaseError::HasherError(HasherError::HashingError {
                message: "Hashing error".to_string()
            })
        );
    }
}

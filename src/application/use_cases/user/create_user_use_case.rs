use std::pin::Pin;

use crate::{
    application::ports::{
        database::user_database_port::UserDatabaseError,
        hasher::hasher_port::{HasherError, HasherPort},
        id_generator::id_generator_port::IdGeneratorPort,
    },
    domain::{
        entities::user::user_entity::UserEntityBuilder, errors::user::user_errors::UserError,
    },
    infrastructure::repositories::user::create_user_repository::CreateUserRepositoryPort,
    presentation::dtos::user::create_user_dto::CreateUserDto,
};

#[derive(Debug, PartialEq)]
pub enum CreateUserUseCaseError {
    HasherError(HasherError),
    UserError(UserError),
    DatabaseError(UserDatabaseError),
}

impl std::fmt::Display for CreateUserUseCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateUserUseCaseError::HasherError(e) => write!(f, "{}", e),
            CreateUserUseCaseError::UserError(e) => write!(f, "{}", e),
            CreateUserUseCaseError::DatabaseError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for CreateUserUseCaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CreateUserUseCaseError::HasherError(e) => Some(e),
            CreateUserUseCaseError::UserError(e) => Some(e),
            CreateUserUseCaseError::DatabaseError(e) => Some(e),
        }
    }
}

pub trait CreateUserUseCasePort: CreateUserUseCasePortClone + Send + Sync {
    fn perform(
        &self,
        create_user_dto: CreateUserDto,
    ) -> Pin<Box<dyn Future<Output = Result<(), CreateUserUseCaseError>> + Send + '_>>;
}

pub trait CreateUserUseCasePortClone {
    fn clone_box(&self) -> Box<dyn CreateUserUseCasePort + Send + Sync>;
}

impl<T> CreateUserUseCasePortClone for T
where
    T: CreateUserUseCasePort + Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn CreateUserUseCasePort + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CreateUserUseCasePort + Send + Sync> {
    fn clone(&self) -> Box<dyn CreateUserUseCasePort + Send + Sync> {
        self.as_ref().clone_box()
    }
}

pub struct CreateUserUseCase {
    hasher_adapter: Box<dyn HasherPort>,
    id_generator_adapter: Box<dyn IdGeneratorPort>,
    create_user_repository: Box<dyn CreateUserRepositoryPort>,
}

impl CreateUserUseCase {
    pub fn new(
        hasher_adapter: Box<dyn HasherPort>,
        id_generator_adapter: Box<dyn IdGeneratorPort>,
        create_user_repository: Box<dyn CreateUserRepositoryPort>,
    ) -> Self {
        Self {
            hasher_adapter,
            id_generator_adapter,
            create_user_repository,
        }
    }
}

impl CreateUserUseCasePort for CreateUserUseCase {
    fn perform(
        &self,
        create_user_dto: CreateUserDto,
    ) -> Pin<Box<dyn Future<Output = Result<(), CreateUserUseCaseError>> + Send + '_>> {
        Box::pin(async move {
            if create_user_dto.password != create_user_dto.password_confirmation {
                return Err(CreateUserUseCaseError::UserError(
                    UserError::PasswordsDoNotMatch,
                ));
            }

            let hashed_password = self
                .hasher_adapter
                .hash(create_user_dto.password.as_str())
                .map_err(CreateUserUseCaseError::HasherError)?;

            let generated_id: String = self.id_generator_adapter.generate_id();

            let user_entity = UserEntityBuilder::default()
                .id(generated_id)
                .first_name(create_user_dto.first_name)
                .last_name(create_user_dto.last_name)
                .email(create_user_dto.email)
                .password(hashed_password)
                .build();

            self.create_user_repository
                .as_ref()
                .execute(user_entity)
                .await
                .map_err(CreateUserUseCaseError::DatabaseError)?;

            Ok(())
        })
    }
}

impl Clone for CreateUserUseCase {
    fn clone(&self) -> Self {
        Self {
            hasher_adapter: self.hasher_adapter.clone_box(),
            id_generator_adapter: self.id_generator_adapter.clone_box(),
            create_user_repository: self.create_user_repository.clone_box(),
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
                database::user_database_port::UserDatabaseError,
                hasher::hasher_port::{HasherError, HasherPort},
                id_generator::id_generator_port::IdGeneratorPort,
            },
            use_cases::user::create_user_use_case::{
                CreateUserUseCase, CreateUserUseCaseError, CreateUserUseCasePort,
            },
        },
        domain::{entities::user::user_entity::UserEntity, errors::user::user_errors::UserError},
        infrastructure::repositories::user::create_user_repository::CreateUserRepositoryPort,
        presentation::dtos::user::create_user_dto::CreateUserDto,
    };

    mock! {
        pub CreateUserRepository {}

        impl CreateUserRepositoryPort for CreateUserRepository {
            fn execute(
                &self,
                user_entity: UserEntity,
            ) -> Pin<Box<dyn Future<Output = Result<(), UserDatabaseError>> + Send + 'static>>;
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
        let mut create_user_repository_mock: MockCreateUserRepository =
            MockCreateUserRepository::default();

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

        let create_user_use_case: CreateUserUseCase = CreateUserUseCase::new(
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

        let result: Result<(), CreateUserUseCaseError> =
            create_user_use_case.perform(create_user_dto).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_return_error_if_create_user_repository_fails() {
        let mut create_user_repository_mock: MockCreateUserRepository =
            MockCreateUserRepository::default();

        create_user_repository_mock
            .expect_execute()
            .times(1)
            .returning(|_| {
                Box::pin(async move {
                    Err(UserDatabaseError::InsertError {
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

        let create_user_use_case: CreateUserUseCase = CreateUserUseCase::new(
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

        let result: Result<(), CreateUserUseCaseError> =
            create_user_use_case.perform(create_user_dto).await;

        assert!(result.is_err());

        let error: CreateUserUseCaseError = result.unwrap_err();

        assert_eq!(
            error,
            CreateUserUseCaseError::DatabaseError(UserDatabaseError::InsertError {
                message: "Database error".to_string()
            })
        );
    }

    #[tokio::test]
    async fn should_return_error_if_passwords_do_not_match() {
        let create_user_repository_mock: MockCreateUserRepository =
            MockCreateUserRepository::default();

        let hasher_adapter_mock: MockHasherAdapter = MockHasherAdapter::default();

        let id_generator_adapter_mock: MockIdGeneratorAdapter = MockIdGeneratorAdapter::default();

        let create_user_use_case: CreateUserUseCase = CreateUserUseCase::new(
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

        let result: Result<(), CreateUserUseCaseError> =
            create_user_use_case.perform(create_user_dto).await;

        assert!(result.is_err());

        let error: CreateUserUseCaseError = result.unwrap_err();

        assert_eq!(
            error,
            CreateUserUseCaseError::UserError(UserError::PasswordsDoNotMatch)
        );
    }

    #[tokio::test]
    async fn should_return_error_if_password_hash_fails() {
        let create_user_repository_mock: MockCreateUserRepository =
            MockCreateUserRepository::default();

        let mut hasher_adapter_mock: MockHasherAdapter = MockHasherAdapter::default();

        hasher_adapter_mock.expect_hash().times(1).returning(|_| {
            Err(HasherError::HashingError {
                message: "Hashing error".to_string(),
            })
        });

        let id_generator_adapter_mock: MockIdGeneratorAdapter = MockIdGeneratorAdapter::default();

        let create_user_use_case: CreateUserUseCase = CreateUserUseCase::new(
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

        let result: Result<(), CreateUserUseCaseError> =
            create_user_use_case.perform(create_user_dto).await;

        assert!(result.is_err());

        let error: CreateUserUseCaseError = result.unwrap_err();

        assert_eq!(
            error,
            CreateUserUseCaseError::HasherError(HasherError::HashingError {
                message: "Hashing error".to_string()
            })
        );
    }
}

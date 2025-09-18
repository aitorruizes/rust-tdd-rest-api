use std::pin::Pin;

use crate::{
    application::ports::database::user_database_port::{UserDatabaseError, UserDatabasePort},
    domain::entities::user::user_entity::UserEntity,
};

pub trait CreateUserRepositoryPort: Send + Sync {
    fn execute(
        &self,
        user_entity: UserEntity,
    ) -> Pin<Box<dyn Future<Output = Result<UserEntity, UserDatabaseError>> + Sync + Send + '_>>;
}

pub struct CreateUserRepository {
    user_database_gateway: Box<dyn UserDatabasePort>,
}

impl CreateUserRepository {
    pub fn new(user_database_gateway: Box<dyn UserDatabasePort>) -> Self {
        CreateUserRepository {
            user_database_gateway,
        }
    }
}

impl CreateUserRepositoryPort for CreateUserRepository {
    fn execute(
        &self,
        user_entity: UserEntity,
    ) -> Pin<Box<dyn Future<Output = Result<UserEntity, UserDatabaseError>> + Sync + Send + '_>>
    {
        Box::pin(async move {
            self.user_database_gateway
                .as_ref()
                .insert_user(user_entity)
                .await
                .map_err(|err| UserDatabaseError::InsertError {
                    message: err.to_string(),
                })
        })
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;

    use mockall::mock;

    use crate::{
        application::ports::database::user_database_port::{UserDatabaseError, UserDatabasePort},
        domain::entities::user::user_entity::{UserEntity, UserEntityBuilder},
        infrastructure::repositories::user::create_user_repository::{
            CreateUserRepository, CreateUserRepositoryPort,
        },
    };

    mock! {
        pub UserDatabasePort {}

        impl UserDatabasePort for UserDatabasePort {
            fn insert_user(
                &self,
                user_entity: UserEntity,
            ) -> Pin<Box<dyn Future<Output = Result<UserEntity, UserDatabaseError>> + Send + Sync>>;
        }
    }

    #[tokio::test]
    async fn should_successfully_create_an_user() {
        let user_entity_builder: UserEntityBuilder = UserEntityBuilder::default();

        let user_entity: UserEntity = user_entity_builder
            .id("550e8400-e29b-41d4-a716-446655440000".to_string())
            .first_name("John".to_string())
            .last_name("Doe".to_string())
            .email("john.doe@example.com".to_string())
            .password("Password123!".to_string())
            .is_admin(false)
            .created_at("2025-09-17T19:00:00Z".to_string())
            .updated_at("2025-09-17T19:00:00Z".to_string())
            .build();

        let user_entity_for_returning: UserEntity = user_entity.clone();
        let mut user_gateway_mock: MockUserDatabasePort = MockUserDatabasePort::new();

        user_gateway_mock.expect_insert_user().returning(move |_| {
            let user_entity: UserEntity = user_entity_for_returning.clone();

            Box::pin(async move { Ok(user_entity) })
        });

        let create_user_repository: CreateUserRepository =
            CreateUserRepository::new(Box::new(user_gateway_mock));

        let result: Result<UserEntity, UserDatabaseError> =
            create_user_repository.execute(user_entity).await;

        assert!(result.is_ok());

        let created_user: UserEntity = result.unwrap();

        assert_eq!(created_user.first_name, "John");
        assert_eq!(created_user.last_name, "Doe");
        assert_eq!(created_user.email, "john.doe@example.com");
        assert_eq!(created_user.password, "Password123!");
        assert!(!created_user.is_admin);
        assert_eq!(created_user.created_at, "2025-09-17T19:00:00Z");
        assert_eq!(created_user.updated_at, "2025-09-17T19:00:00Z");
    }

    #[tokio::test]
    async fn should_return_error_if_insert_user_fails() {
        let user_entity_builder: UserEntityBuilder = UserEntityBuilder::default();

        let user_entity: UserEntity = user_entity_builder
            .id("550e8400-e29b-41d4-a716-446655440000".to_string())
            .first_name("John".to_string())
            .last_name("Doe".to_string())
            .email("john.doe@example.com".to_string())
            .password("Password123!".to_string())
            .is_admin(false)
            .created_at("2025-09-17T19:00:00Z".to_string())
            .updated_at("2025-09-17T19:00:00Z".to_string())
            .build();

        let mut user_gateway_mock: MockUserDatabasePort = MockUserDatabasePort::new();

        user_gateway_mock.expect_insert_user().returning(move |_| {
            Box::pin(async move {
                Err(UserDatabaseError::InsertError {
                    message: "Database error".to_string(),
                })
            })
        });

        let create_user_repository: CreateUserRepository =
            CreateUserRepository::new(Box::new(user_gateway_mock));

        let result: Result<UserEntity, UserDatabaseError> =
            create_user_repository.execute(user_entity).await;

        assert!(result.is_err());

        let user_database_error: UserDatabaseError = result.unwrap_err();

        assert_eq!(
            user_database_error,
            UserDatabaseError::InsertError {
                message: "Insert Error: Database error".to_string()
            }
        );
    }
}

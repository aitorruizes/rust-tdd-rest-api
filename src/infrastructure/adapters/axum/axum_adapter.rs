use axum::Router;
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;

use crate::{
    application::{
        ports::{
            database::{database_port::PoolWrapper, user_database_port::UserDatabasePort},
            environment::environment_port::EnvironmentPort,
            hasher::hasher_port::HasherPort,
            id_generator::id_generator_port::IdGeneratorPort,
            logger::logger_port::LoggerPort,
            tcp_server::tcp_server_port::TcpServerPort,
            validator::validator_port::ValidatorPort,
            web_framework::web_framework_port::{
                ServeFuture, WebFrameworkPort, WebFrameworkRoutePort,
            },
        },
        use_cases::user::create_user_use_case::{CreateUserUseCase, CreateUserUseCasePort},
    },
    infrastructure::{
        adapters::{
            axum::axum_route_adapter::AxumRouteAdapter, bcrypt::bcrypt_adapter::BcryptAdapter,
            uuid::uuid_adapter::UuidAdapter,
        },
        gateways::database::user_database_gateway::UserDatabaseGateway,
        repositories::user::create_user_repository::{
            CreateUserRepository, CreateUserRepositoryPort,
        },
    },
    presentation::{
        controllers::user::{
            create_user_controller::CreateUserController,
            create_user_validator::CreateUserValidator,
        },
        ports::{controller::controller_port::ControllerPort, router::router_port::RouterPort},
        routers::core::core_router::CoreRouter,
    },
};

pub struct AxumAdapter {
    tcp_server_adapter: Box<dyn TcpServerPort>,
    loggger_adapter: Box<dyn LoggerPort>,
    environment_adapter: Box<dyn EnvironmentPort>,
}

impl AxumAdapter {
    pub fn new(
        tcp_server_adapter: Box<dyn TcpServerPort>,
        loggger_adapter: Box<dyn LoggerPort>,
        environment_adapter: Box<dyn EnvironmentPort>,
    ) -> Self {
        AxumAdapter {
            tcp_server_adapter,
            loggger_adapter,
            environment_adapter,
        }
    }
}

impl WebFrameworkPort for AxumAdapter {
    fn serve(&self, database_pool: Box<dyn PoolWrapper>) -> ServeFuture<'_> {
        Box::pin(async move {
            let server_host: String = self
                .environment_adapter
                .get_environment_file("SERVER_HOST")
                .unwrap_or_else(|err| {
                    self.loggger_adapter.log_error(&err.to_string());
                    std::process::exit(1)
                });

            let server_port: String = self
                .environment_adapter
                .get_environment_file("SERVER_PORT")
                .unwrap_or_else(|err| {
                    self.loggger_adapter.log_error(&err.to_string());
                    std::process::exit(1)
                });

            let server_address: String = format!("{}:{}", server_host, server_port);

            let tcp_listener: TcpListener = self
                .tcp_server_adapter
                .create_listener(server_address.clone())
                .await
                .unwrap_or_else(|err| {
                    self.loggger_adapter.log_error(&err.to_string());
                    std::process::exit(1)
                });

            let server_started_message: String =
                format!("Server successfully started at '{}'.", server_address);

            self.loggger_adapter.log_info(&server_started_message);

            let hasher_adapter: Box<dyn HasherPort> = Box::new(BcryptAdapter);
            let id_generator_adapter: Box<dyn IdGeneratorPort> = Box::new(UuidAdapter);

            let user_database_gateway: Box<dyn UserDatabasePort> =
                Box::new(UserDatabaseGateway::new(
                    *database_pool
                        .into_inner()
                        .downcast::<Pool<Postgres>>()
                        .expect("Failed to downcast"),
                ));

            let create_user_repository: Box<dyn CreateUserRepositoryPort> =
                Box::new(CreateUserRepository::new(user_database_gateway));

            let create_user_use_case: Box<dyn CreateUserUseCasePort> =
                Box::new(CreateUserUseCase::new(
                    hasher_adapter,
                    id_generator_adapter,
                    create_user_repository,
                ));

            let create_user_validator: Box<dyn ValidatorPort> = Box::new(CreateUserValidator);

            let create_user_controller: Box<dyn ControllerPort> = Box::new(
                CreateUserController::new(create_user_validator, create_user_use_case),
            );

            let axum_route_adapter: Box<dyn WebFrameworkRoutePort> = Box::new(AxumRouteAdapter);

            let core_router: CoreRouter =
                CoreRouter::new(axum_route_adapter, create_user_controller);

            let axum_router = core_router
                .register_routes()
                .into_inner()
                .downcast::<Router>()
                .unwrap();

            axum::serve(tcp_listener, axum_router)
                .await
                .unwrap_or_else(|err| {
                    self.loggger_adapter.log_error(&err.to_string());
                    std::process::exit(1)
                });

            Ok(())
        })
    }
}

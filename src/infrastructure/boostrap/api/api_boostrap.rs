use std::pin::Pin;

use axum::Router;
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;

use crate::{
    application::{
        ports::{
            auth::sign_up_repository::SignUpRepositoryPort,
            database::database_port::{DatabasePort, PoolWrapper},
            hasher::hasher_port::HasherPort,
            id_generator::id_generator_port::IdGeneratorPort,
        },
        use_cases::auth::sign_up_use_case::{SignUpUseCase, SignUpUseCasePort},
    },
    infrastructure::{
        adapters::{
            bcrypt::bcrypt_adapter::BcryptAdapter, regex::regex_adapter::RegexAdapter,
            uuid::uuid_adapter::UuidAdapter,
        },
        gateways::database::database_gateway::DatabaseGateway,
        repositories::auth::sign_up_repository::SignUpRepository,
    },
    presentation::{
        controllers::auth::{
            sign_up_controller::SignUpController, sign_up_validator::SignUpValidator,
        },
        ports::router::router_port::RouterPort,
        routers::core::core_router::CoreRouter,
    },
};

pub struct ApiBootstrap;

pub type SetupFuture = Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>>>>;

pub trait ApiBootstrapPort {
    fn setup(&self) -> SetupFuture;
}

impl ApiBootstrap {
    pub fn new() -> Self {
        ApiBootstrap
    }
}

impl ApiBootstrapPort for ApiBootstrap {
    fn setup(&self) -> SetupFuture {
        Box::pin(async move {
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .init();

            match dotenvy::dotenv() {
                Ok(_) => tracing::info!("Environment file successfully loaded."),
                Err(err) => {
                    tracing::error!("{}", &err.to_string());

                    std::process::exit(1)
                }
            };

            let database_gateway: DatabaseGateway = DatabaseGateway;

            let database_pool: Box<dyn PoolWrapper> = database_gateway
                .initialize_pool()
                .await
                .unwrap_or_else(|err| {
                    tracing::error!("{}", &err.to_string());

                    std::process::exit(1)
                });

            tracing::info!("Database pool successfully initialized.");

            let server_host: String = std::env::var("SERVER_HOST").unwrap_or_else(|err| {
                tracing::error!("{}", &err.to_string());

                std::process::exit(1)
            });

            let server_port: String = std::env::var("SERVER_PORT").unwrap_or_else(|err| {
                tracing::error!("{}", &err.to_string());

                std::process::exit(1)
            });

            let server_address: String = format!("{}:{}", server_host, server_port);

            let tcp_listener: TcpListener = TcpListener::bind(server_address.clone())
                .await
                .map_err(|err| {
                    tracing::error!("{}", &err.to_string());

                    std::process::exit(1)
                })?;

            let server_started_message: String =
                format!("Server successfully started at '{}'.", server_address);

            tracing::info!("{}", server_started_message);

            let hasher_adapter: Box<dyn HasherPort> = Box::new(BcryptAdapter);
            let id_generator_adapter: Box<dyn IdGeneratorPort> = Box::new(UuidAdapter);

            let sign_up_repository: Box<dyn SignUpRepositoryPort> =
                Box::new(SignUpRepository::new(
                    *database_pool
                        .into_inner()
                        .downcast::<Pool<Postgres>>()
                        .unwrap(),
                ));

            let sign_up_use_case: Box<dyn SignUpUseCasePort> = Box::new(SignUpUseCase::new(
                hasher_adapter,
                id_generator_adapter,
                sign_up_repository,
            ));

            let sign_up_validator: SignUpValidator = SignUpValidator;
            let regex_adapter: RegexAdapter = RegexAdapter;

            let sign_up_controller: SignUpController =
                SignUpController::new(sign_up_validator, regex_adapter, sign_up_use_case);

            let core_router: CoreRouter = CoreRouter::new(sign_up_controller);
            let axum_router: Router = core_router.register_routes();

            axum::serve(tcp_listener, axum_router)
                .await
                .unwrap_or_else(|err| {
                    tracing::error!("{}", &err.to_string());

                    std::process::exit(1)
                });

            Ok(())
        })
    }
}

impl Default for ApiBootstrap {
    fn default() -> Self {
        Self::new()
    }
}

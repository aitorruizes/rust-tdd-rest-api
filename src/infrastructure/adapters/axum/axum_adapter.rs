use axum::Router;
use sqlx::{Pool, Postgres};
use std::pin::Pin;
use tokio::net::TcpListener;

use crate::{
    application::{
        ports::{
            auth::sign_up_repository::SignUpRepositoryPort, database::database_port::PoolWrapper,
            hasher::hasher_port::HasherPort, id_generator::id_generator_port::IdGeneratorPort,
        },
        use_cases::auth::sign_up_use_case::{SignUpUseCase, SignUpUseCasePort},
    },
    infrastructure::{
        adapters::{
            axum::axum_route_adapter::AxumRouteAdapter, bcrypt::bcrypt_adapter::BcryptAdapter,
            dotenvy::dotenvy_adapter::DotenvyAdapter, regex::regex_adapter::RegexAdapter,
            tokio::tokio_adapter::TokioAdapter, tracing::tracing_adapter::TracingAdapter,
            uuid::uuid_adapter::UuidAdapter,
        },
        repositories::auth::sign_up_repository::SignUpRepository,
    },
    presentation::{
        controllers::user::{
            create_user_controller::CreateUserController,
            create_user_validator::CreateUserValidator,
        },
        ports::router::router_port::RouterPort,
        routers::core::core_router::CoreRouter,
    },
};

pub type ServeFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + 'a>>> + 'a>>;

pub struct AxumAdapter {
    tokio_adapter: TokioAdapter,
    tracing_adapter: TracingAdapter,
    dotenvy_adapter: DotenvyAdapter,
}

impl AxumAdapter {
    pub fn new(
        tokio_adapter: TokioAdapter,
        tracing_adapter: TracingAdapter,
        dotenvy_adapter: DotenvyAdapter,
    ) -> Self {
        AxumAdapter {
            tokio_adapter,
            tracing_adapter,
            dotenvy_adapter,
        }
    }

    pub fn serve(self, database_pool: Box<dyn PoolWrapper>) -> ServeFuture<'static> {
        Box::pin(async move {
            let server_host: String = self
                .dotenvy_adapter
                .get_environment_file("SERVER_HOST")
                .unwrap_or_else(|err| {
                    self.tracing_adapter.log_error(&err.to_string());
                    std::process::exit(1)
                });

            let server_port: String = self
                .dotenvy_adapter
                .get_environment_file("SERVER_PORT")
                .unwrap_or_else(|err| {
                    self.tracing_adapter.log_error(&err.to_string());
                    std::process::exit(1)
                });

            let server_address: String = format!("{}:{}", server_host, server_port);

            let tcp_listener: TcpListener = self
                .tokio_adapter
                .create_listener(server_address.clone())
                .await
                .unwrap_or_else(|err| {
                    self.tracing_adapter.log_error(&err.to_string());
                    std::process::exit(1)
                });

            let server_started_message: String =
                format!("Server successfully started at '{}'.", server_address);

            self.tracing_adapter.log_info(&server_started_message);

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

            let create_user_validator: CreateUserValidator = CreateUserValidator;
            let pattern_matching: RegexAdapter = RegexAdapter;

            let create_user_controller: CreateUserController = CreateUserController::new(
                create_user_validator,
                pattern_matching,
                sign_up_use_case,
            );

            let axum_route_adapter: AxumRouteAdapter = AxumRouteAdapter;

            let core_router: CoreRouter =
                CoreRouter::new(axum_route_adapter, create_user_controller);

            let axum_router: Router = core_router.register_routes();

            axum::serve(tcp_listener, axum_router)
                .await
                .unwrap_or_else(|err| {
                    self.tracing_adapter.log_error(&err.to_string());
                    std::process::exit(1)
                });

            Ok(())
        })
    }
}

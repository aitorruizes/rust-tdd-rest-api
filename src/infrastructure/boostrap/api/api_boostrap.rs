use std::pin::Pin;

use crate::{
    application::ports::{
        database::database_port::{DatabasePort, PoolWrapper},
        environment::environment_port::EnvironmentPort,
        logger::logger_port::LoggerPort,
        logger_subscriber::logger_subsriber_port::LoggerSubscriberPort,
        tcp_server::tcp_server_port::TcpServerPort,
        web_framework::web_framework_port::WebFrameworkPort,
    },
    infrastructure::{
        adapters::{
            axum::axum_adapter::AxumAdapter, dotenvy::dotenvy_adapter::DotenvyAdapter,
            tokio::tokio_adapter::TokioAdapter, tracing::tracing_adapter::TracingAdapter,
            tracing_subscriber::tracing_subscriber_adapter::TracingSubscriberAdapter,
        },
        gateways::database::database_gateway::DatabaseGateway,
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
            let logger_subscriber_adapter: Box<dyn LoggerSubscriberPort> =
                Box::new(TracingSubscriberAdapter);

            logger_subscriber_adapter.initialize();

            let logger_adapter: Box<dyn LoggerPort> = Box::new(TracingAdapter);
            let environment_adapter: Box<dyn EnvironmentPort> = Box::new(DotenvyAdapter::new());

            match environment_adapter.load_environment_file() {
                Ok(_) => logger_adapter.log_info("Environment file successfully loaded."),
                Err(err) => {
                    logger_adapter.log_error(&err.to_string());
                    std::process::exit(1)
                }
            };

            let database_gateway: Box<dyn DatabasePort> = Box::new(DatabaseGateway);

            let database_pool: Box<dyn PoolWrapper> = database_gateway
                .initialize_pool()
                .await
                .unwrap_or_else(|err| {
                    logger_adapter.log_error(&err.to_string());
                    std::process::exit(1)
                });

            logger_adapter.log_info("Database pool successfully initialized.");

            let tcp_server_adapter: Box<dyn TcpServerPort> = Box::new(TokioAdapter);

            let web_framework_adapter: Box<dyn WebFrameworkPort> = Box::new(AxumAdapter::new(
                tcp_server_adapter,
                logger_adapter,
                environment_adapter,
            ));

            web_framework_adapter.serve(database_pool).await
        })
    }
}

impl Default for ApiBootstrap {
    fn default() -> Self {
        Self::new()
    }
}

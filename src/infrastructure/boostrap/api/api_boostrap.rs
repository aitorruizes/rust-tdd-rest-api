use std::pin::Pin;

use crate::{
    application::ports::database::database_port::{DatabasePort, PoolWrapper},
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
            let tracing_subscriber_adapter: TracingSubscriberAdapter = TracingSubscriberAdapter;

            tracing_subscriber_adapter.initialize();

            let tracing_adapter: TracingAdapter = TracingAdapter;
            let dotenvy_adapter: DotenvyAdapter = DotenvyAdapter::new();

            match dotenvy_adapter.load_environment_file() {
                Ok(_) => tracing_adapter.log_info("Environment file successfully loaded."),
                Err(err) => {
                    tracing_adapter.log_error(&err.to_string());
                    std::process::exit(1)
                }
            };

            let database_gateway: DatabaseGateway = DatabaseGateway;

            let database_pool: Box<dyn PoolWrapper> = database_gateway
                .initialize_pool()
                .await
                .unwrap_or_else(|err| {
                    tracing_adapter.log_error(&err.to_string());

                    std::process::exit(1)
                });

            tracing_adapter.log_info("Database pool successfully initialized.");

            let tokio_adapter: TokioAdapter = TokioAdapter;

            let axum_adapter: AxumAdapter =
                AxumAdapter::new(tokio_adapter, tracing_adapter, dotenvy_adapter);

            axum_adapter.serve(database_pool).await
        })
    }
}

impl Default for ApiBootstrap {
    fn default() -> Self {
        Self::new()
    }
}

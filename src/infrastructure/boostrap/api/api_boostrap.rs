use std::{net::SocketAddr, pin::Pin, sync::Arc};

use tokio::net::TcpListener;

use crate::{
    infrastructure::{
        factories::controller::{
            auth::{
                sign_in_controller_factory::SignInControllerFactory,
                sign_up_controller_factory::SignUpControllerFactory,
            },
            user::get_user_by_id_controller_factory::GetUserByIdControllerFactory,
        },
        gateways::database::database_gateway::DatabaseGateway,
    },
    presentation::{
        ports::router::router_port::RouterPort, routers::core::core_router::CoreRouter,
    },
};

pub struct ApiBootstrap;

pub type SetupFuture = Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>>>>;

pub trait ApiBootstrapPort {
    fn setup(&self) -> SetupFuture;
}

impl ApiBootstrap {
    #[must_use]
    pub const fn new() -> Self {
        Self
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
            }

            let database_gateway = DatabaseGateway;

            let database_pool = Arc::new(database_gateway.initialize_pool().await.unwrap_or_else(
                |err| {
                    tracing::error!("{}", &err.to_string());

                    std::process::exit(1)
                },
            ));

            tracing::info!("Database pool successfully initialized.");

            let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|err| {
                tracing::error!("{}", &err.to_string());

                std::process::exit(1)
            });

            let server_port = std::env::var("SERVER_PORT").unwrap_or_else(|err| {
                tracing::error!("{}", &err.to_string());

                std::process::exit(1)
            });

            let server_address = format!("{server_host}:{server_port}");

            let tcp_listener = TcpListener::bind(server_address.clone())
                .await
                .map_err(|err| {
                    tracing::error!("{}", &err.to_string());

                    std::process::exit(1)
                })?;

            let server_started_message =
                format!("Server successfully started at '{server_address}'.");

            tracing::info!("{}", server_started_message);

            let sign_up_controller_factory = SignUpControllerFactory::new(database_pool.clone());
            let sign_up_controller = sign_up_controller_factory.build();

            let sign_in_controller_factory = SignInControllerFactory::new(database_pool.clone());
            let sign_in_controller = sign_in_controller_factory.build();

            let get_user_by_id_controller_factory =
                GetUserByIdControllerFactory::new(database_pool);

            let get_user_by_id_controller = get_user_by_id_controller_factory.build();

            let core_router = CoreRouter::new(
                sign_up_controller,
                sign_in_controller,
                get_user_by_id_controller,
            );
            let axum_router = core_router.register_routes();

            axum::serve(
                tcp_listener,
                axum_router.into_make_service_with_connect_info::<SocketAddr>(),
            )
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

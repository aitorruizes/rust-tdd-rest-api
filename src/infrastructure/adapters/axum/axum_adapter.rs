use axum::Router;
use tokio::net::TcpListener;

use crate::{
    application::ports::{
        environment::environment_port::EnvironmentPort,
        logger::logger_port::LoggerPort,
        tcp_server::tcp_server_port::TcpServerPort,
        web_framework::web_framework_port::{ServeFuture, WebFrameworkPort},
    },
    infrastructure::adapters::axum::axum_route_adapter::AxumRouteAdapter,
    presentation::{
        controllers::user::create_user_controller::CreateUserController,
        ports::router::router_port::RouterPort, routers::core::core_router::CoreRouter,
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
    fn serve(&self) -> ServeFuture<'_> {
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

            let axum_route_adapter: AxumRouteAdapter = AxumRouteAdapter;
            let create_user_controller = CreateUserController;

            let core_router: CoreRouter = CoreRouter::new(
                Box::new(axum_route_adapter),
                Box::new(create_user_controller),
            );

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

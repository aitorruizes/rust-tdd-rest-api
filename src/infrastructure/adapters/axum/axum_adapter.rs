use std::pin::Pin;

use axum::{Router, routing::get};
use tokio::net::TcpListener;

use crate::application::ports::{
    environment::environment_port::EnvironmentPort, logger::logger_port::LoggerPort,
    tcp_server::tcp_server_port::TcpServerPort,
    web_framework::web_framework_port::WebFrameworkPort,
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
    fn serve(&self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + '_>> {
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

            let app: Router = Router::new().route("/", get(|| async { "Hello, World!" }));

            axum::serve(tcp_listener, app).await.unwrap_or_else(|err| {
                self.loggger_adapter.log_error(&err.to_string());
                std::process::exit(1)
            });

            Ok(())
        })
    }
}

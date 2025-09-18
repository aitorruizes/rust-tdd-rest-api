use std::pin::Pin;
use tokio::net::TcpListener;

use crate::application::ports::tcp_server::tcp_server_port::{TcpServerError, TcpServerPort};

pub struct TokioAdapter;

impl TokioAdapter {
    pub fn new() -> Self {
        TokioAdapter
    }
}

impl TcpServerPort for TokioAdapter {
    fn create_listener(
        self,
        server_address: String,
    ) -> Pin<Box<dyn Future<Output = Result<TcpListener, TcpServerError>>>> {
        Box::pin(async move {
            let tcp_listener: TcpListener =
                TcpListener::bind(server_address).await.map_err(|err| {
                    TcpServerError::ListenerCreationError {
                        message: err.to_string(),
                    }
                })?;

            Ok(tcp_listener)
        })
    }
}

impl Default for TokioAdapter {
    fn default() -> Self {
        Self::new()
    }
}

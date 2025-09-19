use std::pin::Pin;
use tokio::net::TcpListener;

#[derive(Debug)]
pub enum TcpServerError {
    ListenerCreationError { message: String },
}

impl std::fmt::Display for TcpServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TcpServerError::ListenerCreationError { message } => {
                write!(f, "An error occurred while creating listener: {}.", message)
            }
        }
    }
}

pub struct TokioAdapter;

impl TokioAdapter {
    pub fn new() -> Self {
        TokioAdapter
    }

    pub fn create_listener(
        &self,
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

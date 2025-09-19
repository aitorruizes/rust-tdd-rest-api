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

impl std::error::Error for TcpServerError {}

pub trait TcpServerPort {
    fn create_listener(
        &self,
        server_address: String,
    ) -> Pin<Box<dyn Future<Output = Result<TcpListener, TcpServerError>>>>;
}

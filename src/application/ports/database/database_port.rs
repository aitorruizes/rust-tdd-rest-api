use std::{any::Any, pin::Pin};

pub trait PoolWrapper: Send + Sync {
    fn into_inner(self: Box<Self>) -> Box<dyn Any + Send + Sync>;
}

#[derive(Debug)]
pub enum DatabaseError {
    Pool { message: String },
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::Pool { message } => {
                write!(
                    f,
                    "An error occurred while initializing database pool: {}.",
                    message
                )
            }
        }
    }
}

impl std::error::Error for DatabaseError {}

pub type InitializePoolFuture =
    Pin<Box<dyn Future<Output = Result<Box<dyn PoolWrapper>, DatabaseError>>>>;

pub trait DatabasePort {
    fn initialize_pool(&self) -> InitializePoolFuture;
}

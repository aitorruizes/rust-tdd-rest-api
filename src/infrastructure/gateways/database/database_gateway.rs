use std::pin::Pin;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

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
    Pin<Box<dyn Future<Output = Result<Pool<Postgres>, DatabaseError>>>>;

pub struct DatabaseGateway;

impl DatabaseGateway {
    pub fn new() -> Self {
        DatabaseGateway
    }

    pub fn initialize_pool(&self) -> InitializePoolFuture {
        Box::pin(async move {
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect("postgres://postgres:1234@localhost:5432/test")
                .await
                .map_err(|err| DatabaseError::Pool {
                    message: err.to_string(),
                })?;

            Ok(pool)
        })
    }
}

impl Default for DatabaseGateway {
    fn default() -> Self {
        Self::new()
    }
}

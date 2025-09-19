use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::any::Any;

use crate::application::ports::database::database_port::{
    DatabaseError, DatabasePort, InitializePoolFuture, PoolWrapper,
};

pub struct SqlxPoolWrapper {
    pool: Pool<Postgres>,
}

impl PoolWrapper for SqlxPoolWrapper {
    fn into_inner(self: Box<Self>) -> Box<dyn Any + Send + Sync> {
        Box::new(self.pool)
    }
}

pub struct DatabaseGateway;

impl DatabaseGateway {
    pub fn new() -> Self {
        DatabaseGateway
    }
}

impl DatabasePort for DatabaseGateway {
    fn initialize_pool(&self) -> InitializePoolFuture {
        Box::pin(async move {
            let pool: Pool<Postgres> = PgPoolOptions::new()
                .max_connections(5)
                .connect("postgres://postgres:1234@localhost:5432/test")
                .await
                .map_err(|err| DatabaseError::Pool {
                    message: err.to_string(),
                })?;

            Ok(Box::new(SqlxPoolWrapper { pool }) as Box<dyn PoolWrapper>)
        })
    }
}

impl Default for DatabaseGateway {
    fn default() -> Self {
        Self::new()
    }
}

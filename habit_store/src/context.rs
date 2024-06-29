use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to connect to the db because: {0}")]
    Connection(#[from] sqlx::Error),
}
pub struct Context {
    pool: Pool<Postgres>,
}

impl Context {
    pub async fn new() -> Result<Context, Error> {
        // todo: parametrise the connection string
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:postgres123@0.0.0.0:25432/habits_db")
            .await?;

        Ok(Context { pool })
    }
}

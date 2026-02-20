pub mod models;
pub mod queries;

use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, sqlite::SqlitePoolOptions, Migrator};

use crate::db::queries::DbPool;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn init_pool(database_url: &str) -> Result<DbPool> {
    if database_url.starts_with("sqlite://") {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        MIGRATOR.run(&pool).await?;
        Ok(DbPool::Sqlite(pool))
    } else {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;
        MIGRATOR.run(&pool).await?;
        Ok(DbPool::Postgres(pool))
    }
}

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tracing::info;

pub type DbPool = Pool<Sqlite>;

pub async fn init_sqlite(path: &str) -> Result<DbPool, sqlx::Error> {
    let database_url = format!("sqlite://{path}");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    info!(%path, "SQLite connection established");

    // TODO: run migrations here once they exist
    // TODO: seed the database with initial data

    Ok(pool)
}

use crate::routes::config::DB_DIR;
use sqlx::{SqlitePool, migrate::Migrator, sqlite::SqlitePoolOptions};
use std::fs;
use std::path::PathBuf;
use tracing::info;

pub mod feed;
pub mod message_queue;
pub mod task;
pub mod task_dependency;
pub mod task_group;
pub mod worker;

/// Embedded SQLx migrator. The macro bundles the migrations at compile time, so
/// the server binary can apply them without extra files at runtime.
#[allow(dead_code)]
pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn ensure_db() -> Result<SqlitePool, sqlx::Error> {
    if let Err(err) = fs::create_dir_all(DB_DIR.as_str()) {
        panic!(
            "Failed to create database directory {}: {err}",
            DB_DIR.as_str()
        );
    }

    let db_path = PathBuf::from(format!("{}/robotfarm.db", DB_DIR.as_str()));
    let database_url = format!("sqlite://{}", db_path.display());

    info!("Connecting to database at {}", database_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    MIGRATOR
        .run(&pool)
        .await
        .unwrap_or_else(|err| panic!("Failed to run migrations: {err}"));

    Ok(pool)
}

use crate::routes::config::DB_DIR;
use once_cell::sync::OnceCell;
use sqlx::{SqlitePool, migrate::Migrator, sqlite::SqlitePoolOptions};
use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;
use tracing::info;

pub mod feed;
pub mod message_queue;
pub mod session;
pub mod task;
pub mod task_dependency;
pub mod task_group;
pub mod worker;

/// Embedded SQLx migrator. The macro bundles the migrations at compile time, so
/// the server binary can apply them without extra files at runtime.
#[allow(dead_code)]
pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

static DB_POOL: OnceCell<SqlitePool> = OnceCell::new();

pub type DbResult<T> = Result<T, sqlx::Error>;

pub fn pool() -> &'static SqlitePool {
    DB_POOL
        .get()
        .expect("database pool not initialized. Call ensure_db() first.")
}

pub async fn ensure_db() -> Result<SqlitePool, sqlx::Error> {
    if let Some(existing) = DB_POOL.get() {
        return Ok(existing.clone());
    }

    if let Err(err) = fs::create_dir_all(DB_DIR.as_str()) {
        panic!(
            "Failed to create database directory {}: {err}",
            DB_DIR.as_str()
        );
    }

    let db_path = PathBuf::from(format!("{}/robotfarm.db", DB_DIR.as_str()));
    let database_url = format!("sqlite://{}", db_path.display());

    info!("Connecting to database at {}", database_url);

    if !db_path.exists() {
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent).map_err(|err| sqlx::Error::Io(err))?;
        }

        OpenOptions::new()
            .create(true)
            .write(true)
            .open(&db_path)
            .map_err(sqlx::Error::Io)?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    MIGRATOR
        .run(&pool)
        .await
        .unwrap_or_else(|err| panic!("Failed to run migrations: {err}"));

    task_group::ensure_builtin_groups(&pool)
        .await
        .unwrap_or_else(|err| panic!("Failed to seed builtin task groups: {err}"));

    let _ = DB_POOL.set(pool.clone());

    Ok(pool)
}

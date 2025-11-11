use sqlx::migrate::Migrator;

/// Embedded SQLx migrator. The macro bundles the migrations at compile time, so
/// the server binary can apply them without extra files at runtime.
#[allow(dead_code)]
pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

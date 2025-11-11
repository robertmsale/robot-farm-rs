use crate::globals::PROJECT_DIR;
use axum::{Json, http::StatusCode};
use openapi::models::{AppendFilesConfig, Config as WorkspaceConfig};
use serde_json::Error as SerdeError;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::LazyLock;
use thiserror::Error;

pub static CONFIG_DIR: LazyLock<String> =
    LazyLock::new(|| format!("{}/.robot-farm", PROJECT_DIR.as_str()));

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config file not found at {0}")]
    NotFound(String),
    #[error("config IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("config serialization error: {0}")]
    Serde(#[from] SerdeError),
}

fn config_file_path() -> PathBuf {
    PathBuf::from(format!("{}/config.json", CONFIG_DIR.as_str()))
}

pub fn config_exists() -> bool {
    config_file_path().exists()
}

fn load_config_from_disk() -> Result<WorkspaceConfig, ConfigError> {
    let path = config_file_path();
    let raw = fs::read_to_string(&path).map_err(|err| match err.kind() {
        ErrorKind::NotFound => ConfigError::NotFound(path.display().to_string()),
        _ => ConfigError::Io(err),
    })?;

    let config: WorkspaceConfig = serde_json::from_str(&raw)?;
    Ok(config)
}

pub fn write_config_to_disk(config: &WorkspaceConfig) -> Result<(), ConfigError> {
    let path = config_file_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(ConfigError::Io)?;
    }

    let serialized = serde_json::to_string_pretty(config)?;
    fs::write(&path, serialized).map_err(ConfigError::Io)?;
    Ok(())
}

pub fn delete_config_file() -> Result<(), ConfigError> {
    let path = config_file_path();
    fs::remove_file(&path).map_err(|err| match err.kind() {
        ErrorKind::NotFound => ConfigError::NotFound(path.display().to_string()),
        _ => ConfigError::Io(err),
    })
}

fn default_config() -> WorkspaceConfig {
    WorkspaceConfig {
        append_agents_file: Box::new(AppendFilesConfig::new(vec![], vec![])),
        commands: vec![],
        post_turn_checks: vec![],
    }
}

pub fn ensure_config_exists() -> Result<(), ConfigError> {
    if config_exists() {
        return Ok(());
    }
    let default = default_config();
    write_config_to_disk(&default)
}

fn map_error(err: ConfigError) -> (StatusCode, String) {
    let status = match err {
        ConfigError::NotFound(_) => StatusCode::NOT_FOUND,
        ConfigError::Serde(_) => StatusCode::BAD_REQUEST,
        ConfigError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
    (status, err.to_string())
}

pub async fn get_config() -> Result<Json<WorkspaceConfig>, (StatusCode, String)> {
    load_config_from_disk().map(Json).map_err(map_error)
}

pub async fn create_config(
    Json(payload): Json<WorkspaceConfig>,
) -> Result<(StatusCode, Json<WorkspaceConfig>), (StatusCode, String)> {
    if config_exists() {
        return Err((StatusCode::CONFLICT, "Config already exists".to_string()));
    }

    write_config_to_disk(&payload).map_err(map_error)?;
    Ok((StatusCode::CREATED, Json(payload)))
}

pub async fn update_config(
    Json(payload): Json<WorkspaceConfig>,
) -> Result<Json<WorkspaceConfig>, (StatusCode, String)> {
    write_config_to_disk(&payload).map_err(map_error)?;
    Ok(Json(payload))
}

pub async fn delete_config() -> Result<StatusCode, (StatusCode, String)> {
    delete_config_file().map_err(map_error)?;
    Ok(StatusCode::NO_CONTENT)
}

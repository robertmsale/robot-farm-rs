use crate::{
    config_sync,
    globals::PROJECT_DIR,
    system::{codex_config, dirty_staging::DirtyStagingAction as SystemDirtyStagingAction},
};
use axum::{Json, http::StatusCode};
use openapi::models::{
    AppendFilesConfig, Config as WorkspaceConfig, DockerOverrides, config::DirtyStagingAction,
};
use serde_json::{Error as SerdeError, Value};
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::LazyLock;
use thiserror::Error;

pub static CONFIG_DIR: LazyLock<String> =
    LazyLock::new(|| format!("{}/.robot-farm-rs", PROJECT_DIR.as_str()));
pub static DB_DIR: LazyLock<String> = LazyLock::new(|| format!("{}/db", CONFIG_DIR.as_str()));

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

pub fn load_config_from_disk() -> Result<WorkspaceConfig, ConfigError> {
    let path = config_file_path();
    let raw = fs::read_to_string(&path).map_err(|err| match err.kind() {
        ErrorKind::NotFound => ConfigError::NotFound(path.display().to_string()),
        _ => ConfigError::Io(err),
    })?;

    let mut value: Value = serde_json::from_str(&raw)?;
    let defaults_added = hydrate_new_fields(&mut value)?;
    let action = value
        .get("dirty_staging_action")
        .and_then(|v| v.as_str())
        .and_then(SystemDirtyStagingAction::from_str)
        .unwrap_or(SystemDirtyStagingAction::Commit);
    crate::system::dirty_staging::set(action);
    let config: WorkspaceConfig = serde_json::from_value(value)?;
    if defaults_added {
        write_config_to_disk(&config)?;
    }
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
        workspace_path: Some(PROJECT_DIR.as_str().to_string()),
        append_agents_file: Box::new(AppendFilesConfig::new(vec![], vec![])),
        models: Box::new(codex_config::default_models()),
        reasoning: Box::new(codex_config::default_reasoning()),
        commands: vec![],
        post_turn_checks: vec![],
        docker_overrides: Box::new(default_docker_overrides()),
        dirty_staging_action: Some(DirtyStagingAction::Commit),
        on_staging_change: Some(vec![]),
        persistent_threads: Some(false),
        ghost_commits: Some(false),
        drift_manager: Some(false),
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

fn map_sync_error(err: config_sync::ConfigSyncError) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("configuration sync failed: {err}"),
    )
}

pub async fn get_config() -> Result<Json<WorkspaceConfig>, (StatusCode, String)> {
    load_config_from_disk()
        .map(|mut cfg| {
            cfg.workspace_path = Some(PROJECT_DIR.as_str().to_string());
            cfg
        })
        .map(Json)
        .map_err(map_error)
}

pub async fn create_config(
    Json(payload): Json<WorkspaceConfig>,
) -> Result<(StatusCode, Json<WorkspaceConfig>), (StatusCode, String)> {
    if config_exists() {
        return Err((StatusCode::CONFLICT, "Config already exists".to_string()));
    }

    validate_workspace_config(&payload)?;
    let mut to_disk = payload.clone();
    to_disk.workspace_path = None;
    write_config_to_disk(&to_disk).map_err(map_error)?;
    config_sync::reload_from_disk().map_err(map_sync_error)?;
    let mut response = payload;
    response.workspace_path = Some(PROJECT_DIR.as_str().to_string());
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn update_config(
    Json(payload): Json<WorkspaceConfig>,
) -> Result<Json<WorkspaceConfig>, (StatusCode, String)> {
    validate_workspace_config(&payload)?;
    let mut to_disk = payload.clone();
    to_disk.workspace_path = None;
    write_config_to_disk(&to_disk).map_err(map_error)?;
    config_sync::reload_from_disk().map_err(map_sync_error)?;
    let mut response = payload;
    response.workspace_path = Some(PROJECT_DIR.as_str().to_string());
    Ok(Json(response))
}

pub async fn delete_config() -> Result<StatusCode, (StatusCode, String)> {
    delete_config_file().map_err(map_error)?;
    config_sync::clear_state().map_err(map_sync_error)?;
    Ok(StatusCode::NO_CONTENT)
}

fn hydrate_new_fields(value: &mut Value) -> Result<bool, SerdeError> {
    let Some(object) = value.as_object_mut() else {
        return Ok(false);
    };

    let mut changed = false;
    if !object.contains_key("models") {
        object.insert(
            "models".to_string(),
            serde_json::to_value(codex_config::default_models())?,
        );
        changed = true;
    }

    if !object.contains_key("reasoning") {
        object.insert(
            "reasoning".to_string(),
            serde_json::to_value(codex_config::default_reasoning())?,
        );
        changed = true;
    }

    if !object.contains_key("docker_overrides") {
        object.insert(
            "docker_overrides".to_string(),
            serde_json::to_value(default_docker_overrides())?,
        );
        changed = true;
    }
    if !object.contains_key("dirty_staging_action") {
        object.insert(
            "dirty_staging_action".to_string(),
            serde_json::to_value(SystemDirtyStagingAction::Commit.as_str())?,
        );
        changed = true;
    }
    if !object.contains_key("on_staging_change") {
        object.insert(
            "on_staging_change".to_string(),
            serde_json::to_value(Vec::<String>::new())?,
        );
        changed = true;
    }
    if !object.contains_key("persistent_threads") {
        object.insert(
            "persistent_threads".to_string(),
            serde_json::Value::Bool(false),
        );
        changed = true;
    }
    if !object.contains_key("ghost_commits") {
        object.insert("ghost_commits".to_string(), serde_json::Value::Bool(false));
        changed = true;
    }
    if !object.contains_key("drift_manager") {
        object.insert("drift_manager".to_string(), serde_json::Value::Bool(false));
        changed = true;
    }

    Ok(changed)
}

fn validate_workspace_config(config: &WorkspaceConfig) -> Result<(), (StatusCode, String)> {
    codex_config::validate_preferences(config.models.as_ref(), config.reasoning.as_ref())
        .map_err(|msg| (StatusCode::BAD_REQUEST, msg))
}

fn default_docker_overrides() -> DockerOverrides {
    DockerOverrides {
        orchestrator: vec![],
        worker: vec![],
        wizard: vec![],
    }
}

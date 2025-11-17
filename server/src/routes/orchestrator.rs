use crate::{db, globals::PROJECT_DIR, shared::shell};
use axum::{Json, http::StatusCode};
use openapi::models::{ExecCommandInput, ExecResult};
use std::path::{Path, PathBuf};
use tracing::error;

pub async fn delete_orchestrator_session() -> StatusCode {
    if let Err(err) = db::session::delete_session("orchestrator").await {
        error!(?err, "failed to clear orchestrator session");
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        StatusCode::NO_CONTENT
    }
}

pub async fn exec_orchestrator_command(
    Json(payload): Json<ExecCommandInput>,
) -> Result<Json<ExecResult>, StatusCode> {
    let command = payload.command.trim();
    if command.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let workspace_root = Path::new(PROJECT_DIR.as_str());
    let staging: PathBuf = workspace_root.join("staging");
    let working_dir = shell::resolve_working_dir(workspace_root, &staging, payload.cwd.as_deref())
        .map_err(|err| {
            error!(?err, "invalid working directory for orchestrator command");
            StatusCode::BAD_REQUEST
        })?;
    let result = shell::run_shell_command(&working_dir, command)
        .await
        .map_err(|err| {
            error!(?err, "failed to execute orchestrator command");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(result))
}

use crate::{globals::PROJECT_DIR, shared::shell};
use axum::{Json, body::Bytes, http::StatusCode, response::IntoResponse};
use openapi::models::ExecResult;
use std::path::PathBuf;
use tracing::error;

pub async fn delete_orchestrator_session() -> StatusCode {
    // TODO: clear orchestrator session state.
    StatusCode::NO_CONTENT
}

pub async fn exec_orchestrator_command(body: Bytes) -> Result<Json<ExecResult>, StatusCode> {
    let command = String::from_utf8(body.to_vec()).map_err(|_| StatusCode::BAD_REQUEST)?;
    let staging: PathBuf = PathBuf::from(PROJECT_DIR.as_str()).join("staging");
    let result = shell::run_shell_command(&staging, command.trim())
        .await
        .map_err(|err| {
            error!(?err, "failed to execute orchestrator command");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(result))
}

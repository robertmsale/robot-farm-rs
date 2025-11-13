use crate::{db, globals::PROJECT_DIR, shared::shell};
use axum::{Json, extract::Path, http::StatusCode};
use openapi::models::{ExecCommandInput, ExecResult, Worker};
use std::path::PathBuf;
use tracing::{error, warn};

pub async fn list_workers() -> Json<Vec<Worker>> {
    let workers = db::worker::list_workers().await;
    Json(workers)
}

pub async fn create_worker() -> (StatusCode, Json<Worker>) {
    let worker = db::worker::create_worker().await;
    (StatusCode::CREATED, Json(worker))
}

pub async fn delete_worker(Path(_worker_id): Path<i64>) -> StatusCode {
    db::worker::delete_worker(_worker_id).await;
    StatusCode::NO_CONTENT
}

pub async fn delete_worker_session(Path(worker_id): Path<i64>) -> StatusCode {
    let owner = format!("ws{worker_id}");
    if let Err(err) = db::session::delete_session(&owner).await {
        warn!(?err, worker_id, "failed to clear worker session");
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        StatusCode::NO_CONTENT
    }
}

pub async fn exec_worker_command(
    Path(worker_id): Path<i64>,
    Json(payload): Json<ExecCommandInput>,
) -> Result<Json<ExecResult>, StatusCode> {
    let command = payload.command.trim();
    if command.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let workspace: PathBuf = PathBuf::from(PROJECT_DIR.as_str()).join(format!("ws{worker_id}"));
    let working_dir =
        shell::resolve_working_dir(&workspace, payload.cwd.as_deref()).map_err(|err| {
            error!(?err, worker_id, "invalid working directory");
            StatusCode::BAD_REQUEST
        })?;
    let result = shell::run_shell_command(&working_dir, command)
        .await
        .map_err(|err| {
            error!(?err, worker_id, "failed to execute worker command");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(result))
}

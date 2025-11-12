use crate::{db, globals::PROJECT_DIR, shared::shell};
use axum::{Json, body::Bytes, extract::Path, http::StatusCode};
use openapi::models::{ExecResult, Worker};
use std::path::PathBuf;
use tracing::error;

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

pub async fn delete_worker_session(Path(_worker_id): Path<i64>) -> StatusCode {
    let _ = _worker_id;
    // TODO: clear worker session state.
    StatusCode::NO_CONTENT
}

pub async fn exec_worker_command(
    Path(worker_id): Path<i64>,
    body: Bytes,
) -> Result<Json<ExecResult>, StatusCode> {
    let command = String::from_utf8(body.to_vec()).map_err(|_| StatusCode::BAD_REQUEST)?;
    let workspace: PathBuf = PathBuf::from(PROJECT_DIR.as_str()).join(format!("ws{worker_id}"));
    let result = shell::run_shell_command(&workspace, command.trim())
        .await
        .map_err(|err| {
            error!(?err, worker_id, "failed to execute worker command");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(result))
}

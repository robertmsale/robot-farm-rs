use crate::{
    db,
    globals::PROJECT_DIR,
    realtime::{self, RealtimeEvent},
    shared::shell,
    system::queue::QueueCoordinator,
    threads,
    threads::queue_manager::QueueManagerError,
};
use axum::{Json, extract::Path as AxumPath, http::StatusCode};
use openapi::models::{ExecCommandInput, ExecResult, Worker};
use std::path::{Path, PathBuf};
use tracing::{error, info, warn};

pub async fn list_workers() -> Json<Vec<Worker>> {
    let workers = db::worker::list_workers().await;
    Json(workers)
}

pub async fn create_worker() -> Result<(StatusCode, Json<Worker>), StatusCode> {
    match db::worker::create_worker().await {
        Ok(worker) => {
            QueueCoordinator::global().register_worker(worker.id);
            broadcast_worker_snapshot().await;
            info!(worker_id = worker.id, "created worker worktree");
            Ok((StatusCode::CREATED, Json(worker)))
        }
        Err(err) => {
            error!(?err, "failed to create worker");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_worker(AxumPath(_worker_id): AxumPath<i64>) -> StatusCode {
    db::worker::delete_worker(_worker_id).await;
    StatusCode::NO_CONTENT
}

pub async fn delete_worker_session(AxumPath(worker_id): AxumPath<i64>) -> StatusCode {
    let owner = format!("ws{worker_id}");
    if let Err(err) = db::session::delete_session(&owner).await {
        warn!(?err, worker_id, "failed to clear worker session");
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        StatusCode::NO_CONTENT
    }
}

pub async fn exec_worker_command(
    AxumPath(worker_id): AxumPath<i64>,
    Json(payload): Json<ExecCommandInput>,
) -> Result<Json<ExecResult>, StatusCode> {
    let command = payload.command.trim();
    if command.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let workspace_root = Path::new(PROJECT_DIR.as_str());
    let workspace: PathBuf = workspace_root.join(format!("ws{worker_id}"));
    let working_dir =
        shell::resolve_working_dir(workspace_root, &workspace, payload.cwd.as_deref()).map_err(
            |err| {
                error!(?err, worker_id, "invalid working directory");
                StatusCode::BAD_REQUEST
            },
        )?;
    let result = shell::run_shell_command(&working_dir, command)
        .await
        .map_err(|err| {
            error!(?err, worker_id, "failed to execute worker command");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(result))
}

pub async fn terminate_worker(AxumPath(worker_id): AxumPath<i64>) -> StatusCode {
    let handles = threads::thread_handles();
    match handles.queue.kill_worker(worker_id).await {
        Ok(_) => StatusCode::ACCEPTED,
        Err(QueueManagerError::WorkerNotRunning(_)) => StatusCode::CONFLICT,
        Err(err) => {
            error!(?err, worker_id, "failed to terminate worker process");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn broadcast_worker_snapshot() {
    let workers = db::worker::list_workers().await;
    realtime::publish(RealtimeEvent::WorkersSnapshot { workers });
}

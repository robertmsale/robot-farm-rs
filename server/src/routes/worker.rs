use crate::db;
use axum::{Json, extract::Path, http::StatusCode};
use openapi::models::Worker;

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

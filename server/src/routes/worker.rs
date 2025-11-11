use axum::{Json, extract::Path, http::StatusCode};
use openapi::models::{Worker, WorkerState};

fn sample_worker(id: i64) -> Worker {
    Worker {
        id,
        last_seen: 0,
        state: WorkerState::Ready,
    }
}

pub async fn list_workers() -> Json<Vec<Worker>> {
    // TODO: list workers from backing store.
    Json(vec![sample_worker(1)])
}

pub async fn create_worker() -> (StatusCode, Json<Worker>) {
    // TODO: create worker record.
    (StatusCode::CREATED, Json(sample_worker(0)))
}

pub async fn delete_worker(Path(_worker_id): Path<i64>) -> StatusCode {
    // TODO: delete worker record.
    StatusCode::NO_CONTENT
}

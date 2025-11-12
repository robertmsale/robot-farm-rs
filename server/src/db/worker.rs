use openapi::models::{Worker, WorkerState};

fn sample_worker(id: i64) -> Worker {
    Worker {
        id,
        last_seen: 0,
        state: WorkerState::Ready,
    }
}

pub async fn list_workers() -> Vec<Worker> {
    // TODO: SELECT FROM workers table.
    vec![sample_worker(1)]
}

pub async fn create_worker() -> Worker {
    // TODO: INSERT new worker row.
    sample_worker(0)
}

pub async fn delete_worker(_worker_id: i64) -> bool {
    // TODO: DELETE worker row.
    true
}

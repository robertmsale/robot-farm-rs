use crate::{system::queue::QueueCoordinator, threads};
use axum::{Json, http::StatusCode};
use openapi::models::QueueState;
use tracing::error;

pub async fn get_queue_state() -> Json<QueueState> {
    Json(QueueState {
        paused: QueueCoordinator::global().is_paused(),
    })
}

pub async fn update_queue_state(
    Json(payload): Json<QueueState>,
) -> Result<Json<QueueState>, StatusCode> {
    let handles = threads::thread_handles();
    let coordinator = QueueCoordinator::global();
    if payload.paused {
        if let Err(err) = handles.queue.pause().await {
            error!(?err, "failed to pause queue manager");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        coordinator.pause();
    } else {
        if let Err(err) = handles.queue.resume().await {
            error!(?err, "failed to resume queue manager");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        coordinator.resume();
    }
    Ok(Json(QueueState {
        paused: coordinator.is_paused(),
    }))
}

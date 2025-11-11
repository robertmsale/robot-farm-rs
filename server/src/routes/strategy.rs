use axum::Json;
use openapi::models::{ActiveStrategy, Strategy};

pub async fn get_active_strategy() -> Json<ActiveStrategy> {
    // TODO: read current strategy from persistent storage.
    Json(ActiveStrategy {
        id: Strategy::Planning,
        focus: Some(vec![1, 2]),
    })
}

pub async fn update_active_strategy(Json(payload): Json<ActiveStrategy>) -> Json<ActiveStrategy> {
    // TODO: persist active strategy update.
    Json(payload)
}

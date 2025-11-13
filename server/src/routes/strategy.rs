use crate::system::strategy::StrategyState;
use axum::Json;
use openapi::models::ActiveStrategy;

pub async fn get_active_strategy() -> Json<ActiveStrategy> {
    Json(StrategyState::global().snapshot())
}

pub async fn update_active_strategy(Json(payload): Json<ActiveStrategy>) -> Json<ActiveStrategy> {
    StrategyState::global().update(payload.clone());
    Json(payload)
}

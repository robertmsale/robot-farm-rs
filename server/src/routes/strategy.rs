use crate::realtime::{self, RealtimeEvent};
use crate::system::{queue::QueueCoordinator, strategy::StrategyState};
use axum::Json;
use openapi::models::ActiveStrategy;
use tracing::info;

pub async fn get_active_strategy() -> Json<ActiveStrategy> {
    Json(StrategyState::global().snapshot())
}

pub async fn update_active_strategy(Json(payload): Json<ActiveStrategy>) -> Json<ActiveStrategy> {
    let strategy = StrategyState::global().update(payload);
    let coordinator = QueueCoordinator::global();
    let hints = coordinator.orchestrator_hints(&strategy);
    if !hints.is_empty() {
        info!("recording {} orchestrator hints", hints.len());
        coordinator.record_assignment_hint(&hints);
    }
    realtime::publish(RealtimeEvent::StrategyState {
        id: strategy.id.clone(),
        focus: strategy.focus.clone().unwrap_or_default(),
    });
    Json(strategy)
}

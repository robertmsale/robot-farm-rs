use axum::http::StatusCode;

pub async fn delete_orchestrator_session() -> StatusCode {
    // TODO: clear orchestrator session state.
    StatusCode::NO_CONTENT
}

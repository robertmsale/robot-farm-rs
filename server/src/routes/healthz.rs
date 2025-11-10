use axum::Json;
use openapi::models::GetHealthz200Response;

pub async fn healthz_handler() -> Json<GetHealthz200Response> {
    let payload = GetHealthz200Response {
        status: "ok".to_string(),
    };
    Json(payload)
}

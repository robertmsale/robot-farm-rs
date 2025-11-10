mod healthz;

use axum::{Router, http::Method, routing::get};
use tower_http::cors::{Any, CorsLayer};

pub fn build_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any)
        .allow_headers(Any);

    Router::new()
        .route("/healthz", get(healthz::healthz_handler))
        .layer(cors)
}

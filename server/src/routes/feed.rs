use axum::{Json, extract::Query, http::StatusCode};
use openapi::models::{Feed, FeedLevel, FeedOrderField};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize)]
pub struct FeedQueryParams {
    pub source: Option<String>,
    pub target: Option<String>,
    pub status: Option<FeedLevel>,
    pub order_by: Option<FeedOrderField>,
}

fn sample_feed(id: i64) -> Feed {
    Feed {
        id,
        source: "System".to_string(),
        target: "Orchestrator".to_string(),
        ts: 0,
        level: FeedLevel::Info,
        text: format!("Sample feed event #{id}"),
        raw: "{}".to_string(),
        category: "general".to_string(),
    }
}

pub async fn list_feed(Query(_query): Query<FeedQueryParams>) -> Json<Vec<Feed>> {
    // TODO: load feed entries from persistence layer.
    Json(vec![sample_feed(1)])
}

pub async fn delete_feed() -> StatusCode {
    // TODO: delete feed entries from persistence layer.
    StatusCode::NO_CONTENT
}

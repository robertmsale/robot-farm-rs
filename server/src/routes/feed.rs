use crate::db;
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

pub async fn list_feed(Query(_query): Query<FeedQueryParams>) -> Json<Vec<Feed>> {
    let feed = db::feed::list_feed().await;
    Json(feed)
}

pub async fn delete_feed() -> StatusCode {
    db::feed::delete_feed().await;
    StatusCode::NO_CONTENT
}

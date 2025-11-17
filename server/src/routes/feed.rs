use crate::{
    db,
    realtime::{self, RealtimeEvent},
};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use openapi::models::{Feed, FeedLevel, FeedOrderField};
use serde::Deserialize;
use tracing::error;

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize)]
pub struct FeedQueryParams {
    pub source: Option<String>,
    pub target: Option<String>,
    pub status: Option<FeedLevel>,
    pub order_by: Option<FeedOrderField>,
}

pub async fn list_feed(
    Query(query): Query<FeedQueryParams>,
) -> Result<Json<Vec<Feed>>, StatusCode> {
    let filters = db::feed::FeedFilters {
        source: query.source,
        target: query.target,
        level: query.status,
        order_by: query.order_by,
        include_raw: false,
    };

    match db::feed::list_feed(filters).await {
        Ok(feed) => Ok(Json(feed)),
        Err(err) => {
            error!(?err, "failed to fetch feed entries");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_feed_entry(Path(feed_id): Path<i64>) -> Result<Json<Feed>, StatusCode> {
    match db::feed::get_feed_entry(feed_id).await {
        Ok(Some(feed)) => Ok(Json(feed)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(err) => {
            error!(?err, feed_id, "failed to fetch feed entry");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_feed() -> StatusCode {
    match db::feed::delete_feed().await {
        Ok(_) => {
            realtime::publish(RealtimeEvent::FeedCleared);
            StatusCode::NO_CONTENT
        }
        Err(err) => {
            error!(?err, "failed to clear feed");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

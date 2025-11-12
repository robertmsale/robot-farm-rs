use crate::db;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use openapi::models::{InsertMessage, Message};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize)]
pub struct MessageQueueQuery {
    pub from: Option<String>,
    pub to: Option<String>,
}

pub async fn list_messages(Query(_query): Query<MessageQueueQuery>) -> Json<Vec<Message>> {
    let messages = db::message_queue::list_messages().await;
    Json(messages)
}

pub async fn delete_all_messages() -> StatusCode {
    db::message_queue::delete_all_messages().await;
    StatusCode::NO_CONTENT
}

pub async fn insert_message_relative(
    Path(message_id): Path<i64>,
    Json(payload): Json<InsertMessage>,
) -> Json<Vec<Message>> {
    let queue = db::message_queue::insert_message_relative(message_id, payload).await;
    Json(queue)
}

pub async fn delete_message_by_id(Path(_message_id): Path<i64>) -> StatusCode {
    db::message_queue::delete_message_by_id(_message_id).await;
    StatusCode::NO_CONTENT
}

pub async fn delete_messages_for_recipient(Path(_recipient): Path<String>) -> StatusCode {
    db::message_queue::delete_messages_for_recipient(&_recipient).await;
    StatusCode::NO_CONTENT
}

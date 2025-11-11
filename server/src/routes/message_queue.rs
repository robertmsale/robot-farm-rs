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

fn sample_message(id: i64) -> Message {
    Message {
        id,
        from: "Orchestrator".to_string(),
        to: "Quality Assurance".to_string(),
        message: "Placeholder message".to_string(),
        inserted_at: 0,
    }
}

pub async fn list_messages(Query(_query): Query<MessageQueueQuery>) -> Json<Vec<Message>> {
    // TODO: fetch messages based on filters.
    Json(vec![sample_message(1)])
}

pub async fn delete_all_messages() -> StatusCode {
    // TODO: clear message queue.
    StatusCode::NO_CONTENT
}

pub async fn insert_message_relative(
    Path(message_id): Path<i64>,
    Json(_payload): Json<InsertMessage>,
) -> Json<Vec<Message>> {
    // TODO: insert message before/after the reference message.
    let _ = message_id;
    Json(vec![sample_message(1)])
}

pub async fn delete_message_by_id(Path(_message_id): Path<i64>) -> StatusCode {
    // TODO: delete a single message.
    StatusCode::NO_CONTENT
}

pub async fn delete_messages_for_recipient(Path(_recipient): Path<String>) -> StatusCode {
    // TODO: delete all messages targeting the recipient.
    StatusCode::NO_CONTENT
}

use crate::db;
use crate::threads;
use crate::threads::database_manager::DatabaseManagerError;
use crate::threads::queue_manager::QueueManagerError;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use openapi::models::{InsertMessage, Message};
use serde::Deserialize;
use tracing::error;

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize)]
pub struct MessageQueueQuery {
    pub from: Option<String>,
    pub to: Option<String>,
}

pub async fn list_messages(
    Query(query): Query<MessageQueueQuery>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    let filters = db::message_queue::MessageFilters {
        from: query.from,
        to: query.to,
    };

    let handles = threads::thread_handles();
    match handles.queue.list_messages(filters).await {
        Ok(messages) => Ok(Json(messages)),
        Err(err) => {
            error!(?err, "failed to list queue messages");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_all_messages() -> StatusCode {
    let handles = threads::thread_handles();
    match handles.queue.delete_all_messages().await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(err) => {
            error!(?err, "failed to clear message queue");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn insert_message_relative(
    Path(message_id): Path<i64>,
    Json(payload): Json<InsertMessage>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    let directive = match payload {
        InsertMessage::InsertMessageOneOf(before) => {
            db::message_queue::RelativePosition::Before(before.before)
        }
        InsertMessage::InsertMessageOneOf1(after) => {
            db::message_queue::RelativePosition::After(after.after)
        }
    };

    let handles = threads::thread_handles();
    match handles
        .queue
        .insert_message_relative(message_id, directive)
        .await
    {
        Ok(queue) => Ok(Json(queue)),
        Err(err) if is_message_missing(&err) => Err(StatusCode::NOT_FOUND),
        Err(err) => {
            error!(?err, "failed to reorder message queue");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_message_by_id(Path(message_id): Path<i64>) -> StatusCode {
    let handles = threads::thread_handles();
    match handles.queue.delete_message_by_id(message_id).await {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(err) => {
            error!(?err, message_id, "failed to delete queue message");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn delete_messages_for_recipient(Path(recipient): Path<String>) -> StatusCode {
    let handles = threads::thread_handles();
    match handles
        .queue
        .delete_messages_for_recipient(recipient.clone())
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(err) => {
            error!(?err, recipient, "failed to delete messages for recipient");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

fn is_message_missing(error: &QueueManagerError) -> bool {
    match error {
        QueueManagerError::Database(DatabaseManagerError::MessageQueue(inner)) => matches!(
            inner,
            db::message_queue::MessageQueueError::MessageNotFound
                | db::message_queue::MessageQueueError::AnchorNotFound
        ),
        _ => false,
    }
}

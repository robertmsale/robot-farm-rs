use crate::db::{self, DbResult};
use chrono::Utc;
use openapi::models::Message;
use sqlx::{QueryBuilder, Row, Sqlite};

#[derive(Clone, Debug, Default)]
pub struct MessageFilters {
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Clone, Copy, Debug)]
pub enum RelativePosition {
    Before(i64),
    After(i64),
}

#[derive(thiserror::Error, Debug)]
pub enum MessageQueueError {
    #[error("message not found")]
    MessageNotFound,
    #[error("anchor message not found")]
    AnchorNotFound,
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

pub async fn list_messages(filters: MessageFilters) -> DbResult<Vec<Message>> {
    let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT id, from_actor, to_actor, message, inserted_at FROM message_queue",
    );
    let mut has_clause = false;

    if let Some(from) = filters.from {
        builder
            .push(if has_clause { " AND " } else { " WHERE " })
            .push("from_actor = ")
            .push_bind(from);
        has_clause = true;
    }

    if let Some(to) = filters.to {
        builder
            .push(if has_clause { " AND " } else { " WHERE " })
            .push("to_actor = ")
            .push_bind(to);
    }

    builder.push(" ORDER BY inserted_at ASC");

    let rows = builder.build().fetch_all(db::pool()).await?;
    Ok(rows.into_iter().map(row_to_message).collect())
}

pub async fn delete_all_messages() -> DbResult<()> {
    sqlx::query("DELETE FROM message_queue")
        .execute(db::pool())
        .await?;
    Ok(())
}

pub async fn delete_message_by_id(message_id: i64) -> DbResult<bool> {
    let result = sqlx::query("DELETE FROM message_queue WHERE id = ?1")
        .bind(message_id)
        .execute(db::pool())
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_messages_for_recipient(recipient: &str) -> DbResult<u64> {
    let result = sqlx::query("DELETE FROM message_queue WHERE to_actor = ?1")
        .bind(recipient)
        .execute(db::pool())
        .await?;
    Ok(result.rows_affected())
}

pub async fn insert_message_relative(
    message_id: i64,
    directive: RelativePosition,
) -> Result<Vec<Message>, MessageQueueError> {
    let mut pooled_conn = db::pool().acquire().await?;
    let conn = pooled_conn.as_mut();
    sqlx::query("BEGIN IMMEDIATE").execute(&mut *conn).await?;

    let rows = sqlx::query(
        "SELECT id, from_actor, to_actor, message, inserted_at FROM message_queue ORDER BY inserted_at ASC",
    )
    .fetch_all(&mut *conn)
    .await?;

    let mut queue: Vec<Message> = rows.into_iter().map(row_to_message).collect();
    if queue.is_empty() {
        let _ = sqlx::query("ROLLBACK").execute(&mut *conn).await;
        return Err(MessageQueueError::MessageNotFound);
    }

    let current_index = match queue.iter().position(|item| item.id == message_id) {
        Some(idx) => idx,
        None => {
            let _ = sqlx::query("ROLLBACK").execute(&mut *conn).await;
            return Err(MessageQueueError::MessageNotFound);
        }
    };

    let anchor_id = match directive {
        RelativePosition::Before(anchor) => anchor,
        RelativePosition::After(anchor) => anchor,
    };

    let anchor_index = match queue.iter().position(|item| item.id == anchor_id) {
        Some(idx) => idx,
        None => {
            let _ = sqlx::query("ROLLBACK").execute(&mut *conn).await;
            return Err(MessageQueueError::AnchorNotFound);
        }
    };

    let mut new_index = match directive {
        RelativePosition::Before(_) => anchor_index,
        RelativePosition::After(_) => anchor_index + 1,
    };

    let message = queue.remove(current_index);
    if new_index > current_index {
        new_index = new_index.saturating_sub(1);
    }
    if new_index > queue.len() {
        new_index = queue.len();
    }
    queue.insert(new_index, message);

    let base = Utc::now().timestamp();
    for (offset, item) in queue.iter_mut().enumerate() {
        item.inserted_at = base + offset as i64;
        sqlx::query("UPDATE message_queue SET inserted_at = ?1 WHERE id = ?2")
            .bind(item.inserted_at)
            .bind(item.id)
            .execute(&mut *conn)
            .await?;
    }

    sqlx::query("COMMIT").execute(&mut *conn).await?;
    Ok(queue)
}

fn row_to_message(row: sqlx::sqlite::SqliteRow) -> Message {
    Message {
        id: row.get("id"),
        from: row.get("from_actor"),
        to: row.get("to_actor"),
        message: row.get("message"),
        inserted_at: row.get("inserted_at"),
    }
}

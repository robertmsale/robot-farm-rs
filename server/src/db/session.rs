use crate::db::{self, DbResult};

pub async fn upsert_session(owner: &str, thread_id: &str) -> DbResult<()> {
    sqlx::query(
        r#"
        INSERT INTO session(owner, thread_id, updated_at)
        VALUES (?1, ?2, strftime('%s','now'))
        ON CONFLICT(owner)
        DO UPDATE SET thread_id = excluded.thread_id,
                      updated_at = excluded.updated_at
        "#,
    )
    .bind(owner)
    .bind(thread_id)
    .execute(db::pool())
    .await?;
    Ok(())
}

pub async fn delete_session(owner: &str) -> DbResult<()> {
    sqlx::query("DELETE FROM session WHERE owner = ?1")
        .bind(owner)
        .execute(db::pool())
        .await?;
    Ok(())
}

pub async fn clear_sessions() -> DbResult<()> {
    sqlx::query("DELETE FROM session")
        .execute(db::pool())
        .await?;
    Ok(())
}

pub async fn get_session(owner: &str) -> DbResult<Option<String>> {
    let row = sqlx::query("SELECT thread_id FROM session WHERE owner = ?1")
        .bind(owner)
        .fetch_optional(db::pool())
        .await?;
    Ok(row.map(|r| r.get::<String, _>("thread_id")))
}

use sqlx::Row;

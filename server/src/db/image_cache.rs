use crate::db::{self, DbResult};
use chrono::Utc;
use sqlx::Row;

#[derive(Clone, Debug)]
pub struct ImageCacheEntry {
    pub tag: String,
    pub hash: String,
    pub updated_at: i64,
}

pub async fn get_hash(tag: &str) -> DbResult<Option<String>> {
    let row = sqlx::query(
        r#"
        SELECT hash
        FROM image_cache
        WHERE tag = ?1
        "#,
    )
    .bind(tag)
    .fetch_optional(db::pool())
    .await?;

    Ok(row.map(|r| r.get::<String, _>("hash")))
}

pub async fn upsert_hash(tag: &str, hash: &str) -> DbResult<()> {
    let now = Utc::now().timestamp();
    sqlx::query(
        r#"
        INSERT INTO image_cache (tag, hash, updated_at)
        VALUES (?1, ?2, ?3)
        ON CONFLICT(tag) DO UPDATE SET
            hash = excluded.hash,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(tag)
    .bind(hash)
    .bind(now)
    .execute(db::pool())
    .await?;
    Ok(())
}

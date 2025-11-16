use crate::db::{self, DbResult};
use openapi::models::{Task, TaskCreateInput, TaskStatus, TaskUpdateInput};
use sqlx::Row;
use tracing::debug;

fn parse_status(raw: &str) -> TaskStatus {
    match raw {
        "Blocked" => TaskStatus::Blocked,
        "Done" => TaskStatus::Done,
        _ => TaskStatus::Ready,
    }
}

fn row_to_task(row: sqlx::sqlite::SqliteRow) -> Task {
    let status: String = row.get("status");
    Task {
        id: row.get("id"),
        group_id: row.get("group_id"),
        slug: row.get("slug"),
        title: row.get("title"),
        commit_hash: row.get("commit_hash"),
        status: parse_status(&status),
        owner: row.get("owner"),
        description: row.get("description"),
    }
}

pub async fn list_tasks() -> DbResult<Vec<Task>> {
    let rows = sqlx::query(
        r#"
        SELECT id, group_id, slug, title, commit_hash, status, owner, description
        FROM task
        ORDER BY id ASC
        "#,
    )
    .fetch_all(db::pool())
    .await?;

    Ok(rows.into_iter().map(row_to_task).collect())
}

pub async fn create_task(payload: TaskCreateInput) -> DbResult<Task> {
    let TaskCreateInput {
        group_id,
        slug,
        title,
        commit_hash,
        status,
        owner,
        description,
    } = payload;

    let status_str = status.to_string();

    let row = sqlx::query(
        r#"
        INSERT INTO task (group_id, slug, title, commit_hash, status, owner, description)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        RETURNING id, group_id, slug, title, commit_hash, status, owner, description
        "#,
    )
    .bind(group_id)
    .bind(slug)
    .bind(title)
    .bind(commit_hash)
    .bind(status_str)
    .bind(owner)
    .bind(description)
    .fetch_one(db::pool())
    .await?;

    Ok(row_to_task(row))
}

pub async fn get_task(task_id: i64) -> DbResult<Option<Task>> {
    let row = sqlx::query(
        r#"
        SELECT id, group_id, slug, title, commit_hash, status, owner, description
        FROM task
        WHERE id = ?1
        "#,
    )
    .bind(task_id)
    .fetch_optional(db::pool())
    .await?;

    Ok(row.map(row_to_task))
}

pub async fn update_task(task_id: i64, payload: TaskUpdateInput) -> DbResult<Option<Task>> {
    let TaskUpdateInput {
        group_id,
        slug,
        title,
        commit_hash,
        status,
        owner,
        description,
    } = payload;

    if group_id.is_none()
        && slug.is_none()
        && title.is_none()
        && commit_hash.is_none()
        && status.is_none()
        && owner.is_none()
        && description.is_none()
    {
        return get_task(task_id).await;
    }

    debug!(task_id, ?group_id, ?slug, ?title, ?status, owner = ?owner, "Applying task update");

    let status = status.map(|s| s.to_string());

    let row = sqlx::query(
        r#"
        UPDATE task SET
            group_id = COALESCE(?1, group_id),
            slug = COALESCE(?2, slug),
            title = COALESCE(?3, title),
            commit_hash = COALESCE(?4, commit_hash),
            status = COALESCE(?5, status),
            owner = COALESCE(?6, owner),
            description = COALESCE(?7, description)
        WHERE id = ?8
        RETURNING id, group_id, slug, title, commit_hash, status, owner, description
        "#,
    )
    .bind(group_id)
    .bind(slug)
    .bind(title)
    .bind(commit_hash)
    .bind(status)
    .bind(owner)
    .bind(description)
    .bind(task_id)
    .fetch_optional(db::pool())
    .await?;

    Ok(row.map(row_to_task))
}

pub async fn delete_task(task_id: i64) -> DbResult<bool> {
    let result = sqlx::query(
        r#"
        DELETE FROM task
        WHERE id = ?1
        "#,
    )
    .bind(task_id)
    .execute(db::pool())
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_task_by_slug(slug: &str) -> DbResult<Option<Task>> {
    let row = sqlx::query(
        r#"
        SELECT id, group_id, slug, title, commit_hash, status, owner, description
        FROM task
        WHERE slug = ?1
        "#,
    )
    .bind(slug)
    .fetch_optional(db::pool())
    .await?;

    Ok(row.map(row_to_task))
}

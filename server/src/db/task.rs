use crate::db::{self, DbResult};
use openapi::models::{Task, TaskCreateInput, TaskStatus, TaskUpdateInput};
use sqlx::Row;
use tracing::debug;

fn normalize_owner(owner: Option<String>) -> Option<String> {
    owner.map(|o| o.to_ascii_lowercase())
}

fn normalize_owner_owned(owner: String) -> String {
    owner.to_ascii_lowercase()
}

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
        model_override: row.get("model_override"),
        reasoning_override: row.get("reasoning_override"),
    }
}

pub async fn list_tasks() -> DbResult<Vec<Task>> {
    let rows = sqlx::query(
        r#"
        SELECT id, group_id, slug, title, commit_hash, status, owner, description
        , model_override, reasoning_override
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
        model_override,
        reasoning_override,
    } = payload;

    let status_str = status.to_string();
    let owner = normalize_owner_owned(owner);

    let row = sqlx::query(
        r#"
        INSERT INTO task (
            group_id,
            slug,
            title,
            commit_hash,
            status,
            owner,
            description,
            model_override,
            reasoning_override
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        RETURNING id, group_id, slug, title, commit_hash, status, owner, description, model_override, reasoning_override
        "#,
    )
    .bind(group_id)
    .bind(slug)
    .bind(title)
    .bind(commit_hash)
    .bind(status_str)
    .bind(owner)
    .bind(description)
    .bind(model_override)
    .bind(reasoning_override)
    .fetch_one(db::pool())
    .await?;

    Ok(row_to_task(row))
}

pub async fn get_task(task_id: i64) -> DbResult<Option<Task>> {
    let row = sqlx::query(
        r#"
        SELECT id, group_id, slug, title, commit_hash, status, owner, description
        , model_override, reasoning_override
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
        model_override,
        reasoning_override,
    } = payload;

    if group_id.is_none()
        && slug.is_none()
        && title.is_none()
        && commit_hash.is_none()
        && status.is_none()
        && owner.is_none()
        && description.is_none()
        && model_override.is_none()
        && reasoning_override.is_none()
    {
        return get_task(task_id).await;
    }

    debug!(task_id, ?group_id, ?slug, ?title, ?status, owner = ?owner, "Applying task update");

    let status = status.map(|s| s.to_string());
    let owner = normalize_owner(owner);

    let row = sqlx::query(
        r#"
        UPDATE task SET
            group_id = COALESCE(?1, group_id),
            slug = COALESCE(?2, slug),
            title = COALESCE(?3, title),
            commit_hash = COALESCE(?4, commit_hash),
            status = COALESCE(?5, status),
            owner = COALESCE(?6, owner),
            description = COALESCE(?7, description),
            model_override = COALESCE(?8, model_override),
            reasoning_override = COALESCE(?9, reasoning_override)
        WHERE id = ?10
        RETURNING id, group_id, slug, title, commit_hash, status, owner, description, model_override, reasoning_override
        "#,
    )
    .bind(group_id)
    .bind(slug)
    .bind(title)
    .bind(commit_hash)
    .bind(status)
    .bind(owner)
    .bind(description)
    .bind(model_override)
    .bind(reasoning_override)
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
        , model_override, reasoning_override
        FROM task
        WHERE slug = ?1
        "#,
    )
    .bind(slug)
    .fetch_optional(db::pool())
    .await?;

    Ok(row.map(row_to_task))
}

/// Mark a task as Done and set its owner, matched by slug. Returns true if a row was updated.
pub async fn mark_done_and_owner(slug: &str, owner: &str) -> DbResult<bool> {
    let normalized_owner = owner.to_ascii_lowercase();
    let result = sqlx::query(
        r#"
        UPDATE task
        SET status = 'Done', owner = ?2
        WHERE slug = ?1
        "#,
    )
    .bind(slug)
    .bind(normalized_owner)
    .execute(db::pool())
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn count_ready_in_group(group_id: i64) -> DbResult<i64> {
    let row = sqlx::query(
        r#"
        SELECT COUNT(*) as cnt
        FROM task
        WHERE group_id = ?1
          AND status = 'Ready'
        "#,
    )
    .bind(group_id)
    .fetch_one(db::pool())
    .await?;

    Ok(row.get::<i64, _>("cnt"))
}

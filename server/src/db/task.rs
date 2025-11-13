use crate::db::{self, DbResult};
use openapi::models::{Task, TaskCreateInput, TaskStatus, TaskUpdateInput};
use sqlx::{QueryBuilder, Row, Sqlite};

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
    }
}

pub async fn list_tasks() -> DbResult<Vec<Task>> {
    let rows = sqlx::query(
        r#"
        SELECT id, group_id, slug, title, commit_hash, status, owner
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
    } = payload;

    let status_str = status.to_string();

    let row = sqlx::query(
        r#"
        INSERT INTO task (group_id, slug, title, commit_hash, status, owner)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        RETURNING id, group_id, slug, title, commit_hash, status, owner
        "#,
    )
    .bind(group_id)
    .bind(slug)
    .bind(title)
    .bind(commit_hash)
    .bind(status_str)
    .bind(owner)
    .fetch_one(db::pool())
    .await?;

    Ok(row_to_task(row))
}

pub async fn get_task(task_id: i64) -> DbResult<Option<Task>> {
    let row = sqlx::query(
        r#"
        SELECT id, group_id, slug, title, commit_hash, status, owner
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
    } = payload;

    if group_id.is_none()
        && slug.is_none()
        && title.is_none()
        && commit_hash.is_none()
        && status.is_none()
        && owner.is_none()
    {
        return get_task(task_id).await;
    }

    let mut builder = QueryBuilder::<Sqlite>::new("UPDATE task SET ");
    let mut assignments = builder.separated(", ");

    if let Some(group_id) = group_id {
        assignments.push("group_id = ").push_bind(group_id);
    }
    if let Some(slug) = slug {
        assignments.push("slug = ").push_bind(slug);
    }
    if let Some(title) = title {
        assignments.push("title = ").push_bind(title);
    }
    if let Some(commit_hash) = commit_hash {
        assignments.push("commit_hash = ").push_bind(commit_hash);
    }
    if let Some(status) = status {
        assignments.push("status = ").push_bind(status.to_string());
    }
    if let Some(owner) = owner {
        assignments.push("owner = ").push_bind(owner);
    }

    builder
        .push(" WHERE id = ")
        .push_bind(task_id)
        .push(" RETURNING id, group_id, slug, title, commit_hash, status, owner");

    let row = builder.build().fetch_optional(db::pool()).await?;
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
        SELECT id, group_id, slug, title, commit_hash, status, owner
        FROM task
        WHERE slug = ?1
        "#,
    )
    .bind(slug)
    .fetch_optional(db::pool())
    .await?;

    Ok(row.map(row_to_task))
}

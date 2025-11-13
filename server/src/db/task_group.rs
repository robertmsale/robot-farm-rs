use crate::db::{self, DbResult};
use openapi::models::{TaskGroup, TaskGroupCreateInput, TaskGroupStatus, TaskGroupUpdateInput};
use sqlx::{QueryBuilder, Row, Sqlite, SqlitePool};

const BUILTIN_GROUPS: &[(&str, &str, &str)] = &[
    ("chores", "Chores", "Keep the codebase clean and efficient."),
    (
        "bugs",
        "Bugs",
        "Minor inconveniences that need adjustments.",
    ),
    (
        "hotfix",
        "Hotfix",
        "Major catastrophes requiring immediate attention.",
    ),
];

fn parse_status(raw: &str) -> TaskGroupStatus {
    match raw {
        "Done" => TaskGroupStatus::Done,
        _ => TaskGroupStatus::Ready,
    }
}

fn row_to_task_group(row: sqlx::sqlite::SqliteRow) -> TaskGroup {
    let status: String = row.get("status");
    TaskGroup {
        id: row.get("id"),
        slug: row.get("slug"),
        title: row.get("title"),
        description: row.get("description"),
        status: parse_status(&status),
    }
}

pub async fn list_task_groups() -> DbResult<Vec<TaskGroup>> {
    let rows = sqlx::query(
        r#"
        SELECT id, slug, title, description, status
        FROM task_group
        ORDER BY id ASC
        "#,
    )
    .fetch_all(db::pool())
    .await?;

    Ok(rows.into_iter().map(row_to_task_group).collect())
}

pub async fn create_task_group(payload: TaskGroupCreateInput) -> DbResult<TaskGroup> {
    let TaskGroupCreateInput {
        slug,
        title,
        description,
    } = payload;

    let status = TaskGroupStatus::Ready.to_string();

    let row = sqlx::query(
        r#"
        INSERT INTO task_group (slug, title, description, status)
        VALUES (?1, ?2, ?3, ?4)
        RETURNING id, slug, title, description, status
        "#,
    )
    .bind(slug)
    .bind(title)
    .bind(description)
    .bind(status)
    .fetch_one(db::pool())
    .await?;

    Ok(row_to_task_group(row))
}

pub async fn get_task_group(task_group_id: i64) -> DbResult<Option<TaskGroup>> {
    let row = sqlx::query(
        r#"
        SELECT id, slug, title, description, status
        FROM task_group
        WHERE id = ?1
        "#,
    )
    .bind(task_group_id)
    .fetch_optional(db::pool())
    .await?;

    Ok(row.map(row_to_task_group))
}

pub async fn update_task_group(
    task_group_id: i64,
    payload: TaskGroupUpdateInput,
) -> DbResult<Option<TaskGroup>> {
    let TaskGroupUpdateInput {
        slug,
        title,
        description,
    } = payload;

    if slug.is_none() && title.is_none() && description.is_none() {
        return get_task_group(task_group_id).await;
    }

    let mut builder = QueryBuilder::<Sqlite>::new("UPDATE task_group SET ");
    let mut assignments = builder.separated(", ");

    if let Some(slug) = slug {
        assignments.push("slug = ").push_bind(slug);
    }
    if let Some(title) = title {
        assignments.push("title = ").push_bind(title);
    }
    if let Some(description) = description {
        assignments.push("description = ").push_bind(description);
    }

    builder
        .push(" WHERE id = ")
        .push_bind(task_group_id)
        .push(" RETURNING id, slug, title, description, status");

    let row = builder.build().fetch_optional(db::pool()).await?;
    Ok(row.map(row_to_task_group))
}

pub async fn delete_task_group(task_group_id: i64) -> DbResult<bool> {
    let result = sqlx::query(
        r#"
        DELETE FROM task_group
        WHERE id = ?1
        "#,
    )
    .bind(task_group_id)
    .execute(db::pool())
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn ensure_builtin_groups(pool: &SqlitePool) -> DbResult<()> {
    for (slug, title, description) in BUILTIN_GROUPS {
        sqlx::query(
            r#"
            INSERT INTO task_group (slug, title, description, status)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(slug) DO UPDATE SET
                title = excluded.title,
                description = excluded.description
            "#,
        )
        .bind(slug)
        .bind(title)
        .bind(description)
        .bind(TaskGroupStatus::Ready.to_string())
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn get_task_group_by_slug(slug: &str) -> DbResult<Option<TaskGroup>> {
    let row = sqlx::query(
        r#"
        SELECT id, slug, title, description, status
        FROM task_group
        WHERE slug = ?1
        "#,
    )
    .bind(slug)
    .fetch_optional(db::pool())
    .await?;

    Ok(row.map(row_to_task_group))
}

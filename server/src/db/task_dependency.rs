use crate::db::{self, DbResult};
use openapi::models::{TaskDependency, TaskDependencyCreateInput};
use sqlx::Row;

pub async fn list_task_dependencies(task_id: i64) -> DbResult<Vec<i64>> {
    let rows = sqlx::query(
        r#"
        SELECT depends_on_task_id
        FROM task_deps
        WHERE task_id = ?1
        ORDER BY depends_on_task_id ASC
        "#,
    )
    .bind(task_id)
    .fetch_all(db::pool())
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| row.get::<i64, _>("depends_on_task_id"))
        .collect())
}

pub async fn create_task_dependency(
    payload: TaskDependencyCreateInput,
) -> DbResult<TaskDependency> {
    let TaskDependencyCreateInput {
        task_id,
        depends_on_task_id,
    } = payload;

    sqlx::query(
        r#"
        INSERT INTO task_deps (task_id, depends_on_task_id)
        VALUES (?1, ?2)
        "#,
    )
    .bind(task_id)
    .bind(depends_on_task_id)
    .execute(db::pool())
    .await?;

    Ok(TaskDependency {
        task_id,
        depends_on_task_id,
    })
}

pub async fn delete_task_dependency(task_id: i64, depends_on_task_id: i64) -> DbResult<bool> {
    let result = sqlx::query(
        r#"
        DELETE FROM task_deps
        WHERE task_id = ?1 AND depends_on_task_id = ?2
        "#,
    )
    .bind(task_id)
    .bind(depends_on_task_id)
    .execute(db::pool())
    .await?;

    Ok(result.rows_affected() > 0)
}

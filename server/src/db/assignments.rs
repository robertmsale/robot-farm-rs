use crate::db::{self, DbResult};
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct ActiveAssignment {
    pub worker: String,
    pub task_slug: String,
}

pub async fn list_active_assignments() -> DbResult<Vec<ActiveAssignment>> {
    let rows = sqlx::query(
        r#"
        SELECT owner, slug
        FROM task
        WHERE status = 'Ready' AND owner LIKE 'ws%'
        "#,
    )
    .fetch_all(db::pool())
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| ActiveAssignment {
            worker: row.get::<String, _>("owner"),
            task_slug: row.get::<String, _>("slug"),
        })
        .collect())
}

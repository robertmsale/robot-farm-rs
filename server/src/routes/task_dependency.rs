use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use openapi::models::{TaskDependency, TaskDependencyCreateInput};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TaskDependencyQuery {
    pub task_id: i64,
}

pub async fn list_task_dependencies(Query(query): Query<TaskDependencyQuery>) -> Json<Vec<i64>> {
    // TODO: fetch dependencies for task_id.
    let deps = if query.task_id == 0 {
        vec![]
    } else {
        vec![1, 2, 3]
    };
    Json(deps)
}

pub async fn create_task_dependency(
    Json(payload): Json<TaskDependencyCreateInput>,
) -> (StatusCode, Json<TaskDependency>) {
    // TODO: persist dependency.
    let dependency = TaskDependency {
        task_id: payload.task_id,
        depends_on_task_id: payload.depends_on_task_id,
    };
    (StatusCode::CREATED, Json(dependency))
}

pub async fn delete_task_dependency(
    Path((task_id, depends_on_task_id)): Path<(i64, i64)>,
) -> StatusCode {
    // TODO: delete dependency record.
    let _ = (task_id, depends_on_task_id);
    StatusCode::NO_CONTENT
}

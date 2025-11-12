use crate::db;
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
    let deps = db::task_dependency::list_task_dependencies(query.task_id).await;
    Json(deps)
}

pub async fn create_task_dependency(
    Json(payload): Json<TaskDependencyCreateInput>,
) -> (StatusCode, Json<TaskDependency>) {
    let dependency = db::task_dependency::create_task_dependency(payload).await;
    (StatusCode::CREATED, Json(dependency))
}

pub async fn delete_task_dependency(
    Path((task_id, depends_on_task_id)): Path<(i64, i64)>,
) -> StatusCode {
    db::task_dependency::delete_task_dependency(task_id, depends_on_task_id).await;
    StatusCode::NO_CONTENT
}

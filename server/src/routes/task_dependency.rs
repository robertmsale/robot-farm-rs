use crate::db;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use openapi::models::{TaskDependency, TaskDependencyCreateInput};
use serde::Deserialize;
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct TaskDependencyQuery {
    pub task_id: i64,
}

pub async fn list_task_dependencies(
    Query(query): Query<TaskDependencyQuery>,
) -> Result<Json<Vec<i64>>, StatusCode> {
    let deps = db::task_dependency::list_task_dependencies(query.task_id)
        .await
        .map_err(|err| {
            error!(
                ?err,
                task_id = query.task_id,
                "failed to list task dependencies"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(deps))
}

pub async fn create_task_dependency(
    Json(payload): Json<TaskDependencyCreateInput>,
) -> Result<(StatusCode, Json<TaskDependency>), StatusCode> {
    let dependency = db::task_dependency::create_task_dependency(payload)
        .await
        .map_err(|err| {
            error!(?err, "failed to create task dependency");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok((StatusCode::CREATED, Json(dependency)))
}

pub async fn delete_task_dependency(
    Path((task_id, depends_on_task_id)): Path<(i64, i64)>,
) -> Result<StatusCode, StatusCode> {
    let deleted = db::task_dependency::delete_task_dependency(task_id, depends_on_task_id)
        .await
        .map_err(|err| {
            error!(
                ?err,
                task_id, depends_on_task_id, "failed to delete task dependency"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

use crate::db;
use axum::{Json, extract::Path, http::StatusCode};
use openapi::models::{TaskGroup, TaskGroupCreateInput, TaskGroupUpdateInput};
use tracing::error;

pub async fn list_task_groups() -> Result<Json<Vec<TaskGroup>>, StatusCode> {
    let groups = db::task_group::list_task_groups().await.map_err(|err| {
        error!(?err, "failed to list task groups");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(groups))
}

pub async fn create_task_group(
    Json(payload): Json<TaskGroupCreateInput>,
) -> Result<(StatusCode, Json<TaskGroup>), StatusCode> {
    let task_group = db::task_group::create_task_group(payload)
        .await
        .map_err(|err| {
            error!(?err, "failed to create task group");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok((StatusCode::CREATED, Json(task_group)))
}

pub async fn get_task_group(Path(task_group_id): Path<i64>) -> Result<Json<TaskGroup>, StatusCode> {
    let group = db::task_group::get_task_group(task_group_id)
        .await
        .map_err(|err| {
            error!(?err, task_group_id, "failed to load task group");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(group))
}

pub async fn update_task_group(
    Path(task_group_id): Path<i64>,
    Json(payload): Json<TaskGroupUpdateInput>,
) -> Result<Json<TaskGroup>, StatusCode> {
    let group = db::task_group::update_task_group(task_group_id, payload)
        .await
        .map_err(|err| {
            error!(?err, task_group_id, "failed to update task group");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(group))
}

pub async fn delete_task_group(Path(task_group_id): Path<i64>) -> Result<StatusCode, StatusCode> {
    let deleted = db::task_group::delete_task_group(task_group_id)
        .await
        .map_err(|err| {
            error!(?err, task_group_id, "failed to delete task group");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

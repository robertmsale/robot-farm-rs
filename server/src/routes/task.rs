use crate::db;
use crate::db::task_group;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use openapi::models::{Task, TaskCreateInput, TaskStatus, TaskUpdateInput};
use serde::Deserialize;
use tracing::error;

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize)]
pub struct TaskListQuery {
    pub group_slug: Option<String>,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub commit_hash: Option<String>,
    pub status: Option<TaskStatus>,
    pub owner: Option<String>,
}

pub async fn list_tasks(
    Query(_query): Query<TaskListQuery>,
) -> Result<Json<Vec<Task>>, StatusCode> {
    // TODO: pass filters into db layer.
    let tasks = db::task::list_tasks().await.map_err(|err| {
        error!(?err, "failed to list tasks");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(tasks))
}

pub async fn create_task(
    Json(payload): Json<TaskCreateInput>,
) -> Result<(StatusCode, Json<Task>), StatusCode> {
    task_group::get_task_group(payload.group_id)
        .await
        .map_err(|err| {
            error!(
                ?err,
                group_id = payload.group_id,
                "failed to verify task group"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or_else(|| {
            error!(
                group_id = payload.group_id,
                "attempted to create task for unknown group"
            );
            StatusCode::BAD_REQUEST
        })?;

    let task = db::task::create_task(payload).await.map_err(|err| {
        error!(?err, "failed to create task");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn get_task(Path(task_id): Path<i64>) -> Result<Json<Task>, StatusCode> {
    let task = db::task::get_task(task_id)
        .await
        .map_err(|err| {
            error!(?err, task_id, "failed to load task");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(task))
}

pub async fn update_task(
    Path(task_id): Path<i64>,
    Json(payload): Json<TaskUpdateInput>,
) -> Result<Json<Task>, StatusCode> {
    let task = db::task::update_task(task_id, payload)
        .await
        .map_err(|err| {
            error!(?err, task_id, "failed to update task");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(task))
}

pub async fn delete_task(Path(task_id): Path<i64>) -> Result<StatusCode, StatusCode> {
    let deleted = db::task::delete_task(task_id).await.map_err(|err| {
        error!(?err, task_id, "failed to delete task");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

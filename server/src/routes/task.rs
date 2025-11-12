use crate::db;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use openapi::models::{Task, TaskCreateInput, TaskStatus, TaskUpdateInput};
use serde::Deserialize;

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

pub async fn list_tasks(Query(_query): Query<TaskListQuery>) -> Json<Vec<Task>> {
    // TODO: pass filters into db layer.
    let tasks = db::task::list_tasks().await;
    Json(tasks)
}

pub async fn create_task(Json(payload): Json<TaskCreateInput>) -> (StatusCode, Json<Task>) {
    let task = db::task::create_task(payload).await;
    (StatusCode::CREATED, Json(task))
}

pub async fn get_task(Path(task_id): Path<i64>) -> Json<Task> {
    let task = db::task::get_task(task_id).await.unwrap_or_else(|| Task {
        id: task_id,
        group_id: 0,
        slug: "".to_string(),
        title: "".to_string(),
        commit_hash: None,
        status: TaskStatus::Ready,
        owner: "".to_string(),
    });
    Json(task)
}

pub async fn update_task(
    Path(task_id): Path<i64>,
    Json(payload): Json<TaskUpdateInput>,
) -> Json<Task> {
    let task = db::task::update_task(task_id, payload)
        .await
        .unwrap_or_else(|| Task {
            id: task_id,
            group_id: 0,
            slug: "".to_string(),
            title: "".to_string(),
            commit_hash: None,
            status: TaskStatus::Ready,
            owner: "".to_string(),
        });
    Json(task)
}

pub async fn delete_task(Path(_task_id): Path<i64>) -> StatusCode {
    db::task::delete_task(_task_id).await;
    StatusCode::NO_CONTENT
}

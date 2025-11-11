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

fn sample_task(id: i64) -> Task {
    Task {
        id,
        group_id: 1,
        slug: format!("task-{id}"),
        title: format!("Task {id}"),
        commit_hash: Some("abc123".to_string()),
        status: TaskStatus::Ready,
        owner: "Orchestrator".to_string(),
    }
}

pub async fn list_tasks(Query(_query): Query<TaskListQuery>) -> Json<Vec<Task>> {
    // TODO: replace with database-backed task listing.
    Json(vec![sample_task(1)])
}

pub async fn create_task(Json(payload): Json<TaskCreateInput>) -> (StatusCode, Json<Task>) {
    // TODO: persist the task and return the created record.
    let task = Task {
        id: 0,
        group_id: payload.group_id,
        slug: payload.slug,
        title: payload.title,
        commit_hash: payload.commit_hash,
        status: payload.status,
        owner: payload.owner,
    };

    (StatusCode::CREATED, Json(task))
}

pub async fn get_task(Path(task_id): Path<i64>) -> Json<Task> {
    // TODO: fetch the task from storage.
    Json(sample_task(task_id))
}

pub async fn update_task(
    Path(task_id): Path<i64>,
    Json(_payload): Json<TaskUpdateInput>,
) -> Json<Task> {
    // TODO: apply update payload to the task.
    Json(sample_task(task_id))
}

pub async fn delete_task(Path(_task_id): Path<i64>) -> StatusCode {
    // TODO: delete the task from storage.
    StatusCode::NO_CONTENT
}

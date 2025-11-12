use crate::db;
use axum::{Json, extract::Path, http::StatusCode};
use openapi::models::{TaskGroup, TaskGroupCreateInput, TaskGroupStatus, TaskGroupUpdateInput};

pub async fn list_task_groups() -> Json<Vec<TaskGroup>> {
    let groups = db::task_group::list_task_groups().await;
    Json(groups)
}

pub async fn create_task_group(
    Json(payload): Json<TaskGroupCreateInput>,
) -> (StatusCode, Json<TaskGroup>) {
    let task_group = db::task_group::create_task_group(payload).await;
    (StatusCode::CREATED, Json(task_group))
}

pub async fn get_task_group(Path(task_group_id): Path<i64>) -> Json<TaskGroup> {
    let group = db::task_group::get_task_group(task_group_id)
        .await
        .unwrap_or_else(|| TaskGroup {
            id: task_group_id,
            slug: "".to_string(),
            title: "".to_string(),
            description: "".to_string(),
            status: TaskGroupStatus::Ready,
        });
    Json(group)
}

pub async fn update_task_group(
    Path(task_group_id): Path<i64>,
    Json(payload): Json<TaskGroupUpdateInput>,
) -> Json<TaskGroup> {
    let group = db::task_group::update_task_group(task_group_id, payload)
        .await
        .unwrap_or_else(|| TaskGroup {
            id: task_group_id,
            slug: "".to_string(),
            title: "".to_string(),
            description: "".to_string(),
            status: TaskGroupStatus::Ready,
        });
    Json(group)
}

pub async fn delete_task_group(Path(_task_group_id): Path<i64>) -> StatusCode {
    db::task_group::delete_task_group(_task_group_id).await;
    StatusCode::NO_CONTENT
}

use axum::{Json, extract::Path, http::StatusCode};
use openapi::models::{TaskGroup, TaskGroupCreateInput, TaskGroupStatus, TaskGroupUpdateInput};

fn sample_task_group(id: i64) -> TaskGroup {
    TaskGroup {
        id,
        slug: format!("group-{id}"),
        title: format!("Task Group {id}"),
        description: "Sample task group".to_string(),
        status: TaskGroupStatus::Ready,
    }
}

pub async fn list_task_groups() -> Json<Vec<TaskGroup>> {
    // TODO: load task groups from storage.
    Json(vec![sample_task_group(1)])
}

pub async fn create_task_group(
    Json(payload): Json<TaskGroupCreateInput>,
) -> (StatusCode, Json<TaskGroup>) {
    // TODO: persist task group and return created record.
    let task_group = TaskGroup {
        id: 0,
        slug: payload.slug,
        title: payload.title,
        description: payload.description,
        status: TaskGroupStatus::Ready,
    };

    (StatusCode::CREATED, Json(task_group))
}

pub async fn get_task_group(Path(task_group_id): Path<i64>) -> Json<TaskGroup> {
    // TODO: fetch task group from storage.
    Json(sample_task_group(task_group_id))
}

pub async fn update_task_group(
    Path(task_group_id): Path<i64>,
    Json(_payload): Json<TaskGroupUpdateInput>,
) -> Json<TaskGroup> {
    // TODO: apply updates to the task group.
    Json(sample_task_group(task_group_id))
}

pub async fn delete_task_group(Path(_task_group_id): Path<i64>) -> StatusCode {
    // TODO: delete task group from storage.
    StatusCode::NO_CONTENT
}

use openapi::models::{Task, TaskCreateInput, TaskStatus, TaskUpdateInput};

fn sample_task(id: i64) -> Task {
    Task {
        id,
        group_id: 1,
        slug: format!("task-{id}"),
        title: format!("Task {id}"),
        commit_hash: Some("HEAD".to_string()),
        status: TaskStatus::Ready,
        owner: "Orchestrator".to_string(),
    }
}

pub async fn list_tasks() -> Vec<Task> {
    // TODO: replace with SELECT query.
    vec![sample_task(1)]
}

pub async fn create_task(payload: TaskCreateInput) -> Task {
    // TODO: insert into database.
    Task {
        id: 0,
        group_id: payload.group_id,
        slug: payload.slug,
        title: payload.title,
        commit_hash: payload.commit_hash,
        status: payload.status,
        owner: payload.owner,
    }
}

pub async fn get_task(task_id: i64) -> Option<Task> {
    // TODO: fetch from database.
    Some(sample_task(task_id))
}

pub async fn update_task(task_id: i64, _payload: TaskUpdateInput) -> Option<Task> {
    // TODO: apply update to database.
    Some(sample_task(task_id))
}

pub async fn delete_task(_task_id: i64) -> bool {
    // TODO: delete row from database.
    true
}

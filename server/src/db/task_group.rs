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

pub async fn list_task_groups() -> Vec<TaskGroup> {
    // TODO: replace with SELECT query.
    vec![sample_task_group(1)]
}

pub async fn create_task_group(payload: TaskGroupCreateInput) -> TaskGroup {
    // TODO: insert into database.
    TaskGroup {
        id: 0,
        slug: payload.slug,
        title: payload.title,
        description: payload.description,
        status: TaskGroupStatus::Ready,
    }
}

pub async fn get_task_group(task_group_id: i64) -> Option<TaskGroup> {
    // TODO: fetch from database.
    Some(sample_task_group(task_group_id))
}

pub async fn update_task_group(
    task_group_id: i64,
    _payload: TaskGroupUpdateInput,
) -> Option<TaskGroup> {
    // TODO: apply updates in database.
    Some(sample_task_group(task_group_id))
}

pub async fn delete_task_group(_task_group_id: i64) -> bool {
    // TODO: delete row from database.
    true
}

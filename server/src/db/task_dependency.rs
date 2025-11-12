use openapi::models::{TaskDependency, TaskDependencyCreateInput};

pub async fn list_task_dependencies(task_id: i64) -> Vec<i64> {
    // TODO: SELECT depends_on_task_id FROM task_deps WHERE task_id = $1.
    if task_id == 0 { vec![] } else { vec![1, 2, 3] }
}

pub async fn create_task_dependency(payload: TaskDependencyCreateInput) -> TaskDependency {
    // TODO: INSERT into task_deps.
    TaskDependency {
        task_id: payload.task_id,
        depends_on_task_id: payload.depends_on_task_id,
    }
}

pub async fn delete_task_dependency(_task_id: i64, _depends_on_task_id: i64) -> bool {
    // TODO: DELETE FROM task_deps.
    true
}


enum TaskGroupStatus {
    ready, // "Ready"
    done,  // "Done"
}

pub struct TaskGroup {
    id: i64,
    slug: String,
    title: String,
    description: String,
    status: TaskGroupStatus, // not updatable, if all tasks in group are "Done", otherwise "Ready" 
}


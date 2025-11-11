pub enum TaskStatus {
    ready,
    blocked,
    done,
}

impl TaskStatus {
    fn as_str(&self) -> &str {
        match self {
            TaskStatus::ready => "Ready",
            TaskStatus::blocked => "Blocked",
            TaskStatus::done => "Done",
        }
    }
}

pub enum Owner {
    orchestrator,
    qa,
    worker(i64),
}

impl Owner {
    fn as_str(&self) -> &str {
        match self {
            Owner::orchestrator => "Orchestrator",
            Owner::qa => "Quality Assurance",
            Owner::worker(id) => format!("ws{}", id),
        }
    }
}

pub struct Task {
    id: i64,
    group_id: i64,
    slug: String,
    title: String,
    status: TaskStatus, 
    owner: Owner,
}



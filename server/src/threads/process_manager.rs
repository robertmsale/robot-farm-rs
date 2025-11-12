

#[derive(Debug, Clone)]
pub enum CommandMsg {
    SpawnWorker {
        worker_id: String,
        image: String,
        args: Vec<String>,
    },
    KillWorker {
        worker_id: String,
    },
    WorkerCompleted {
        worker_id: String,
        task_id: String,
        success: bool,
        output: String,
    }
}


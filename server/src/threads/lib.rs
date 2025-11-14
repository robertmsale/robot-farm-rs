pub mod database_manager;
pub mod middleware;
pub mod process_manager;
pub mod queue_manager;

pub use database_manager::{DatabaseManagerConfig, DatabaseManagerHandle, spawn_database_manager};
pub use middleware::{MiddlewareConfig, MiddlewareHandle, spawn_middleware};
pub use process_manager::{ProcessManagerConfig, ProcessManagerRuntime, spawn_process_manager};
pub use queue_manager::{QueueManagerConfig, QueueManagerHandle, spawn_queue_manager};

use std::sync::OnceLock;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct ThreadHandles {
    pub middleware: MiddlewareHandle,
    pub database: DatabaseManagerHandle,
    pub queue: QueueManagerHandle,
}

static THREAD_HANDLES: OnceLock<ThreadHandles> = OnceLock::new();

pub fn init_background_threads() -> &'static ThreadHandles {
    THREAD_HANDLES.get_or_init(|| spawn_threads())
}

pub fn thread_handles() -> &'static ThreadHandles {
    THREAD_HANDLES
        .get()
        .expect("background threads not initialized")
}

fn spawn_threads() -> ThreadHandles {
    let middleware_config = MiddlewareConfig::default();
    let manager_config = ProcessManagerConfig::default();
    let database_config = DatabaseManagerConfig::default();
    let queue_config = QueueManagerConfig::default();
    let (lifecycle_tx, lifecycle_rx) = mpsc::channel(256);
    let (notification_tx, notification_rx) = mpsc::channel(128);
    let (middleware_handle, directives_rx) = spawn_middleware(middleware_config, lifecycle_rx);
    let runtime = ProcessManagerRuntime {
        directives_rx,
        config: manager_config,
        lifecycle_tx,
        notifications_tx: notification_tx,
    };
    spawn_process_manager(runtime);
    let database_handle = spawn_database_manager(database_config);
    let queue_handle = spawn_queue_manager(
        queue_config,
        database_handle.clone(),
        middleware_handle.clone(),
        notification_rx,
    );

    ThreadHandles {
        middleware: middleware_handle,
        database: database_handle,
        queue: queue_handle,
    }
}

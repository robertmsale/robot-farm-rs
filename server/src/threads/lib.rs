pub mod database_manager;
pub mod middleware;
pub mod process_manager;
pub mod queue_manager;

pub use middleware::{MiddlewareConfig, MiddlewareHandle, spawn_middleware};
pub use process_manager::{ProcessManagerConfig, ProcessManagerRuntime, spawn_process_manager};

use std::sync::OnceLock;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct ThreadHandles {
    pub middleware: MiddlewareHandle,
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
    let (lifecycle_tx, lifecycle_rx) = mpsc::channel(256);
    let (middleware_handle, directives_rx) = spawn_middleware(middleware_config, lifecycle_rx);
    let runtime = ProcessManagerRuntime {
        directives_rx,
        config: manager_config,
        lifecycle_tx,
    };
    spawn_process_manager(runtime);

    ThreadHandles {
        middleware: middleware_handle,
    }
}

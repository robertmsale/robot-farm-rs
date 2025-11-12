use std::path::PathBuf;
use std::time::Duration;

use chrono::Utc;
use server::models::process::{
    KillReason, ProcessEvent, ProcessSpawnIntent, ProcessStream, RunMetadata, RunPriority,
};
use server::threads::{
    MiddlewareConfig, MiddlewareHandle, ProcessManagerConfig, ProcessManagerRuntime,
    spawn_middleware, spawn_process_manager,
};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::timeout;
use uuid::Uuid;

struct TestHarness {
    middleware: Option<MiddlewareHandle>,
    manager_task: Option<JoinHandle<()>>,
}

impl TestHarness {
    fn new() -> Self {
        let middleware_config = MiddlewareConfig {
            batch_window: Duration::from_millis(50),
            max_batch: 8,
            intent_buffer: 16,
            directive_buffer: 16,
        };
        let manager_config = ProcessManagerConfig {
            event_buffer: 32,
            kill_buffer: 4,
        };
        let (lifecycle_tx, lifecycle_rx) = mpsc::channel(32);
        let (middleware_handle, directives_rx) = spawn_middleware(middleware_config, lifecycle_rx);
        let runtime = ProcessManagerRuntime {
            directives_rx,
            config: manager_config,
            lifecycle_tx,
        };
        let manager_task = spawn_process_manager(runtime);

        Self {
            middleware: Some(middleware_handle),
            manager_task: Some(manager_task),
        }
    }

    fn middleware(&self) -> &MiddlewareHandle {
        self.middleware
            .as_ref()
            .expect("middleware handle should be set")
    }

    async fn shutdown(mut self) {
        self.middleware.take();
        if let Some(task) = self.manager_task.take() {
            task.abort();
            let _ = task.await;
        }
    }
}

impl Drop for TestHarness {
    fn drop(&mut self) {
        if let Some(task) = &self.manager_task {
            task.abort();
        }
    }
}

fn build_spawn_intent(command: &str) -> ProcessSpawnIntent {
    let run_id = Uuid::new_v4();
    let metadata = RunMetadata {
        run_id,
        persona: "test".into(),
        workspace_root: PathBuf::from("."),
        tags: vec![],
        priority: RunPriority::Normal,
        issued_at: Utc::now(),
    };

    ProcessSpawnIntent {
        metadata,
        program: "bash".into(),
        args: vec!["-lc".into(), command.into()],
        env: Vec::new(),
        working_dir: std::env::current_dir().expect("current dir"),
        stream_stdout: true,
        stream_stderr: true,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn process_pipeline_streams_output_and_exits() {
    let harness = TestHarness::new();
    let spawn_intent = build_spawn_intent("printf robot-farm");
    let handle_rx = harness
        .middleware()
        .enqueue_spawn(spawn_intent)
        .await
        .expect("spawn intent send");
    let mut handle = handle_rx.await.expect("process handle");

    let (exited_ok, stdout_buf) = timeout(Duration::from_secs(5), async {
        let mut stdout_bytes = Vec::new();

        while let Some(event) = handle.events.recv().await {
            match event {
                ProcessEvent::Output(chunk) if chunk.stream == ProcessStream::Stdout => {
                    stdout_bytes.extend_from_slice(&chunk.bytes);
                }
                ProcessEvent::Exit(exit) => return (exit.status.success(), stdout_bytes),
                ProcessEvent::Killed(killed) => {
                    panic!("process unexpectedly killed: {:?}", killed.reason)
                }
                _ => {}
            }
        }

        (false, stdout_bytes)
    })
    .await
    .expect("process did not complete in time");

    let stdout = String::from_utf8(stdout_buf).expect("stdout utf8");
    assert!(exited_ok, "process should exit successfully");
    assert!(
        stdout.contains("robot-farm"),
        "expected stdout to contain command output"
    );

    harness.shutdown().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn process_pipeline_can_cancel_long_running_command() {
    let harness = TestHarness::new();
    let spawn_intent = build_spawn_intent("sleep 5");
    let handle_rx = harness
        .middleware()
        .enqueue_spawn(spawn_intent)
        .await
        .expect("spawn intent send");
    let mut handle = handle_rx.await.expect("process handle");

    // Ensure process starts before we send kill.
    tokio::time::sleep(Duration::from_millis(100)).await;
    handle
        .kill
        .kill(KillReason::UserRequested)
        .await
        .expect("deliver kill signal");

    let killed_reason = timeout(Duration::from_secs(5), async {
        while let Some(event) = handle.events.recv().await {
            if let ProcessEvent::Killed(killed) = event {
                return killed.reason;
            }
        }
        panic!("expected killed event");
    })
    .await
    .expect("kill timed out");

    match killed_reason {
        KillReason::UserRequested => {}
        other => panic!("unexpected kill reason: {:?}", other),
    }

    harness.shutdown().await;
}

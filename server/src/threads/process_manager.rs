use crate::models::process::{
    KillReason, KillSignal, ProcessDirective, ProcessEvent, ProcessExit, ProcessHandle,
    ProcessKillDirective, ProcessKillHandle, ProcessKilled, ProcessLaunchDirective,
    ProcessLifecycleEvent, ProcessOutputChunk, ProcessOutputError, ProcessRequest,
    ProcessSpawnError, ProcessStream, RunId,
};
use chrono::Utc;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::{debug, error, warn};

pub struct ProcessManagerConfig {
    pub event_buffer: usize,
    pub kill_buffer: usize,
}

impl Default for ProcessManagerConfig {
    fn default() -> Self {
        Self {
            event_buffer: 128,
            kill_buffer: 4,
        }
    }
}

pub struct ProcessManagerRuntime {
    pub directives_rx: mpsc::Receiver<ProcessDirective>,
    pub config: ProcessManagerConfig,
    pub lifecycle_tx: mpsc::Sender<ProcessLifecycleEvent>,
}

struct ActiveProcess {
    kill_tx: mpsc::Sender<KillSignal>,
}

pub fn spawn_process_manager(runtime: ProcessManagerRuntime) -> JoinHandle<()> {
    tokio::spawn(async move {
        run_manager(runtime).await;
    })
}

async fn run_manager(mut runtime: ProcessManagerRuntime) {
    let (cleanup_tx, mut cleanup_rx) = mpsc::channel(64);
    let mut active: HashMap<RunId, ActiveProcess> = HashMap::new();
    let mut directives_closed = false;

    loop {
        tokio::select! {
            Some(run_id) = cleanup_rx.recv() => {
                debug!(%run_id, "process completed, removing from active map");
                active.remove(&run_id);
            }
            directive = runtime.directives_rx.recv(), if !directives_closed => {
                match directive {
                    Some(directive) => {
                        handle_directive(
                            directive,
                            &runtime.config,
                            &mut active,
                            cleanup_tx.clone(),
                            runtime.lifecycle_tx.clone(),
                        )
                        .await
                    }
                    None => {
                        directives_closed = true;
                    }
                }
            }
            else => break,
        }

        if directives_closed && active.is_empty() {
            break;
        }
    }

    // best-effort shutdown for anything still alive
    for (run_id, process) in active.into_iter() {
        warn!(%run_id, "forcing shutdown for leftover process");
        let _ = process.kill_tx.try_send(KillSignal {
            reason: KillReason::Shutdown,
            requested_at: Utc::now(),
        });
    }
}

async fn handle_directive(
    directive: ProcessDirective,
    config: &ProcessManagerConfig,
    active: &mut HashMap<RunId, ActiveProcess>,
    cleanup_tx: mpsc::Sender<RunId>,
    lifecycle_tx: mpsc::Sender<ProcessLifecycleEvent>,
) {
    match directive {
        ProcessDirective::Launch(launch) => {
            launch_process(launch, config, active, cleanup_tx, lifecycle_tx).await;
        }
        ProcessDirective::Kill(kill) => {
            kill_process(kill, active).await;
        }
        ProcessDirective::UpdatePriority(update) => {
            if let Some(state) = active.get(&update.run_id) {
                debug!(
                    run_id = %update.run_id,
                    priority = ?update.new_priority,
                    "priority update noted for running process"
                );
                let _ = state; // reserved for future scheduling hooks
            } else {
                debug!(
                    run_id = %update.run_id,
                    "priority update ignored because process not active"
                );
            }
        }
    }
}

async fn launch_process(
    directive: ProcessLaunchDirective,
    config: &ProcessManagerConfig,
    active: &mut HashMap<RunId, ActiveProcess>,
    cleanup_tx: mpsc::Sender<RunId>,
    lifecycle_tx: mpsc::Sender<ProcessLifecycleEvent>,
) {
    let run_id = directive.request.metadata.run_id;

    if active.contains_key(&run_id) {
        warn!(%run_id, "process already active, replacing existing entry");
    }

    let (events_tx, events_rx) = mpsc::channel(config.event_buffer);
    let (kill_tx, kill_rx) = mpsc::channel(config.kill_buffer);
    let kill_handle = ProcessKillHandle::new(kill_tx.clone());

    let handle = ProcessHandle {
        run_id,
        events: events_rx,
        kill: kill_handle,
    };

    if directive.handle_tx.send(handle).is_err() {
        warn!(%run_id, "caller dropped handle receiver before launch");
    }

    let cleanup = cleanup_tx.clone();
    let request = directive.request;
    tokio::spawn(async move {
        run_child(request, events_tx, kill_rx, lifecycle_tx).await;
        let _ = cleanup.send(run_id).await;
    });

    active.insert(run_id, ActiveProcess { kill_tx });
}

async fn kill_process(directive: ProcessKillDirective, active: &mut HashMap<RunId, ActiveProcess>) {
    if let Some(process) = active.get(&directive.run_id) {
        if process
            .kill_tx
            .send(KillSignal {
                reason: directive.reason,
                requested_at: Utc::now(),
            })
            .await
            .is_err()
        {
            warn!(run_id = %directive.run_id, "failed to deliver kill signal");
        }
    } else {
        debug!(
            run_id = %directive.run_id,
            "kill directive ignored because process not active"
        );
    }
}

async fn run_child(
    request: ProcessRequest,
    events_tx: mpsc::Sender<ProcessEvent>,
    kill_rx: mpsc::Receiver<KillSignal>,
    lifecycle_tx: mpsc::Sender<ProcessLifecycleEvent>,
) {
    let run_id = request.metadata.run_id;
    let stdin_payload = request.stdin.clone();
    let mut command = build_command(&request);

    match command.spawn() {
        Ok(mut child) => {
            let _ = lifecycle_tx
                .send(ProcessLifecycleEvent::Starting { run_id })
                .await;

            if let Some(payload) = stdin_payload {
                if let Some(mut stdin) = child.stdin.take() {
                    tokio::spawn(async move {
                        if stdin.write_all(&payload).await.is_err() {
                            warn!(%run_id, "failed to write wizard prompt to child stdin");
                        }
                        let _ = stdin.shutdown().await;
                    });
                }
            }

            if request.stream_stdout {
                if let Some(stdout) = child.stdout.take() {
                    let tx = events_tx.clone();
                    tokio::spawn(forward_output(stdout, tx, run_id, ProcessStream::Stdout));
                }
            }

            if request.stream_stderr {
                if let Some(stderr) = child.stderr.take() {
                    let tx = events_tx.clone();
                    tokio::spawn(forward_output(stderr, tx, run_id, ProcessStream::Stderr));
                }
            }

            observe_child(child, run_id, events_tx, kill_rx, lifecycle_tx).await;
        }
        Err(err) => {
            let _ = events_tx
                .send(ProcessEvent::SpawnFailed(ProcessSpawnError {
                    run_id,
                    message: err.to_string(),
                    occurred_at: Utc::now(),
                }))
                .await;
            let _ = lifecycle_tx
                .send(ProcessLifecycleEvent::Failed { run_id })
                .await;
        }
    }
}

fn build_command(request: &ProcessRequest) -> Command {
    let mut command = Command::new(&request.program);
    command.args(&request.args);
    command.current_dir(&request.working_dir);
    for (key, value) in &request.env {
        command.env(key, value);
    }

    if request.stdin.is_some() {
        command.stdin(Stdio::piped());
    } else {
        command.stdin(Stdio::null());
    }

    if request.stream_stdout {
        command.stdout(Stdio::piped());
    } else {
        command.stdout(Stdio::null());
    }

    if request.stream_stderr {
        command.stderr(Stdio::piped());
    } else {
        command.stderr(Stdio::null());
    }

    command
}

async fn observe_child(
    mut child: tokio::process::Child,
    run_id: RunId,
    events_tx: mpsc::Sender<ProcessEvent>,
    mut kill_rx: mpsc::Receiver<KillSignal>,
    lifecycle_tx: mpsc::Sender<ProcessLifecycleEvent>,
) {
    let mut kill_reason: Option<KillReason> = None;

    loop {
        tokio::select! {
            Some(signal) = kill_rx.recv() => {
                kill_reason = Some(signal.reason.clone());
                if let Err(err) = child.start_kill() {
                    error!(%run_id, error = ?err, "failed to start kill");
                }
            }
            status = child.wait() => {
                match status {
                    Ok(exit) => {
                        if let Some(reason) = kill_reason {
                            let finished_at = Utc::now();
                            let _ = events_tx
                                .send(ProcessEvent::Killed(ProcessKilled {
                                    run_id,
                                    reason: reason.clone(),
                                    finished_at,
                                }))
                                .await;
                            let _ = lifecycle_tx
                                .send(ProcessLifecycleEvent::Killed { run_id })
                                .await;
                        } else {
                            let _ = events_tx
                                .send(ProcessEvent::Exit(ProcessExit {
                                    run_id,
                                    status: exit,
                                    finished_at: Utc::now(),
                                }))
                                .await;
                            let _ = lifecycle_tx
                                .send(ProcessLifecycleEvent::Finished { run_id })
                                .await;
                        }
                    }
                    Err(err) => {
                        error!(%run_id, error = ?err, "failed to await child exit");
                        let _ = events_tx
                            .send(ProcessEvent::SpawnFailed(ProcessSpawnError {
                                run_id,
                                message: err.to_string(),
                                occurred_at: Utc::now(),
                            }))
                            .await;
                        let _ = lifecycle_tx
                            .send(ProcessLifecycleEvent::Failed { run_id })
                            .await;
                    }
                }
                break;
            }
        }
    }
}

async fn forward_output<R>(
    mut reader: R,
    events_tx: mpsc::Sender<ProcessEvent>,
    run_id: RunId,
    stream: ProcessStream,
) where
    R: AsyncRead + Unpin + Send + 'static,
{
    let mut buffer = vec![0_u8; 8192];
    let mut sequence = 0_u64;

    loop {
        match reader.read(&mut buffer).await {
            Ok(0) => break,
            Ok(n) => {
                let chunk = ProcessOutputChunk {
                    run_id,
                    stream,
                    bytes: buffer[..n].to_vec(),
                    sequence,
                    captured_at: Utc::now(),
                };

                if events_tx.send(ProcessEvent::Output(chunk)).await.is_err() {
                    break;
                }

                sequence += 1;
            }
            Err(err) => {
                let error = ProcessOutputError {
                    run_id,
                    stream,
                    message: err.to_string(),
                    occurred_at: Utc::now(),
                };

                let _ = events_tx.send(ProcessEvent::OutputError(error)).await;
                break;
            }
        }
    }
}

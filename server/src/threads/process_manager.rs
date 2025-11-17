use crate::ai::schemas::WorkerTurn;
use crate::models::codex_events::{CodexEvent, TurnItemDetail};
use crate::models::process::{
    KillReason, KillSignal, ProcessDirective, ProcessEvent, ProcessExit, ProcessHandle,
    ProcessKillDirective, ProcessKillHandle, ProcessKilled, ProcessLaunchDirective,
    ProcessLifecycleEvent, ProcessOutputChunk, ProcessOutputError, ProcessRequest,
    ProcessSpawnError, ProcessStream, RunId, RunMetadata,
};
use chrono::Utc;
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::sync::{Mutex, mpsc};
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn};

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
    pub notifications_tx: mpsc::Sender<ProcessNotification>,
}

#[derive(Debug, Clone)]
pub enum ProcessNotification {
    WorkerTurn {
        run_id: RunId,
        worker_id: i64,
        metadata: RunMetadata,
        turn: WorkerTurn,
    },
    WorkerFeed {
        worker_id: i64,
        message: String,
        raw: String,
        thread_id: Option<String>,
    },
}

struct ActiveProcess {
    kill_tx: mpsc::Sender<KillSignal>,
}

struct WorkerObserver {
    worker_id: i64,
    metadata: RunMetadata,
    collector: Arc<Mutex<StructuredOutputCollector>>,
}

#[derive(Clone)]
struct WorkerFeedContext {
    worker_id: i64,
    notifications_tx: mpsc::Sender<ProcessNotification>,
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
    info!("process manager loop started");

    loop {
        tokio::select! {
            Some(run_id) = cleanup_rx.recv() => {
                info!(%run_id, "process completed, removing from active map");
                active.remove(&run_id);
            }
            directive = runtime.directives_rx.recv(), if !directives_closed => {
                match directive {
                    Some(directive) => {
                        info!(?directive, "process manager received directive");
                        handle_directive(
                            directive,
                            &runtime.config,
                            &mut active,
                            cleanup_tx.clone(),
                            runtime.lifecycle_tx.clone(),
                            runtime.notifications_tx.clone(),
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
    notifications_tx: mpsc::Sender<ProcessNotification>,
) {
    match directive {
        ProcessDirective::Launch(launch) => {
            info!(run_id = %launch.request.metadata.run_id, persona = ?launch.request.metadata.persona, "process manager launching worker run");
            launch_process(
                launch,
                config,
                active,
                cleanup_tx,
                lifecycle_tx,
                notifications_tx,
            )
            .await;
        }
        ProcessDirective::Kill(kill) => {
            info!(run_id = %kill.run_id, "process manager received kill directive");
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
    notifications_tx: mpsc::Sender<ProcessNotification>,
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
        run_child(request, events_tx, kill_rx, lifecycle_tx, notifications_tx).await;
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
    notifications_tx: mpsc::Sender<ProcessNotification>,
) {
    let run_id = request.metadata.run_id;
    let stdin_payload = request.stdin.clone();
    let mut command = build_command(&request);
    let worker_id = parse_worker_id(&request.metadata);
    let worker_collector =
        worker_id.map(|_| Arc::new(Mutex::new(StructuredOutputCollector::default())));
    let worker_feed_ctx = worker_id.map(|id| WorkerFeedContext {
        worker_id: id,
        notifications_tx: notifications_tx.clone(),
    });

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
                    let collector = worker_collector.clone();
                    let feed_ctx = worker_feed_ctx.clone();
                    tokio::spawn(forward_output(
                        stdout,
                        tx,
                        run_id,
                        ProcessStream::Stdout,
                        collector,
                        feed_ctx,
                    ));
                }
            }

            if request.stream_stderr {
                if let Some(stderr) = child.stderr.take() {
                    let tx = events_tx.clone();
                    tokio::spawn(forward_output(
                        stderr,
                        tx,
                        run_id,
                        ProcessStream::Stderr,
                        None,
                        None,
                    ));
                }
            }

            observe_child(
                child,
                run_id,
                events_tx,
                kill_rx,
                lifecycle_tx,
                worker_id.map(|id| WorkerObserver {
                    worker_id: id,
                    metadata: request.metadata.clone(),
                    collector: worker_collector.expect("collector set when worker detected"),
                }),
                notifications_tx.clone(),
            )
            .await;
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

fn parse_worker_id(metadata: &RunMetadata) -> Option<i64> {
    for tag in &metadata.tags {
        if let Some(rest) = tag.strip_prefix("worker:") {
            if let Ok(id) = rest.parse() {
                return Some(id);
            }
        }
    }
    if let Some(rest) = metadata.persona.strip_prefix("worker:") {
        let rest = rest.trim_start_matches("ws");
        return rest.parse().ok();
    }
    None
}

async fn observe_child(
    mut child: tokio::process::Child,
    run_id: RunId,
    events_tx: mpsc::Sender<ProcessEvent>,
    mut kill_rx: mpsc::Receiver<KillSignal>,
    lifecycle_tx: mpsc::Sender<ProcessLifecycleEvent>,
    worker_ctx: Option<WorkerObserver>,
    notifications_tx: mpsc::Sender<ProcessNotification>,
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

    if let Some(observer) = worker_ctx {
        if let Some(turn) = observer.collector.lock().await.take_turn() {
            let notification = ProcessNotification::WorkerTurn {
                run_id,
                worker_id: observer.worker_id,
                metadata: observer.metadata,
                turn,
            };
            if notifications_tx.send(notification).await.is_err() {
                warn!(%run_id, "failed to deliver worker turn notification");
            }
        }
    }
}

#[derive(Default)]
struct StructuredOutputCollector {
    buffer: Vec<u8>,
    turn: Option<WorkerTurn>,
}

impl StructuredOutputCollector {
    fn ingest(&mut self, bytes: &[u8]) -> Vec<WorkerFeedFragment> {
        let mut fragments = Vec::new();
        self.buffer.extend_from_slice(bytes);
        while let Some(pos) = self.buffer.iter().position(|b| *b == b'\n') {
            let mut line = self.buffer.drain(..=pos).collect::<Vec<_>>();
            if let Some(last) = line.last() {
                if *last == b'\n' {
                    line.pop();
                }
            }
            let text = String::from_utf8_lossy(&line).trim().to_string();
            if text.is_empty() {
                continue;
            }
            if let Some(fragment) = self.process_line(&text) {
                fragments.push(fragment);
            }
        }
        fragments
    }

    fn take_turn(&mut self) -> Option<WorkerTurn> {
        if !self.buffer.is_empty() {
            let text = String::from_utf8_lossy(&self.buffer).trim().to_string();
            if !text.is_empty() {
                let _ = self.process_line(&text);
            }
            self.buffer.clear();
        }
        self.turn.take()
    }

    fn process_line(&mut self, line: &str) -> Option<WorkerFeedFragment> {
        let event: CodexEvent = match serde_json::from_str(line) {
            Ok(event) => event,
            Err(_) => return None,
        };
        match event {
            CodexEvent::ItemCompleted { item } => self.fragment_for_detail(item.detail, line),
            CodexEvent::ThreadStarted { thread_id } => Some(WorkerFeedFragment {
                text: format!("Thread started: {thread_id}"),
                raw: line.to_string(),
                thread_id: Some(thread_id),
            }),
            _ => None,
        }
    }

    fn fragment_for_detail(
        &mut self,
        detail: TurnItemDetail,
        raw: &str,
    ) -> Option<WorkerFeedFragment> {
        match detail {
            TurnItemDetail::AgentMessage { text } => {
                if let Ok(turn) = serde_json::from_str::<WorkerTurn>(&text) {
                    self.turn = Some(turn);
                }
                Some(WorkerFeedFragment {
                    text,
                    raw: raw.to_string(),
                    thread_id: None,
                })
            }
            TurnItemDetail::Reasoning { text } => Some(WorkerFeedFragment {
                text: format!("Reasoning:\n{text}"),
                raw: raw.to_string(),
                thread_id: None,
            }),
            TurnItemDetail::CommandExecution(cmd) => {
                let mut summary = format!("Command `{}` {:?}", cmd.command, cmd.status);
                if !cmd.aggregated_output.trim().is_empty() {
                    summary.push_str("\n");
                    summary.push_str(cmd.aggregated_output.trim());
                }
                Some(WorkerFeedFragment {
                    text: summary,
                    raw: raw.to_string(),
                    thread_id: None,
                })
            }
            TurnItemDetail::ItemError { message } => Some(WorkerFeedFragment {
                text: format!("Error: {message}"),
                raw: raw.to_string(),
                thread_id: None,
            }),
            TurnItemDetail::TodoList { items } => {
                let list = items
                    .into_iter()
                    .map(|item| {
                        format!(
                            "- [{}] {}",
                            if item.completed { "x" } else { " " },
                            item.text
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                Some(WorkerFeedFragment {
                    text: format!("TODO List:\n{list}"),
                    raw: raw.to_string(),
                    thread_id: None,
                })
            }
            TurnItemDetail::FileChange(file_change) => {
                let files = file_change
                    .changes
                    .into_iter()
                    .map(|entry| format!("{:?}: {}", entry.kind, entry.path))
                    .collect::<Vec<_>>()
                    .join("\n");
                Some(WorkerFeedFragment {
                    text: format!("File changes:\n{files}"),
                    raw: raw.to_string(),
                    thread_id: None,
                })
            }
            TurnItemDetail::McpToolCall(call) => Some(WorkerFeedFragment {
                text: format!(
                    "MCP tool call {}::{}, status {:?}",
                    call.server, call.tool, call.status
                ),
                raw: raw.to_string(),
                thread_id: None,
            }),
            _ => None,
        }
    }
}

struct WorkerFeedFragment {
    text: String,
    raw: String,
    thread_id: Option<String>,
}

async fn forward_output<R>(
    mut reader: R,
    events_tx: mpsc::Sender<ProcessEvent>,
    run_id: RunId,
    stream: ProcessStream,
    collector: Option<Arc<Mutex<StructuredOutputCollector>>>,
    feed_ctx: Option<WorkerFeedContext>,
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

                if events_tx
                    .send(ProcessEvent::Output(chunk.clone()))
                    .await
                    .is_err()
                {
                    break;
                }

                if let Some(tap) = &collector {
                    if stream == ProcessStream::Stdout {
                        let fragments = {
                            let mut guard = tap.lock().await;
                            guard.ingest(&chunk.bytes)
                        };
                        if let Some(ctx) = feed_ctx.as_ref() {
                            for fragment in fragments {
                                if fragment.text.trim().is_empty() {
                                    continue;
                                }
                                if ctx
                                    .notifications_tx
                                    .send(ProcessNotification::WorkerFeed {
                                        worker_id: ctx.worker_id,
                                        message: fragment.text,
                                        raw: fragment.raw,
                                        thread_id: fragment.thread_id,
                                    })
                                    .await
                                    .is_err()
                                {
                                    break;
                                }
                            }
                        }
                    }
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

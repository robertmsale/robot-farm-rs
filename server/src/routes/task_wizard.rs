use crate::{
    db::feed::{self, NewFeedEntry},
    docker::DOCKER_IMAGE_WIZARD,
    globals::PROJECT_DIR,
    models::process::{
        KillReason, ProcessEvent, ProcessHandle, ProcessKillHandle, ProcessSpawnIntent,
        ProcessStream, RunId, RunMetadata, RunPriority,
    },
    realtime::{self, RealtimeEvent},
    shared::{
        codex_exec::CodexExecBuilder,
        docker::{DockerRunBuilder, ensure_default_mcp_url},
    },
    system::{
        codex_config::{self, AgentKind as CodexAgentKind},
        docker_overrides,
    },
    threads,
};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use chrono::Utc;
use futures::{SinkExt, StreamExt};
use openapi::models::FeedLevel;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use std::{env, path::PathBuf};
use tempfile::TempDir;
use tokio::{sync::mpsc, task::JoinHandle};
use tracing::warn;
use uuid::Uuid;

pub async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    let session_id = Uuid::new_v4();
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut tx_rx) = mpsc::channel::<Message>(64);
    let send_task = tokio::spawn(async move {
        while let Some(message) = tx_rx.recv().await {
            if sender.send(message).await.is_err() {
                break;
            }
        }
    });

    let (internal_tx, mut internal_rx) = mpsc::channel::<InternalEvent>(16);
    let mut state = TaskWizardState::new(session_id);

    send_json(
        &tx,
        json!({
            "type": "welcome",
            "sessionId": session_id.to_string(),
            "threadId": state.thread_id,
            "status": "idle",
        }),
    )
    .await;

    loop {
        tokio::select! {
            Some(event) = internal_rx.recv() => {
                state.handle_internal_event(event, &tx).await;
            }
            msg = receiver.next() => {
                match msg {
            Some(Ok(Message::Text(text))) => {
                if handle_inbound_message(&mut state, text.to_string(), &tx, internal_tx.clone())
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Some(Ok(Message::Binary(bytes))) => {
                if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                    if handle_inbound_message(&mut state, text, &tx, internal_tx.clone()).await.is_err() {
                        break;
                    }
                }
            }
                    Some(Ok(Message::Ping(payload))) => {
                        let _ = tx.send(Message::Pong(payload)).await;
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        break;
                    }
                    Some(Err(err)) => {
                        warn!(?err, "task wizard websocket receive error");
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    state.shutdown().await;
    drop(tx);
    let _ = send_task.await;
}

async fn handle_inbound_message(
    state: &mut TaskWizardState,
    text: String,
    tx: &mpsc::Sender<Message>,
    internal_tx: mpsc::Sender<InternalEvent>,
) -> Result<(), ()> {
    let envelope: Result<TaskWizardInbound, _> = serde_json::from_str(&text);
    match envelope {
        Ok(TaskWizardInbound::Prompt { prompt }) => {
            if prompt.trim().is_empty() {
                send_error(tx, "Prompt cannot be empty").await;
                return Ok(());
            }
            match state.start_run(prompt, tx.clone(), internal_tx).await {
                Ok(_) => {
                    send_json(tx, json!({"type":"status","state":"running"})).await;
                }
                Err(err) => {
                    send_error(tx, err.as_str()).await;
                }
            }
        }
        Ok(TaskWizardInbound::Cancel) => {
            if state.cancel_run().await {
                send_json(tx, json!({"type":"status","state":"cancelling"})).await;
            }
        }
        Ok(TaskWizardInbound::Ping) => {
            send_json(tx, json!({"type":"pong"})).await;
        }
        Err(err) => {
            warn!(?err, "invalid task wizard message");
        }
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum TaskWizardInbound {
    Prompt { prompt: String },
    Cancel,
    Ping,
}

struct TaskWizardState {
    session_id: Uuid,
    thread_id: Option<String>,
    active_run: Option<ActiveRun>,
}

impl TaskWizardState {
    fn new(session_id: Uuid) -> Self {
        Self {
            session_id,
            thread_id: None,
            active_run: None,
        }
    }

    async fn start_run(
        &mut self,
        prompt: String,
        tx: mpsc::Sender<Message>,
        internal_tx: mpsc::Sender<InternalEvent>,
    ) -> Result<(), TaskWizardError> {
        if self.active_run.is_some() {
            return Err(TaskWizardError::Busy);
        }

        let run = spawn_task_wizard_process(
            prompt,
            self.thread_id.clone(),
            self.session_id,
            tx,
            internal_tx,
        )
        .await?;
        self.active_run = Some(run);
        Ok(())
    }

    async fn handle_internal_event(&mut self, event: InternalEvent, tx: &mpsc::Sender<Message>) {
        match event {
            InternalEvent::ThreadStarted { run_id, thread_id } => {
                if self
                    .active_run
                    .as_ref()
                    .map(|run| run.run_id == run_id)
                    .unwrap_or(false)
                {
                    self.thread_id = Some(thread_id.clone());
                    send_json(tx, json!({"type":"thread","threadId": thread_id})).await;
                }
            }
            InternalEvent::Completed {
                run_id,
                status,
                last_agent_message,
            } => {
                if let Some(run) = self.take_active_run_if(run_id) {
                    send_json(tx, json!({"type":"status","state":"idle"})).await;
                    handle_completion(
                        self.session_id,
                        self.thread_id.as_deref(),
                        run,
                        status,
                        last_agent_message,
                        tx,
                    )
                    .await;
                }
            }
        }
    }

    async fn cancel_run(&mut self) -> bool {
        if let Some(run) = &self.active_run {
            let kill = run.kill_handle.clone();
            let _ = kill.kill(KillReason::UserRequested).await;
            true
        } else {
            false
        }
    }

    async fn shutdown(&mut self) {
        if let Some(run) = self.active_run.take() {
            let kill = run.kill_handle.clone();
            let _ = kill.kill(KillReason::UserRequested).await;
            run.stream_task.abort();
        }
        self.thread_id = None;
    }

    fn take_active_run_if(&mut self, run_id: RunId) -> Option<ActiveRun> {
        if self
            .active_run
            .as_ref()
            .map(|run| run.run_id == run_id)
            .unwrap_or(false)
        {
            self.active_run.take()
        } else {
            None
        }
    }
}

struct ActiveRun {
    run_id: RunId,
    kill_handle: ProcessKillHandle,
    prompt: String,
    stream_task: JoinHandle<()>,
    workspace: TempDir,
}

enum TaskWizardError {
    Busy,
    SpawnFailed(String),
}

impl TaskWizardError {
    fn as_str(&self) -> &str {
        match self {
            TaskWizardError::Busy => "Wizard is already running a turn.",
            TaskWizardError::SpawnFailed(msg) => msg,
        }
    }
}

enum InternalEvent {
    ThreadStarted {
        run_id: RunId,
        thread_id: String,
    },
    Completed {
        run_id: RunId,
        status: RunCompletionStatus,
        last_agent_message: Option<String>,
    },
}

enum RunCompletionStatus {
    Success,
    Failed(String),
    Killed,
}

async fn spawn_task_wizard_process(
    prompt: String,
    thread_id: Option<String>,
    session_id: Uuid,
    ws_tx: mpsc::Sender<Message>,
    internal_tx: mpsc::Sender<InternalEvent>,
) -> Result<ActiveRun, TaskWizardError> {
    let workspace = TempDir::new().map_err(|err| TaskWizardError::SpawnFailed(err.to_string()))?;
    write_wizard_agents(&workspace)?;

    let api_port = resolve_api_port();
    let docker_command =
        build_task_wizard_command(api_port, thread_id.as_deref(), workspace.path());
    let run_id = Uuid::new_v4();
    let metadata = RunMetadata {
        run_id,
        persona: format!("task_wizard:{session_id}"),
        workspace_root: PathBuf::from(PROJECT_DIR.as_str()),
        tags: vec!["task_wizard".to_string()],
        priority: RunPriority::Normal,
        issued_at: Utc::now(),
    };

    let intent = ProcessSpawnIntent {
        metadata,
        program: docker_command[0].clone(),
        args: docker_command[1..].to_vec(),
        env: Vec::new(),
        working_dir: PathBuf::from(PROJECT_DIR.as_str()),
        stream_stdout: true,
        stream_stderr: true,
        stdin: Some(format!("{prompt}\n").into_bytes()),
    };

    let handles = threads::thread_handles();
    let handle_rx = handles
        .middleware
        .enqueue_spawn(intent)
        .await
        .map_err(|err| TaskWizardError::SpawnFailed(err.to_string()))?;

    let process_handle = handle_rx
        .await
        .map_err(|_| TaskWizardError::SpawnFailed("process handle channel closed".into()))?;

    let ProcessHandle {
        run_id: _,
        events,
        kill,
    } = process_handle;
    let stream_task = spawn_event_forwarder(events, ws_tx, internal_tx, run_id);

    Ok(ActiveRun {
        run_id,
        kill_handle: kill,
        prompt,
        stream_task,
        workspace,
    })
}

fn build_task_wizard_command(
    api_port: u16,
    thread_id: Option<&str>,
    workspace: &Path,
) -> Vec<String> {
    let builder = match thread_id {
        Some(id) => CodexExecBuilder::resume().session_id(id.to_string()),
        None => CodexExecBuilder::new(),
    };

    let launch_settings = codex_config::settings_for(CodexAgentKind::Wizard);

    let enabled_tools = [
        "tasks_list",
        "tasks_get",
        "tasks_create",
        "tasks_update",
        "tasks_set_status",
        "task_groups_list",
        "task_groups_get",
        "task_groups_create",
        "task_dependencies_get",
        "task_dependencies_set",
    ];

    let tools_arg = format!(
        "mcp_servers.robot_farm.enabled_tools=[{}]",
        enabled_tools
            .iter()
            .map(|tool| format!("\"{tool}\""))
            .collect::<Vec<_>>()
            .join(",")
    );

    let default_mcp_url = ensure_default_mcp_url(api_port);

    let codex_args = builder
        .change_dir("/workspace")
        .json(true)
        .config_override("mcp_servers.robot_farm.enabled=true")
        .config_override("mcp_servers.robot_farm.tool_timeout_sec=900")
        .config_override(tools_arg)
        .config_override(format!("mcp_servers.robot_farm.url=\"{default_mcp_url}\""))
        .config_override("mcp_servers.robot_farm.http_headers.AGENT=\"wizard\"")
        .config_override(format!("model=\"{}\"", launch_settings.model))
        .config_override(format!(
            "model_reasoning_effort=\"{}\"",
            launch_settings.reasoning
        ))
        .skip_git_repo_check(true)
        .build();

    let mut docker_args = DockerRunBuilder::new(DOCKER_IMAGE_WIZARD)
        .remove_container(true)
        .interactive(true)
        .attach("STDOUT")
        .attach("STDERR")
        .user("1000:1000")
        .workdir("/workspace")
        .command(codex_args)
        .volume(workspace, "/workspace", Some("rw".into()))
        .volume(
            format!(
                "{}/.codex",
                env::var("HOME").unwrap_or_else(|_| "/home/codex".to_string())
            ),
            "/home/codex/.codex",
            None,
        )
        .build();

    docker_overrides::apply_overrides(
        CodexAgentKind::Wizard,
        &mut docker_args,
        DOCKER_IMAGE_WIZARD,
    );

    docker_args
}

fn resolve_api_port() -> u16 {
    // Prefer the actual server PORT if set; fall back to ROBOT_FARM_API_PORT for backwards compat.
    let from_port_env = env::var("PORT").ok().and_then(|value| value.parse().ok());
    let from_legacy = env::var("ROBOT_FARM_API_PORT")
        .ok()
        .and_then(|value| value.parse().ok());
    from_port_env.or(from_legacy).unwrap_or(8080)
}

fn write_wizard_agents(workspace: &TempDir) -> Result<(), TaskWizardError> {
    let agents = include_str!("../../../directives/wizard.md");
    let path = workspace.path().join("AGENTS.md");
    fs::write(&path, agents).map_err(|err| TaskWizardError::SpawnFailed(err.to_string()))
}
fn spawn_event_forwarder(
    mut events: mpsc::Receiver<ProcessEvent>,
    ws_tx: mpsc::Sender<Message>,
    internal_tx: mpsc::Sender<InternalEvent>,
    run_id: RunId,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut buffer = JsonLineBuffer::default();
        let mut last_agent_message: Option<String> = None;

        while let Some(event) = events.recv().await {
            match event {
                ProcessEvent::Output(chunk) => {
                    if matches!(chunk.stream, ProcessStream::Stdout) {
                        for line in buffer.ingest(&chunk.bytes) {
                            forward_codex_line(
                                &line,
                                run_id,
                                &ws_tx,
                                &internal_tx,
                                &mut last_agent_message,
                            )
                            .await;
                        }
                    } else {
                        let text = String::from_utf8_lossy(&chunk.bytes);
                        send_json(
                            &ws_tx,
                            json!({"type":"log","stream":"stderr","line": text.trim()}),
                        )
                        .await;
                    }
                }
                ProcessEvent::OutputError(err) => {
                    send_json(
                        &ws_tx,
                        json!({"type":"log","stream":"stderr","line": err.message}),
                    )
                    .await;
                }
                ProcessEvent::Exit(exit) => {
                    if let Some(line) = buffer.flush() {
                        forward_codex_line(
                            &line,
                            run_id,
                            &ws_tx,
                            &internal_tx,
                            &mut last_agent_message,
                        )
                        .await;
                    }
                    let status = if exit.status.success() {
                        RunCompletionStatus::Success
                    } else {
                        RunCompletionStatus::Failed(format!(
                            "Task wizard exited with code {:?}",
                            exit.status.code()
                        ))
                    };
                    let _ = internal_tx
                        .send(InternalEvent::Completed {
                            run_id,
                            status,
                            last_agent_message,
                        })
                        .await;
                    break;
                }
                ProcessEvent::Killed(_) => {
                    let _ = internal_tx
                        .send(InternalEvent::Completed {
                            run_id,
                            status: RunCompletionStatus::Killed,
                            last_agent_message,
                        })
                        .await;
                    break;
                }
                ProcessEvent::SpawnFailed(err) => {
                    let _ = internal_tx
                        .send(InternalEvent::Completed {
                            run_id,
                            status: RunCompletionStatus::Failed(err.message),
                            last_agent_message,
                        })
                        .await;
                    break;
                }
            }
        }
    })
}

async fn forward_codex_line(
    line: &str,
    run_id: RunId,
    ws_tx: &mpsc::Sender<Message>,
    internal_tx: &mpsc::Sender<InternalEvent>,
    last_agent_message: &mut Option<String>,
) {
    match serde_json::from_str::<Value>(line) {
        Ok(value) => {
            if let Some(kind) = value.get("type").and_then(|v| v.as_str()) {
                match kind {
                    "thread.started" => {
                        if let Some(thread_id) = value.get("thread_id").and_then(|v| v.as_str()) {
                            let _ = internal_tx
                                .send(InternalEvent::ThreadStarted {
                                    run_id,
                                    thread_id: thread_id.to_string(),
                                })
                                .await;
                        }
                    }
                    "item.completed" => {
                        if let Some(item) = value.get("item").and_then(|v| v.as_object()) {
                            if item.get("type").and_then(|v| v.as_str()) == Some("agent_message") {
                                if let Some(text) = item.get("text").and_then(|v| v.as_str()) {
                                    *last_agent_message = Some(text.to_string());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            send_json(ws_tx, json!({"type":"codex_event","event": value})).await;
        }
        Err(err) => {
            send_json(
                ws_tx,
                json!({"type":"log","stream":"stdout","line": format!("Invalid JSON: {err}")}),
            )
            .await;
        }
    }
}

async fn handle_completion(
    session_id: Uuid,
    thread_id: Option<&str>,
    run: ActiveRun,
    status: RunCompletionStatus,
    last_agent_message: Option<String>,
    tx: &mpsc::Sender<Message>,
) {
    let prompt = run.prompt.clone();
    run.stream_task.abort();

    match status {
        RunCompletionStatus::Success => {
            let summary = last_agent_message
                .unwrap_or_else(|| "Wizard completed without a final message.".to_string());
            if let Some(feed_entry) =
                persist_feed_entry(session_id, thread_id, &prompt, &summary, true).await
            {
                send_json(
                    tx,
                    json!({
                        "type": "final",
                        "status": "success",
                        "response": summary,
                        "feedEntry": feed_entry,
                    }),
                )
                .await;
            } else {
                send_json(
                    tx,
                    json!({
                        "type": "final",
                        "status": "success",
                        "response": summary,
                    }),
                )
                .await;
            }
        }
        RunCompletionStatus::Failed(reason) => {
            send_json(
                tx,
                json!({
                    "type": "final",
                    "status": "failed",
                    "error": reason,
                }),
            )
            .await;
        }
        RunCompletionStatus::Killed => {
            send_json(
                tx,
                json!({
                    "type": "final",
                    "status": "cancelled",
                }),
            )
            .await;
        }
    }
}

async fn persist_feed_entry(
    session_id: Uuid,
    thread_id: Option<&str>,
    prompt: &str,
    response: &str,
    success: bool,
) -> Option<Value> {
    let raw = TaskWizardFeedRaw {
        variant: "task_wizard",
        session_id: session_id.to_string(),
        thread_id: thread_id.map(|v| v.to_string()),
        prompt: prompt.to_string(),
        response: response.to_string(),
        status: if success { "success" } else { "failed" }.to_string(),
    };

    let entry = NewFeedEntry {
        source: "TaskWizard".to_string(),
        target: "User".to_string(),
        level: if success {
            FeedLevel::Info
        } else {
            FeedLevel::Error
        },
        text: summarize(response),
        raw: serde_json::to_string(&raw).ok()?,
        category: "task_wizard".to_string(),
    };

    match feed::insert_feed_entry(entry).await {
        Ok(feed_entry) => {
            let mut realtime_entry = feed_entry.clone();
            realtime_entry.raw.clear();
            realtime::publish(RealtimeEvent::FeedEntry(realtime_entry));
            serde_json::to_value(feed_entry).ok()
        }
        Err(err) => {
            warn!(?err, "failed to insert task wizard feed entry");
            None
        }
    }
}

#[derive(Serialize)]
struct TaskWizardFeedRaw {
    variant: &'static str,
    session_id: String,
    thread_id: Option<String>,
    prompt: String,
    response: String,
    status: String,
}

fn summarize(text: &str) -> String {
    const LIMIT: usize = 160;
    if text.chars().count() <= LIMIT {
        text.to_string()
    } else {
        text.chars().take(LIMIT).collect::<String>() + "…"
    }
}

#[derive(Default)]
struct JsonLineBuffer {
    buffer: Vec<u8>,
}

impl JsonLineBuffer {
    fn ingest(&mut self, chunk: &[u8]) -> Vec<String> {
        self.buffer.extend_from_slice(chunk);
        let mut lines = Vec::new();
        loop {
            if let Some(pos) = self.buffer.iter().position(|b| *b == b'\n') {
                let mut line = self.buffer.drain(..=pos).collect::<Vec<_>>();
                if let Some(b'\n') = line.last() {
                    line.pop();
                }
                if let Some(b'\r') = line.last() {
                    line.pop();
                }
                if let Ok(text) = String::from_utf8(line) {
                    if !text.trim().is_empty() {
                        lines.push(text);
                    }
                }
            } else {
                break;
            }
        }
        lines
    }

    fn flush(&mut self) -> Option<String> {
        if self.buffer.is_empty() {
            None
        } else if let Ok(text) = String::from_utf8(std::mem::take(&mut self.buffer)) {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        } else {
            None
        }
    }
}

async fn send_json(tx: &mpsc::Sender<Message>, value: Value) {
    if let Ok(text) = serde_json::to_string(&value) {
        let _ = tx.send(Message::Text(text.into())).await;
    }
}

async fn send_error(tx: &mpsc::Sender<Message>, message: &str) {
    send_json(tx, json!({"type":"error","message": message})).await;
}

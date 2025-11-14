use crate::ai::schemas::{WorkerCompletion, WorkerIntent, WorkerTurn};
use crate::db::feed::NewFeedEntry;
use crate::db::message_queue::{MessageFilters, RelativePosition};
use crate::globals::PROJECT_DIR;
use crate::mcp::project_commands::ProjectCommandRegistry;
use crate::models::process::{
    ProcessEvent, ProcessHandle, ProcessIntent, ProcessSpawnIntent, ProcessStream, RunMetadata,
    RunPriority,
};
use crate::post_turn_checks::PostTurnCheckRegistry;
use crate::realtime::{self, RealtimeEvent};
use crate::shared::git::MergeConflict;
use crate::shared::{git, shell};
use crate::system::{
    events::{SystemActor, SystemEvent},
    queue::QueueCoordinator,
};
use crate::threads::database_manager::{DatabaseManagerError, DatabaseManagerHandle};
use crate::threads::middleware::MiddlewareHandle;
use crate::threads::process_manager::ProcessNotification;
use chrono::Utc;
use openapi::models::{CommandConfig, Message};
use serde_json;
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, error, warn};
use uuid::Uuid;

/// Queue manager intents describe the high-level operations that should be
/// serialized through the queue thread.
#[derive(Debug)]
pub enum QueueManagerCommand {
    ListMessages {
        filters: MessageFilters,
        respond_to: oneshot::Sender<Result<Vec<Message>, QueueManagerError>>,
    },
    DeleteAllMessages {
        respond_to: oneshot::Sender<Result<(), QueueManagerError>>,
    },
    DeleteMessageById {
        message_id: i64,
        respond_to: oneshot::Sender<Result<bool, QueueManagerError>>,
    },
    DeleteMessagesForRecipient {
        recipient: String,
        respond_to: oneshot::Sender<Result<u64, QueueManagerError>>,
    },
    InsertMessageRelative {
        message_id: i64,
        directive: RelativePosition,
        respond_to: oneshot::Sender<Result<Vec<Message>, QueueManagerError>>,
    },
    Pause {
        respond_to: oneshot::Sender<Result<(), QueueManagerError>>,
    },
    Resume {
        respond_to: oneshot::Sender<Result<(), QueueManagerError>>,
    },
    EnqueueProcessIntent {
        intent: ProcessIntent,
        respond_to: oneshot::Sender<Result<(), QueueManagerError>>,
    },
}

/// Public handle that other components use to talk to the queue manager.
#[derive(Clone)]
pub struct QueueManagerHandle {
    tx: mpsc::Sender<QueueManagerCommand>,
}

impl QueueManagerHandle {
    pub async fn list_messages(
        &self,
        filters: MessageFilters,
    ) -> Result<Vec<Message>, QueueManagerError> {
        self.request(|respond_to| QueueManagerCommand::ListMessages {
            filters,
            respond_to,
        })
        .await
    }

    pub async fn delete_all_messages(&self) -> Result<(), QueueManagerError> {
        self.request(|respond_to| QueueManagerCommand::DeleteAllMessages { respond_to })
            .await
    }

    pub async fn delete_message_by_id(&self, message_id: i64) -> Result<bool, QueueManagerError> {
        self.request(|respond_to| QueueManagerCommand::DeleteMessageById {
            message_id,
            respond_to,
        })
        .await
    }

    pub async fn delete_messages_for_recipient(
        &self,
        recipient: String,
    ) -> Result<u64, QueueManagerError> {
        self.request(
            |respond_to| QueueManagerCommand::DeleteMessagesForRecipient {
                recipient,
                respond_to,
            },
        )
        .await
    }

    pub async fn insert_message_relative(
        &self,
        message_id: i64,
        directive: RelativePosition,
    ) -> Result<Vec<Message>, QueueManagerError> {
        self.request(|respond_to| QueueManagerCommand::InsertMessageRelative {
            message_id,
            directive,
            respond_to,
        })
        .await
    }

    pub async fn pause(&self) -> Result<(), QueueManagerError> {
        self.request(|respond_to| QueueManagerCommand::Pause { respond_to })
            .await
    }

    pub async fn resume(&self) -> Result<(), QueueManagerError> {
        self.request(|respond_to| QueueManagerCommand::Resume { respond_to })
            .await
    }

    pub async fn enqueue_process_intent(
        &self,
        intent: ProcessIntent,
    ) -> Result<(), QueueManagerError> {
        self.request(|respond_to| QueueManagerCommand::EnqueueProcessIntent { intent, respond_to })
            .await
    }

    async fn request<T>(
        &self,
        build: impl FnOnce(oneshot::Sender<Result<T, QueueManagerError>>) -> QueueManagerCommand,
    ) -> Result<T, QueueManagerError>
    where
        T: Send + 'static,
    {
        let (respond_to, response) = oneshot::channel();
        let command = build(respond_to);
        self.tx
            .send(command)
            .await
            .map_err(|_| QueueManagerError::ChannelClosed)?;
        let result = response
            .await
            .map_err(|_| QueueManagerError::ResponseDropped)?;
        result
    }
}

pub struct QueueManagerConfig {
    pub command_buffer: usize,
}

impl Default for QueueManagerConfig {
    fn default() -> Self {
        Self { command_buffer: 64 }
    }
}

pub fn spawn_queue_manager(
    config: QueueManagerConfig,
    db: DatabaseManagerHandle,
    middleware: MiddlewareHandle,
    notifications_rx: mpsc::Receiver<ProcessNotification>,
) -> QueueManagerHandle {
    let (tx, rx) = mpsc::channel(config.command_buffer);
    let runtime = QueueManagerRuntime {
        db,
        middleware,
        rx,
        notifications_rx,
        state: QueueRuntimeState::default(),
    };
    tokio::spawn(run_queue_manager(runtime));
    QueueManagerHandle { tx }
}

struct QueueManagerRuntime {
    db: DatabaseManagerHandle,
    middleware: MiddlewareHandle,
    rx: mpsc::Receiver<QueueManagerCommand>,
    notifications_rx: mpsc::Receiver<ProcessNotification>,
    state: QueueRuntimeState,
}

impl QueueManagerRuntime {
    async fn handle_command(
        &mut self,
        command: QueueManagerCommand,
    ) -> Result<(), QueueManagerError> {
        match command {
            QueueManagerCommand::ListMessages {
                filters,
                respond_to,
            } => {
                let result = self
                    .db
                    .list_messages(filters)
                    .await
                    .map_err(QueueManagerError::from);
                let _ = respond_to.send(result);
            }
            QueueManagerCommand::DeleteAllMessages { respond_to } => {
                let result = self
                    .db
                    .delete_all_messages()
                    .await
                    .map_err(QueueManagerError::from);
                let _ = respond_to.send(result);
            }
            QueueManagerCommand::DeleteMessageById {
                message_id,
                respond_to,
            } => {
                let result = self
                    .db
                    .delete_message_by_id(message_id)
                    .await
                    .map_err(QueueManagerError::from);
                let _ = respond_to.send(result);
            }
            QueueManagerCommand::DeleteMessagesForRecipient {
                recipient,
                respond_to,
            } => {
                let result = self
                    .db
                    .delete_messages_for_recipient(recipient)
                    .await
                    .map_err(QueueManagerError::from);
                let _ = respond_to.send(result);
            }
            QueueManagerCommand::InsertMessageRelative {
                message_id,
                directive,
                respond_to,
            } => {
                let result = self
                    .db
                    .insert_message_relative(message_id, directive)
                    .await
                    .map_err(QueueManagerError::from);
                let _ = respond_to.send(result);
            }
            QueueManagerCommand::Pause { respond_to } => {
                self.state.paused = true;
                debug!("queue manager paused");
                let _ = respond_to.send(Ok(()));
            }
            QueueManagerCommand::Resume { respond_to } => {
                self.state.paused = false;
                debug!("queue manager resumed");
                if let Err(err) = self.flush_pending().await {
                    let _ = respond_to.send(Err(err));
                } else {
                    let _ = respond_to.send(Ok(()));
                }
            }
            QueueManagerCommand::EnqueueProcessIntent { intent, respond_to } => {
                let result = self.enqueue_intent(intent).await;
                let _ = respond_to.send(result);
            }
        }
        Ok(())
    }

    async fn handle_notification(
        &mut self,
        notification: ProcessNotification,
    ) -> Result<(), QueueManagerError> {
        match notification {
            ProcessNotification::WorkerTurn {
                run_id,
                worker_id,
                metadata,
                turn,
            } => {
                debug!(%run_id, worker_id, "worker turn completed");
                self.process_worker_turn(worker_id, metadata, turn).await?;
            }
        }
        Ok(())
    }

    async fn process_worker_turn(
        &mut self,
        worker_id: i64,
        _metadata: RunMetadata,
        turn: WorkerTurn,
    ) -> Result<(), QueueManagerError> {
        QueueCoordinator::global().clear_assignment(worker_id);

        match turn.intent {
            WorkerIntent::CompleteTask => {
                if let Some(completed) = turn.completed {
                    self.handle_worker_completion(worker_id, completed).await?;
                } else {
                    self.record_validation_failure(worker_id, "missing completion payload")
                        .await?;
                }
            }
            WorkerIntent::Blocked => {
                if let Some(blocked) = turn.blocked {
                    let message = format!(
                        "ws{worker_id} is blocked on {}: {}",
                        blocked.task_slug, blocked.reason
                    );
                    self.enqueue_message(
                        SystemActor::Worker(worker_id),
                        SystemActor::Orchestrator,
                        &message,
                    )
                    .await?;
                }
            }
            WorkerIntent::StatusUpdate | WorkerIntent::AckPause => {
                // Future: pipe summaries to orchestrator feed.
            }
        }

        Ok(())
    }

    async fn handle_worker_completion(
        &mut self,
        worker_id: i64,
        completed: WorkerCompletion,
    ) -> Result<(), QueueManagerError> {
        match self.run_post_turn_pipeline(worker_id, &completed).await {
            Ok(_) => {
                self.notify_orchestrator_completion(worker_id, &completed)
                    .await?
            }
            Err(err) => {
                self.notify_worker_failure(worker_id, &err).await?;
            }
        }
        Ok(())
    }

    async fn run_post_turn_pipeline(
        &mut self,
        worker_id: i64,
        completed: &WorkerCompletion,
    ) -> Result<(), PostTurnError> {
        self.run_post_turn_checks(worker_id).await?;
        self.auto_commit_and_merge(worker_id, completed).await?;
        Ok(())
    }

    async fn run_post_turn_checks(&mut self, worker_id: i64) -> Result<(), PostTurnError> {
        let plan = PostTurnCheckRegistry::global().list();
        if plan.is_empty() {
            return Ok(());
        }
        let registry = ProjectCommandRegistry::global();
        for check_id in plan {
            let command = registry
                .get(&check_id)
                .ok_or_else(|| PostTurnError::UnknownCheck(check_id.clone()))?;
            let result = self
                .run_check_command(worker_id, &check_id, &command)
                .await?;
            if !result.success {
                return Err(PostTurnError::CheckFailed {
                    id: check_id,
                    stdout: result.stdout,
                    stderr: result.stderr,
                    exit_code: result.exit_code,
                });
            }
        }
        Ok(())
    }

    async fn run_check_command(
        &mut self,
        worker_id: i64,
        check_id: &str,
        command: &CommandConfig,
    ) -> Result<ProcessRunResult, PostTurnError> {
        if command.exec.is_empty() {
            return Err(PostTurnError::InvalidCommand(check_id.to_string()));
        }

        let workspace_root = Path::new(PROJECT_DIR.as_str());
        let working_dir = shell::resolve_working_dir(workspace_root, command.cwd.as_deref())
            .map_err(|err| PostTurnError::InvalidWorkingDir(err.to_string()))?;

        let metadata = RunMetadata {
            run_id: Uuid::new_v4(),
            persona: format!("post_turn_check:{check_id}"),
            workspace_root: workspace_root.to_path_buf(),
            tags: vec![
                "post_turn_check".to_string(),
                format!("worker:{worker_id}"),
                format!("check:{check_id}"),
            ],
            priority: RunPriority::High,
            issued_at: Utc::now(),
        };

        let intent = ProcessSpawnIntent {
            metadata,
            program: command.exec[0].clone(),
            args: command.exec.iter().skip(1).cloned().collect(),
            env: Vec::new(),
            working_dir,
            stream_stdout: true,
            stream_stderr: true,
            stdin: None,
        };

        let handle_rx = self
            .middleware
            .enqueue_spawn(intent)
            .await
            .map_err(|err| PostTurnError::Spawn(err.to_string()))?;

        let handle = handle_rx.await.map_err(|_| PostTurnError::HandleDropped)?;

        self.await_process(handle).await
    }

    async fn await_process(
        &self,
        mut handle: ProcessHandle,
    ) -> Result<ProcessRunResult, PostTurnError> {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let mut exit_code: Option<i32> = None;

        while let Some(event) = handle.events.recv().await {
            match event {
                ProcessEvent::Output(chunk) => match chunk.stream {
                    ProcessStream::Stdout => stdout.extend_from_slice(&chunk.bytes),
                    ProcessStream::Stderr => stderr.extend_from_slice(&chunk.bytes),
                },
                ProcessEvent::OutputError(err) => {
                    stderr.extend_from_slice(err.message.as_bytes());
                }
                ProcessEvent::Exit(exit) => {
                    exit_code = exit.status.code();
                    break;
                }
                ProcessEvent::Killed(_) => {
                    return Err(PostTurnError::ProcessTerminated);
                }
                ProcessEvent::SpawnFailed(err) => {
                    return Err(PostTurnError::Spawn(err.message));
                }
            }
        }

        let stdout = String::from_utf8_lossy(&stdout).to_string();
        let stderr = String::from_utf8_lossy(&stderr).to_string();
        let success = exit_code.map(|code| code == 0).unwrap_or(false);

        Ok(ProcessRunResult {
            success,
            exit_code,
            stdout,
            stderr,
        })
    }

    async fn auto_commit_and_merge(
        &self,
        worker_id: i64,
        completed: &WorkerCompletion,
    ) -> Result<(), PostTurnError> {
        let worker_root = worker_worktree_path(worker_id);
        if !worker_root.exists() {
            return Err(PostTurnError::MissingWorktree(worker_root));
        }

        git::stage_all(&worker_root).map_err(PostTurnError::Git)?;

        match git::commit(&worker_root, &completed.commit_summary) {
            Ok(_) => {}
            Err(git::GitError::CommandFailure { stderr })
                if stderr.contains("nothing to commit") =>
            {
                return Err(PostTurnError::NothingToCommit);
            }
            Err(err) => return Err(PostTurnError::Git(err)),
        }

        let staging = staging_path();
        let branch = format!("ws{worker_id}");
        if let Err(err) = git::merge_ff_only(&staging, &branch) {
            debug!(
                ?err,
                worker_id, "fast-forward merge failed, attempting regular merge"
            );
            if let Err(err) = git::merge(&staging, &branch) {
                error!(?err, worker_id, "merge failed");
                let conflicts =
                    git::collect_merge_conflicts(&staging).map_err(PostTurnError::Git)?;
                let _ = git::abort_merge(&staging);
                return Err(PostTurnError::MergeConflict(conflicts));
            }
        }

        Ok(())
    }

    async fn notify_worker_failure(
        &self,
        worker_id: i64,
        error: &PostTurnError,
    ) -> Result<(), QueueManagerError> {
        let message = error.render(worker_id);
        self.enqueue_message(
            SystemActor::System,
            SystemActor::Worker(worker_id),
            &message,
        )
        .await?;
        QueueCoordinator::global().validation_failed(worker_id, &message);
        Ok(())
    }

    async fn notify_orchestrator_completion(
        &self,
        worker_id: i64,
        completed: &WorkerCompletion,
    ) -> Result<(), QueueManagerError> {
        let mut message = format!("ws{worker_id} completed {}", completed.task_slug);
        if let Some(notes) = completed.notes.as_ref().filter(|v| !v.trim().is_empty()) {
            message.push_str(": ");
            message.push_str(notes.trim());
        }
        self.enqueue_message(SystemActor::System, SystemActor::Orchestrator, &message)
            .await?;
        Ok(())
    }

    async fn record_validation_failure(
        &self,
        worker_id: i64,
        reason: &str,
    ) -> Result<(), QueueManagerError> {
        QueueCoordinator::global().validation_failed(worker_id, reason);
        self.enqueue_message(SystemActor::System, SystemActor::Worker(worker_id), reason)
            .await?;
        Ok(())
    }

    async fn enqueue_message(
        &self,
        from: SystemActor,
        to: SystemActor,
        body: &str,
    ) -> Result<Message, QueueManagerError> {
        self.db
            .enqueue_message(from.label(), to.label(), body.to_string())
            .await
            .map_err(QueueManagerError::from)
    }

    async fn enqueue_intent(&mut self, intent: ProcessIntent) -> Result<(), QueueManagerError> {
        if self.state.paused {
            self.state.buffered.push_back(intent);
            return Ok(());
        }

        self.middleware
            .sender()
            .send(intent)
            .await
            .map_err(|err| QueueManagerError::MiddlewareSend(err.to_string()))
    }

    async fn flush_pending(&mut self) -> Result<(), QueueManagerError> {
        while let Some(intent) = self.state.buffered.pop_front() {
            self.middleware
                .sender()
                .send(intent)
                .await
                .map_err(|err| QueueManagerError::MiddlewareSend(err.to_string()))?;
        }
        Ok(())
    }

    async fn flush_system_events(&self) {
        let coordinator = QueueCoordinator::global();
        let events = coordinator.drain_events();
        for event in events {
            let entry = event_to_feed_entry(event);
            match self.db.insert_feed_entry(entry).await {
                Ok(feed_entry) => realtime::publish(RealtimeEvent::FeedEntry(feed_entry)),
                Err(err) => warn!(?err, "failed to persist system event"),
            }
        }
    }
}

struct QueueRuntimeState {
    paused: bool,
    buffered: VecDeque<ProcessIntent>,
}

impl Default for QueueRuntimeState {
    fn default() -> Self {
        Self {
            paused: true,
            buffered: VecDeque::new(),
        }
    }
}

async fn run_queue_manager(mut runtime: QueueManagerRuntime) {
    loop {
        tokio::select! {
            Some(command) = runtime.rx.recv() => {
                if let Err(err) = runtime.handle_command(command).await {
                    warn!(?err, "queue manager command failed");
                }
            }
            Some(notification) = runtime.notifications_rx.recv() => {
                if let Err(err) = runtime.handle_notification(notification).await {
                    warn!(?err, "queue manager notification handling failed");
                }
            }
            else => break,
        }

        runtime.flush_system_events().await;
    }

    warn!("queue manager channels closed; exiting loop");
}

fn event_to_feed_entry(event: SystemEvent) -> NewFeedEntry {
    NewFeedEntry {
        source: event.source.label(),
        target: event.target.label(),
        level: event.level,
        text: event.summary.clone(),
        raw: serde_json::to_string(&event.details).unwrap_or_else(|_| "{}".into()),
        category: event.category.as_str().to_string(),
    }
}

fn worker_worktree_path(worker_id: i64) -> PathBuf {
    Path::new(PROJECT_DIR.as_str()).join(format!("ws{worker_id}"))
}

fn staging_path() -> PathBuf {
    Path::new(PROJECT_DIR.as_str()).join("staging")
}

#[derive(Debug)]
struct ProcessRunResult {
    success: bool,
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
}

#[derive(Debug, Error)]
pub enum QueueManagerError {
    #[error(transparent)]
    Database(#[from] DatabaseManagerError),
    #[error("queue manager channel closed")]
    ChannelClosed,
    #[error("queue manager response channel closed")]
    ResponseDropped,
    #[error("failed to send middleware intent: {0}")]
    MiddlewareSend(String),
}

#[derive(Debug, Error)]
enum PostTurnError {
    #[error("post-turn check '{id}' failed")]
    CheckFailed {
        id: String,
        stdout: String,
        stderr: String,
        exit_code: Option<i32>,
    },
    #[error("post-turn check '{0}' is not defined in config")]
    UnknownCheck(String),
    #[error("post-turn check '{0}' has an empty exec list")]
    InvalidCommand(String),
    #[error("invalid working directory: {0}")]
    InvalidWorkingDir(String),
    #[error("post-turn command spawn failed: {0}")]
    Spawn(String),
    #[error("post-turn process handle dropped before spawn")]
    HandleDropped,
    #[error("post-turn process terminated unexpectedly")]
    ProcessTerminated,
    #[error("worker worktree not found: {0}")]
    MissingWorktree(PathBuf),
    #[error("git error: {0}")]
    Git(#[from] git::GitError),
    #[error("no changes to commit")]
    NothingToCommit,
    #[error("merge conflict detected")]
    MergeConflict(Vec<MergeConflict>),
}

impl PostTurnError {
    fn render(&self, worker_id: i64) -> String {
        match self {
            PostTurnError::CheckFailed {
                id,
                stdout,
                stderr,
                exit_code,
            } => {
                let mut msg = format!(
                    "Post-turn check '{id}' failed for ws{worker_id} (exit {:?}).",
                    exit_code
                );
                if !stdout.trim().is_empty() {
                    msg.push_str("\nstdout:\n");
                    msg.push_str(stdout.trim());
                }
                if !stderr.trim().is_empty() {
                    msg.push_str("\nstderr:\n");
                    msg.push_str(stderr.trim());
                }
                msg
            }
            PostTurnError::UnknownCheck(id) => {
                format!("Configured post-turn check '{id}' no longer exists. Please update config.")
            }
            PostTurnError::InvalidCommand(id) => {
                format!("Post-turn check '{id}' has no executable command configured.")
            }
            PostTurnError::InvalidWorkingDir(dir) => {
                format!("Post-turn check working directory is invalid: {dir}")
            }
            PostTurnError::Spawn(err) => format!("Failed to spawn post-turn check: {err}"),
            PostTurnError::HandleDropped => {
                "Failed to start post-turn check (handle dropped).".to_string()
            }
            PostTurnError::ProcessTerminated => {
                "A post-turn command terminated unexpectedly.".to_string()
            }
            PostTurnError::MissingWorktree(path) => format!(
                "Worker worktree {} is missing; cannot finalize turn.",
                path.display()
            ),
            PostTurnError::Git(err) => format!("Git operation failed: {err}"),
            PostTurnError::NothingToCommit => {
                "No changes were detected to commit; ensure edits were saved.".to_string()
            }
            PostTurnError::MergeConflict(conflicts) => {
                let files: Vec<String> = conflicts.iter().map(|c| c.file.clone()).collect();
                format!(
                    "Merge conflict while syncing ws{worker_id}. Conflicting files: {}",
                    files.join(", ")
                )
            }
        }
    }
}

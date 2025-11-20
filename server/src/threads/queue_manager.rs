use crate::ai::schemas::{
    Assignment, OrchestratorIntent, OrchestratorTurn, WorkerCompletion, WorkerIntent, WorkerTurn,
};
use crate::db;
use crate::db::feed::NewFeedEntry;
use crate::db::message_queue::{MessageFilters, RelativePosition};
use crate::db::task as task_db;
use crate::globals::PROJECT_DIR;
use crate::mcp::project_commands::ProjectCommandRegistry;
use crate::models::process::{
    KillReason, ProcessEvent, ProcessHandle, ProcessIntent, ProcessSpawnIntent, ProcessStream,
    RunId, RunMetadata, RunPriority,
};
use crate::models::strategy::OrchestratorHint;
use crate::post_turn_checks::PostTurnCheckRegistry;
use crate::realtime::{self, RealtimeEvent};
use crate::shared::git::MergeConflict;
use crate::shared::{git, shell};
use crate::system::{
    events::{SystemActor, SystemEvent},
    queue::{QueueCoordinator, QueueError},
    runner::{self, Persona, RunnerConfig},
    strategy::StrategyState,
};
use crate::threads::database_manager::{DatabaseManagerError, DatabaseManagerHandle};
use crate::threads::middleware::MiddlewareHandle;
use crate::threads::process_manager::{AgentRunActor, ProcessNotification};
use chrono::Utc;
use openapi::models::{
    CommandConfig, Feed, FeedLevel, Message, Strategy as ApiStrategy, TaskUpdateInput,
};
use serde_json;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::{Path, PathBuf},
    time::Duration,
};
use thiserror::Error;
use tokio::{
    spawn,
    sync::{mpsc, oneshot},
    time::{MissedTickBehavior, interval},
};
use tracing::{debug, error, info, warn};
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
    EnqueueMessage {
        from: SystemActor,
        to: SystemActor,
        body: String,
        respond_to: oneshot::Sender<Result<Message, QueueManagerError>>,
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
    KillWorker {
        worker_id: i64,
        respond_to: oneshot::Sender<Result<(), QueueManagerError>>,
    },
    KillOrchestrator {
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

    pub async fn enqueue_manual_message(
        &self,
        from: SystemActor,
        to: SystemActor,
        body: String,
    ) -> Result<Message, QueueManagerError> {
        self.request(|respond_to| QueueManagerCommand::EnqueueMessage {
            from,
            to,
            body,
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

    pub async fn kill_worker(&self, worker_id: i64) -> Result<(), QueueManagerError> {
        self.request(|respond_to| QueueManagerCommand::KillWorker {
            worker_id,
            respond_to,
        })
        .await
    }

    pub async fn kill_orchestrator(&self) -> Result<(), QueueManagerError> {
        self.request(|respond_to| QueueManagerCommand::KillOrchestrator { respond_to })
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
                info!("queue command: delete_all_messages");
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
                info!(message_id, "queue command: delete_message_by_id");
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
                info!(recipient, "queue command: delete_messages_for_recipient");
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
                info!(
                    message_id,
                    ?directive,
                    "queue command: insert_message_relative"
                );
                let result = self
                    .db
                    .insert_message_relative(message_id, directive)
                    .await
                    .map_err(QueueManagerError::from);
                let _ = respond_to.send(result);
            }
            QueueManagerCommand::EnqueueMessage {
                from,
                to,
                body,
                respond_to,
            } => {
                info!(from = %from.label(), to = %to.label(), "queue command: enqueue_manual_message");
                let result = self.enqueue_message(from, to, &body).await;
                let _ = respond_to.send(result);
            }
            QueueManagerCommand::Pause { respond_to } => {
                self.state.paused = true;
                info!("queue manager paused");
                let _ = respond_to.send(Ok(()));
            }
            QueueManagerCommand::Resume { respond_to } => {
                self.state.paused = false;
                info!("queue manager resumed");
                if let Err(err) = self.flush_pending().await {
                    let _ = respond_to.send(Err(err));
                } else {
                    let _ = respond_to.send(Ok(()));
                }
            }
            QueueManagerCommand::EnqueueProcessIntent { intent, respond_to } => {
                info!("queue command: enqueue_process_intent");
                let result = self.enqueue_intent(intent).await;
                let _ = respond_to.send(result);
            }
            QueueManagerCommand::KillWorker {
                worker_id,
                respond_to,
            } => {
                info!(worker_id, "queue command: kill_worker");
                let result = self.kill_worker_process(worker_id).await;
                let _ = respond_to.send(result);
            }
            QueueManagerCommand::KillOrchestrator { respond_to } => {
                info!("queue command: kill_orchestrator");
                let result = self.kill_orchestrator_process().await;
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
                info!(%run_id, worker_id, intent = ?turn.intent, "worker turn notification");
                self.process_worker_turn(worker_id, metadata, turn).await?;
            }
            ProcessNotification::OrchestratorTurn {
                run_id,
                metadata,
                turn,
            } => {
                info!(%run_id, intent = ?turn.intent, "orchestrator turn notification");
                self.process_orchestrator_turn(metadata, turn).await?;
            }
            ProcessNotification::AgentFeed {
                actor,
                message,
                raw,
                thread_id,
                category,
            } => match actor {
                AgentRunActor::Worker(worker_id) => {
                    if let Some(thread_id) = thread_id.clone() {
                        match db::session::upsert_session(&format!("ws{worker_id}"), &thread_id)
                            .await
                        {
                            Ok(_) => realtime::publish(RealtimeEvent::WorkerThread {
                                worker_id,
                                thread_id: Some(thread_id.clone()),
                            }),
                            Err(err) => warn!(?err, worker_id, "failed to store worker thread id"),
                        }
                    }
                    self.record_worker_feed(worker_id, &message, &raw, category.as_deref())
                        .await?;
                }
                AgentRunActor::Orchestrator => {
                    if let Some(thread_id) = thread_id {
                        match db::session::upsert_session("orchestrator", &thread_id).await {
                            Ok(_) => realtime::publish(RealtimeEvent::OrchestratorThread {
                                thread_id: Some(thread_id.clone()),
                            }),
                            Err(err) => {
                                warn!(?err, "failed to store orchestrator thread id");
                            }
                        }
                    }
                    self.record_orchestrator_feed(&message, &raw, category.as_deref())
                        .await?;
                }
            },
            ProcessNotification::AgentCompleted { actor, run_id } => match actor {
                AgentRunActor::Worker(worker_id) => {
                    self.state.active_workers.remove(&worker_id);
                    self.state.worker_runs.remove(&worker_id);
                    QueueCoordinator::global().clear_assignment(worker_id);
                    debug!(%run_id, worker_id, "worker run completed");
                }
                AgentRunActor::Orchestrator => {
                    self.state.orchestrator_run = None;
                    debug!(%run_id, "orchestrator run completed");
                }
            },
        }
        Ok(())
    }

    async fn process_worker_turn(
        &mut self,
        worker_id: i64,
        _metadata: RunMetadata,
        turn: WorkerTurn,
    ) -> Result<(), QueueManagerError> {
        info!(worker_id, intent = ?turn.intent, "processing worker turn");

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
            WorkerIntent::StatusUpdate => {
                if let Some(message) = Self::format_status_update(worker_id, &turn) {
                    self.enqueue_message(
                        SystemActor::Worker(worker_id),
                        SystemActor::Orchestrator,
                        &message,
                    )
                    .await?;
                }
                Self::record_support_hint(worker_id);
            }
            WorkerIntent::AckPause => {}
        }

        Ok(())
    }

    async fn process_orchestrator_turn(
        &mut self,
        _metadata: RunMetadata,
        turn: OrchestratorTurn,
    ) -> Result<(), QueueManagerError> {
        match turn.intent {
            OrchestratorIntent::AssignTask => {
                self.handle_orchestrator_assignment(&turn).await?;
            }
            OrchestratorIntent::StatusUpdate => {
                self.handle_orchestrator_status(&turn).await?;
            }
            OrchestratorIntent::AckPause => {}
        }
        Ok(())
    }

    async fn handle_orchestrator_assignment(
        &mut self,
        turn: &OrchestratorTurn,
    ) -> Result<(), QueueManagerError> {
        let Some(target) = turn.target.as_deref() else {
            warn!("orchestrator assignment missing target worker");
            return Ok(());
        };
        let Some(worker_id) = Self::parse_worker_target(target) else {
            warn!(target, "orchestrator assignment target invalid");
            return Ok(());
        };
        let Some(assignment) = turn.assignments.as_ref() else {
            warn!(worker_id, "orchestrator assignment missing payload");
            return Ok(());
        };

        let slug = assignment.task_slug.trim();
        if slug.is_empty() {
            warn!(worker_id, "orchestrator assignment missing task slug");
            return Ok(());
        }

        let task = task_db::get_task_by_slug(slug)
            .await
            .map_err(|err| QueueManagerError::Assignment(err.to_string()))?
            .ok_or_else(|| QueueManagerError::Assignment(format!("task {slug} not found")))?;

        let assignment_message = Self::format_assignment_message(turn, assignment);

        if task.owner.trim().eq_ignore_ascii_case("orchestrator") {
            let mut update = TaskUpdateInput::new();
            let worker_label = format!("ws{worker_id}");
            update.owner = Some(worker_label.clone());
            let updated = task_db::update_task(task.id, update)
                .await
                .map_err(|err| QueueManagerError::Assignment(err.to_string()))?
                .ok_or_else(|| {
                    QueueManagerError::Assignment(format!("task {} missing", task.id))
                })?;

            if let Err(QueueError::WorkerBusy) = QueueCoordinator::global().assign_task(
                worker_id,
                updated.id,
                Some(updated.slug.clone()),
            ) {
                warn!(
                    worker_id,
                    task_id = updated.id,
                    "worker already has an assignment"
                );
            }
        } else {
            warn!(
                worker_id,
                task_id = task.id,
                owner = %task.owner,
                "task already owned by target; sending orchestrator message only"
            );
        }

        self.enqueue_message(
            SystemActor::Orchestrator,
            SystemActor::Worker(worker_id),
            &assignment_message,
        )
        .await?;
        Ok(())
    }

    async fn handle_orchestrator_status(
        &self,
        turn: &OrchestratorTurn,
    ) -> Result<(), QueueManagerError> {
        let Some(target) = turn.target.as_deref() else {
            warn!("orchestrator status update missing target");
            return Ok(());
        };
        let Some(worker_id) = Self::parse_worker_target(target) else {
            warn!(target, "orchestrator status update target invalid");
            return Ok(());
        };

        let message = Self::format_orchestrator_status(turn);
        self.enqueue_message(
            SystemActor::Orchestrator,
            SystemActor::Worker(worker_id),
            &message,
        )
        .await?;
        Ok(())
    }

    async fn handle_worker_completion(
        &mut self,
        worker_id: i64,
        completed: WorkerCompletion,
    ) -> Result<(), QueueManagerError> {
        PostTurnJob::spawn(
            worker_id,
            completed,
            self.middleware.clone(),
            self.db.clone(),
        );
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

    async fn record_worker_feed(
        &self,
        worker_id: i64,
        message: &str,
        raw: &str,
        category: Option<&str>,
    ) -> Result<(), QueueManagerError> {
        let feed_category = category.unwrap_or("worker");
        let entry = NewFeedEntry {
            source: format!("ws{worker_id}"),
            target: format!("ws{worker_id}"),
            level: FeedLevel::Info,
            text: message.to_string(),
            raw: raw.to_string(),
            category: feed_category.to_string(),
        };
        let feed_entry = self
            .db
            .insert_feed_entry(entry)
            .await
            .map_err(QueueManagerError::from)?;
        realtime::publish(RealtimeEvent::FeedEntry(sanitize_feed_entry(&feed_entry)));
        Ok(())
    }

    async fn record_orchestrator_feed(
        &self,
        message: &str,
        raw: &str,
        category: Option<&str>,
    ) -> Result<(), QueueManagerError> {
        let feed_category = category.unwrap_or("orchestrator");
        let entry = NewFeedEntry {
            source: "Orchestrator".to_string(),
            target: "Orchestrator".to_string(),
            level: FeedLevel::Info,
            text: message.to_string(),
            raw: raw.to_string(),
            category: feed_category.to_string(),
        };
        let feed_entry = self
            .db
            .insert_feed_entry(entry)
            .await
            .map_err(QueueManagerError::from)?;
        realtime::publish(RealtimeEvent::FeedEntry(sanitize_feed_entry(&feed_entry)));
        Ok(())
    }

    async fn enqueue_message(
        &self,
        from: SystemActor,
        to: SystemActor,
        body: &str,
    ) -> Result<Message, QueueManagerError> {
        info!(from = %from.label(), to = %to.label(), "queue manager enqueueing message");
        self.db
            .enqueue_message(from.label(), to.label(), body.to_string())
            .await
            .map_err(QueueManagerError::from)
    }

    async fn drive_queue(&mut self) -> Result<(), QueueManagerError> {
        if self.state.paused {
            return Ok(());
        }

        let queue = self
            .db
            .list_messages(MessageFilters::default())
            .await
            .map_err(QueueManagerError::from)?;

        for entry in queue {
            match SystemActor::from_label(&entry.to) {
                Some(SystemActor::Worker(worker_id)) => {
                    if self.state.active_workers.contains(&worker_id) {
                        continue;
                    }
                    match self.dispatch_worker_message(worker_id, &entry).await {
                        Ok(true) => {
                            if let Err(err) = self.db.delete_message_by_id(entry.id).await {
                                warn!(
                                    ?err,
                                    message_id = entry.id,
                                    "failed to delete message after dispatch"
                                );
                            }
                        }
                        Ok(false) => continue,
                        Err(err) => {
                            warn!(
                                ?err,
                                worker_id,
                                message_id = entry.id,
                                "failed to dispatch worker message"
                            );
                        }
                    }
                }
                Some(SystemActor::Orchestrator) => {
                    if self.state.orchestrator_run.is_some() {
                        continue;
                    }
                    match self.dispatch_orchestrator_message(&entry).await {
                        Ok(true) => {
                            if let Err(err) = self.db.delete_message_by_id(entry.id).await {
                                warn!(
                                    ?err,
                                    message_id = entry.id,
                                    "failed to delete orchestrator message after dispatch"
                                );
                            }
                        }
                        Ok(false) => continue,
                        Err(err) => {
                            warn!(
                                ?err,
                                message_id = entry.id,
                                "failed to dispatch orchestrator message"
                            );
                        }
                    }
                }
                _ => continue,
            }
        }

        Ok(())
    }

    async fn dispatch_worker_message(
        &mut self,
        worker_id: i64,
        message: &Message,
    ) -> Result<bool, QueueManagerError> {
        let worktree = worker_worktree_path(worker_id);
        if !worktree.exists() {
            warn!(worker_id, path = %worktree.display(), "worker worktree missing; skipping queue entry");
            return Ok(false);
        }

        let plan =
            runner::plan_codex_run(Persona::Worker(worker_id), None, RunnerConfig::default());
        let mut docker_args = plan.docker_args;
        if docker_args.is_empty() {
            warn!(worker_id, "docker run command missing for worker persona");
            return Ok(false);
        }
        let program = docker_args.remove(0);
        docker_args.extend(plan.codex_args);

        let run_id = Uuid::new_v4();
        let metadata = RunMetadata {
            run_id,
            persona: format!("worker:ws{worker_id}"),
            workspace_root: PathBuf::from(PROJECT_DIR.as_str()),
            tags: vec![format!("worker:{worker_id}")],
            priority: RunPriority::Normal,
            issued_at: Utc::now(),
        };

        let intent = ProcessSpawnIntent {
            metadata,
            program,
            args: docker_args,
            env: Vec::new(),
            working_dir: PathBuf::from(PROJECT_DIR.as_str()),
            stream_stdout: true,
            stream_stderr: true,
            stdin: Some(Self::format_worker_prompt(message).into_bytes()),
        };

        let handle_rx = self
            .middleware
            .enqueue_spawn(intent)
            .await
            .map_err(|err| QueueManagerError::MiddlewareSend(err.to_string()))?;
        let handle = handle_rx
            .await
            .map_err(|_| QueueManagerError::ResponseDropped)?;

        self.state.worker_runs.insert(worker_id, run_id);
        Self::spawn_process_event_drain(worker_id, handle.events);
        self.state.active_workers.insert(worker_id);
        info!(
            worker_id,
            message_id = message.id,
            "dispatched worker turn from queue"
        );
        Ok(true)
    }

    async fn dispatch_orchestrator_message(
        &mut self,
        message: &Message,
    ) -> Result<bool, QueueManagerError> {
        let staging = staging_path();
        if !staging.exists() {
            warn!(path = %staging.display(), "staging directory missing; skipping orchestrator dispatch");
            return Ok(false);
        }

        let session_id = match db::session::get_session("orchestrator").await {
            Ok(value) => value,
            Err(err) => {
                warn!(?err, "failed to load orchestrator session id");
                None
            }
        };

        let plan = runner::plan_codex_run(
            Persona::Orchestrator,
            session_id.as_deref(),
            RunnerConfig::default(),
        );
        let mut docker_args = plan.docker_args;
        if docker_args.is_empty() {
            warn!("docker run command missing for orchestrator persona");
            return Ok(false);
        }
        let program = docker_args.remove(0);
        docker_args.extend(plan.codex_args);

        let run_id = Uuid::new_v4();
        let metadata = RunMetadata {
            run_id,
            persona: "orchestrator".to_string(),
            workspace_root: PathBuf::from(PROJECT_DIR.as_str()),
            tags: vec!["orchestrator".to_string()],
            priority: RunPriority::Normal,
            issued_at: Utc::now(),
        };

        let intent = ProcessSpawnIntent {
            metadata,
            program,
            args: docker_args,
            env: Vec::new(),
            working_dir: PathBuf::from(PROJECT_DIR.as_str()),
            stream_stdout: true,
            stream_stderr: true,
            stdin: Some(Self::format_orchestrator_prompt(message).into_bytes()),
        };

        let handle_rx = self
            .middleware
            .enqueue_spawn(intent)
            .await
            .map_err(|err| QueueManagerError::MiddlewareSend(err.to_string()))?;
        let handle = handle_rx
            .await
            .map_err(|_| QueueManagerError::ResponseDropped)?;

        self.state.orchestrator_run = Some(run_id);
        Self::spawn_agent_event_drain(AgentRunActor::Orchestrator, handle.events);
        info!(
            message_id = message.id,
            "dispatched orchestrator turn from queue"
        );
        Ok(true)
    }

    fn spawn_process_event_drain(worker_id: i64, events: mpsc::Receiver<ProcessEvent>) {
        Self::spawn_agent_event_drain(AgentRunActor::Worker(worker_id), events);
    }

    fn spawn_agent_event_drain(agent: AgentRunActor, mut events: mpsc::Receiver<ProcessEvent>) {
        spawn(async move {
            const MAX_CAPTURE: usize = 8 * 1024;
            let mut stdout_buf = Vec::with_capacity(MAX_CAPTURE);
            let mut stderr_buf = Vec::with_capacity(MAX_CAPTURE);

            while let Some(event) = events.recv().await {
                match event {
                    ProcessEvent::Output(chunk) => {
                        let target = match chunk.stream {
                            ProcessStream::Stdout => &mut stdout_buf,
                            ProcessStream::Stderr => &mut stderr_buf,
                        };
                        if target.len() < MAX_CAPTURE {
                            let remaining = MAX_CAPTURE - target.len();
                            let take = remaining.min(chunk.bytes.len());
                            target.extend_from_slice(&chunk.bytes[..take]);
                        }
                    }
                    ProcessEvent::Exit(exit) => {
                        match agent {
                            AgentRunActor::Worker(worker_id) => {
                                if !exit.status.success() {
                                    error!(
                                        worker_id,
                                        code = exit.status.code(),
                                        stdout = %String::from_utf8_lossy(&stdout_buf).trim(),
                                        stderr = %String::from_utf8_lossy(&stderr_buf).trim(),
                                        "worker process exited with failure"
                                    );
                                } else {
                                    info!(worker_id, "worker process completed");
                                }
                            }
                            AgentRunActor::Orchestrator => {
                                if !exit.status.success() {
                                    error!(
                                        code = exit.status.code(),
                                        stdout = %String::from_utf8_lossy(&stdout_buf).trim(),
                                        stderr = %String::from_utf8_lossy(&stderr_buf).trim(),
                                        "orchestrator process exited with failure"
                                    );
                                } else {
                                    info!("orchestrator process completed");
                                }
                            }
                        }
                        break;
                    }
                    ProcessEvent::SpawnFailed(err) => {
                        match agent {
                            AgentRunActor::Worker(worker_id) => {
                                error!(
                                    worker_id,
                                    message = err.message,
                                    "worker process failed to spawn"
                                );
                            }
                            AgentRunActor::Orchestrator => {
                                error!(
                                    message = err.message,
                                    "orchestrator process failed to spawn"
                                );
                            }
                        }
                        break;
                    }
                    ProcessEvent::Killed(killed) => {
                        match agent {
                            AgentRunActor::Worker(worker_id) => {
                                warn!(worker_id, reason = ?killed.reason, "worker process killed");
                            }
                            AgentRunActor::Orchestrator => {
                                warn!(reason = ?killed.reason, "orchestrator process killed");
                            }
                        }
                        break;
                    }
                    _ => {}
                }
            }
            match agent {
                AgentRunActor::Worker(worker_id) => {
                    debug!(worker_id, "worker process event channel closed");
                }
                AgentRunActor::Orchestrator => {
                    debug!("orchestrator process event channel closed");
                }
            }
        });
    }

    fn format_worker_prompt(message: &Message) -> String {
        let summary = message.message.trim();
        format!(
            "Queue message #{id} from {from} to {to}:\n\n{summary}\n\nRespond with a JSON object matching the Robot Farm worker turn schema.\n",
            id = message.id,
            from = message.from.trim(),
            to = message.to.trim(),
            summary = summary,
        )
    }

    fn parse_worker_target(target: &str) -> Option<i64> {
        let lowered = target.trim().to_ascii_lowercase();
        lowered.strip_prefix("ws")?.parse().ok()
    }

    fn format_assignment_message(turn: &OrchestratorTurn, assignment: &Assignment) -> String {
        let mut sections = Vec::new();
        sections.push(format!(
            "New assignment: {} ({})",
            assignment.task_slug, assignment.task_title
        ));
        if !turn.summary.trim().is_empty() {
            sections.push(format!("Summary: {}", turn.summary.trim()));
        }
        if let Some(details) = turn
            .details
            .as_deref()
            .map(str::trim)
            .filter(|d| !d.is_empty())
        {
            sections.push(format!("Details:\n{}", details));
        }
        if !assignment.steps.is_empty() {
            let steps = assignment
                .steps
                .iter()
                .enumerate()
                .map(|(idx, step)| format!("{}. {}", idx + 1, step))
                .collect::<Vec<_>>()
                .join("\n");
            sections.push(format!("Steps:\n{}", steps));
        }
        if let Some(acceptance) = assignment
            .acceptance
            .as_deref()
            .map(str::trim)
            .filter(|d| !d.is_empty())
        {
            sections.push(format!("Acceptance Criteria:\n{}", acceptance));
        }
        sections.push(
            "Respond with STATUS_UPDATE for long-running work and COMPLETE_TASK when finished."
                .to_string(),
        );
        sections.join("\n\n")
    }

    fn format_orchestrator_status(turn: &OrchestratorTurn) -> String {
        let mut parts = Vec::new();
        if !turn.summary.trim().is_empty() {
            parts.push(format!("Status: {}", turn.summary.trim()));
        }
        if let Some(details) = turn
            .details
            .as_deref()
            .map(str::trim)
            .filter(|d| !d.is_empty())
        {
            parts.push(details.to_string());
        }
        if parts.is_empty() {
            "Orchestrator sent a status update.".to_string()
        } else {
            parts.join("\n\n")
        }
    }

    fn format_orchestrator_prompt(message: &Message) -> String {
        let summary = message.message.trim();
        let footer = Self::strategy_footer();
        format!(
            "Queue message #{id} from {from} to {to}:\n\n{summary}\n\nRespond with a JSON object matching the Robot Farm orchestrator turn schema.\n\n{footer}\n",
            id = message.id,
            from = message.from.trim(),
            to = message.to.trim(),
            summary = summary,
            footer = footer,
        )
    }

    fn strategy_footer() -> String {
        let strategy = StrategyState::global().snapshot();
        let strategy_label = strategy.id.to_string();
        let focus = strategy
            .focus
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|group| group.to_string())
            .collect::<Vec<_>>();
        let focus_str = if focus.is_empty() {
            "none".to_string()
        } else {
            focus.join(", ")
        };
        format!(
            "Active Strategy: {} | Focus Groups: {}",
            strategy_label, focus_str
        )
    }

    fn format_status_update(worker_id: i64, turn: &WorkerTurn) -> Option<String> {
        let summary = turn.summary.trim();
        let details = turn
            .details
            .as_deref()
            .map(str::trim)
            .filter(|d| !d.is_empty());
        if summary.is_empty() && details.is_none() {
            return None;
        }

        let mut body = if summary.is_empty() {
            format!("ws{worker_id} sent a status update.")
        } else {
            format!("ws{worker_id}: {summary}")
        };
        if let Some(details) = details {
            body.push_str("\n\n");
            body.push_str(details);
        }
        Some(body)
    }

    fn record_support_hint(worker_id: i64) {
        let strategy = StrategyState::global().snapshot();
        let mut hints = vec![OrchestratorHint::SendSupport {
            to_worker: worker_id,
        }];
        if should_seed_assignments(&strategy.id) {
            let focus = strategy
                .focus
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|group| group.to_string())
                .collect();
            hints.push(OrchestratorHint::AssignTask {
                to_worker: worker_id,
                from_groups: focus,
            });
        }
        QueueCoordinator::global().record_assignment_hint(&hints);
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

    async fn kill_worker_process(&mut self, worker_id: i64) -> Result<(), QueueManagerError> {
        let run_id = *self
            .state
            .worker_runs
            .get(&worker_id)
            .ok_or(QueueManagerError::WorkerNotRunning(worker_id))?;
        info!(worker_id, %run_id, "issuing kill for worker run");
        self.middleware
            .enqueue_kill(run_id, KillReason::UserRequested)
            .await
            .map_err(|err| QueueManagerError::MiddlewareSend(err.to_string()))
    }

    async fn kill_orchestrator_process(&mut self) -> Result<(), QueueManagerError> {
        let run_id = self
            .state
            .orchestrator_run
            .ok_or(QueueManagerError::OrchestratorNotRunning)?;
        info!(%run_id, "issuing kill for orchestrator run");
        self.middleware
            .enqueue_kill(run_id, KillReason::UserRequested)
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
        if !events.is_empty() {
            info!(count = events.len(), "flushing system events to feed");
        }
        for event in events {
            let entry = event_to_feed_entry(event);
            match self.db.insert_feed_entry(entry).await {
                Ok(feed_entry) => {
                    realtime::publish(RealtimeEvent::FeedEntry(sanitize_feed_entry(&feed_entry)));
                }
                Err(err) => warn!(?err, "failed to persist system event"),
            }
        }
    }
}

struct QueueRuntimeState {
    paused: bool,
    buffered: VecDeque<ProcessIntent>,
    active_workers: HashSet<i64>,
    worker_runs: HashMap<i64, RunId>,
    orchestrator_run: Option<RunId>,
}

impl Default for QueueRuntimeState {
    fn default() -> Self {
        Self {
            paused: true,
            buffered: VecDeque::new(),
            active_workers: HashSet::new(),
            worker_runs: HashMap::new(),
            orchestrator_run: None,
        }
    }
}

fn should_seed_assignments(strategy: &ApiStrategy) -> bool {
    !matches!(strategy, ApiStrategy::Planning | ApiStrategy::WindDown)
}

async fn run_queue_manager(mut runtime: QueueManagerRuntime) {
    info!("queue manager loop started");
    let mut tick = interval(Duration::from_millis(500));
    tick.set_missed_tick_behavior(MissedTickBehavior::Skip);
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
            _ = tick.tick() => {
                if let Err(err) = runtime.drive_queue().await {
                    warn!(?err, "queue manager drive tick failed");
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

struct PostTurnJob {
    worker_id: i64,
    completion: WorkerCompletion,
    middleware: MiddlewareHandle,
    db: DatabaseManagerHandle,
}

impl PostTurnJob {
    fn spawn(
        worker_id: i64,
        completion: WorkerCompletion,
        middleware: MiddlewareHandle,
        db: DatabaseManagerHandle,
    ) {
        let job = Self {
            worker_id,
            completion,
            middleware,
            db,
        };
        spawn(async move {
            job.run().await;
        });
    }

    async fn run(self) {
        match self.execute().await {
            Ok(_) => {
                if let Err(err) = self.notify_orchestrator_completion().await {
                    warn!(
                        worker_id = self.worker_id,
                        ?err,
                        "failed to notify orchestrator completion"
                    );
                }
                self.clear_worker_session().await;
            }
            Err(error) => {
                if let Err(err) = self.notify_worker_failure(&error).await {
                    warn!(
                        worker_id = self.worker_id,
                        ?err,
                        "failed to notify worker about post-turn failure"
                    );
                }
            }
        }
    }

    async fn execute(&self) -> Result<(), PostTurnError> {
        self.run_post_turn_checks().await?;
        auto_commit_and_merge(self.worker_id, &self.completion).await?;
        Ok(())
    }

    async fn run_post_turn_checks(&self) -> Result<(), PostTurnError> {
        let plan = PostTurnCheckRegistry::global().list();
        if plan.is_empty() {
            return Ok(());
        }
        let registry = ProjectCommandRegistry::global();
        for check_id in plan {
            let command = registry
                .get(&check_id)
                .ok_or_else(|| PostTurnError::UnknownCheck(check_id.clone()))?;
            let result = self.run_check_command(&check_id, &command).await?;
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
        &self,
        check_id: &str,
        command: &CommandConfig,
    ) -> Result<ProcessRunResult, PostTurnError> {
        if command.exec.is_empty() {
            return Err(PostTurnError::InvalidCommand(check_id.to_string()));
        }

        let workspace_root = Path::new(PROJECT_DIR.as_str());
        let worker_root = worker_worktree_path(self.worker_id);
        let working_dir =
            shell::resolve_working_dir(workspace_root, &worker_root, command.cwd.as_deref())
                .map_err(|err| PostTurnError::InvalidWorkingDir(err.to_string()))?;

        let metadata = RunMetadata {
            run_id: Uuid::new_v4(),
            persona: format!("post_turn_check:{check_id}"),
            workspace_root: workspace_root.to_path_buf(),
            tags: vec![
                "post_turn_check".to_string(),
                format!("worker:{}", self.worker_id),
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

        Self::await_process(handle).await
    }

    async fn await_process(mut handle: ProcessHandle) -> Result<ProcessRunResult, PostTurnError> {
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

    async fn notify_worker_failure(&self, error: &PostTurnError) -> Result<(), QueueManagerError> {
        let message = error.render(self.worker_id);
        self.db
            .enqueue_message(
                SystemActor::System.label(),
                SystemActor::Worker(self.worker_id).label(),
                message.clone(),
            )
            .await
            .map_err(QueueManagerError::from)?;
        QueueCoordinator::global().validation_failed(self.worker_id, &message);
        Ok(())
    }

    async fn notify_orchestrator_completion(&self) -> Result<(), QueueManagerError> {
        let mut message = format!(
            "ws{} completed {}",
            self.worker_id, self.completion.task_slug
        );
        if let Some(notes) = self
            .completion
            .notes
            .as_ref()
            .and_then(|v| (!v.trim().is_empty()).then_some(v.trim()))
        {
            message.push_str(": ");
            message.push_str(notes);
        }
        self.db
            .enqueue_message(
                SystemActor::System.label(),
                SystemActor::Orchestrator.label(),
                message,
            )
            .await
            .map_err(QueueManagerError::from)?;
        Ok(())
    }

    async fn clear_worker_session(&self) {
        let owner = format!("ws{}", self.worker_id);
        if let Err(err) = db::session::delete_session(&owner).await {
            warn!(
                worker_id = self.worker_id,
                ?err,
                "failed to clear worker session"
            );
        } else {
            realtime::publish(RealtimeEvent::WorkerThread {
                worker_id: self.worker_id,
                thread_id: None,
            });
        }
    }
}

async fn auto_commit_and_merge(
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
        Err(git::GitError::CommandFailure { stderr }) if stderr.contains("nothing to commit") => {
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
            let conflicts = git::collect_merge_conflicts(&staging).map_err(PostTurnError::Git)?;
            let _ = git::abort_merge(&staging);
            return Err(PostTurnError::MergeConflict(conflicts));
        }
    }

    Ok(())
}

fn sanitize_feed_entry(entry: &Feed) -> Feed {
    let mut sanitized = entry.clone();
    sanitized.raw.clear();
    sanitized
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
    #[error("worker {0} is not running")]
    WorkerNotRunning(i64),
    #[error("orchestrator is not running")]
    OrchestratorNotRunning,
    #[error("assignment processing failed: {0}")]
    Assignment(String),
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

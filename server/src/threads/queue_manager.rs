use crate::db::feed::NewFeedEntry;
use crate::db::message_queue::{MessageFilters, RelativePosition};
use crate::models::process::ProcessIntent;
use crate::system::{events::SystemEvent, queue::QueueCoordinator};
use crate::threads::database_manager::{DatabaseManagerError, DatabaseManagerHandle};
use crate::threads::middleware::MiddlewareHandle;
use openapi::models::Message;
use std::collections::VecDeque;
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, warn};

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
) -> QueueManagerHandle {
    let (tx, rx) = mpsc::channel(config.command_buffer);
    let runtime = QueueManagerRuntime {
        db,
        middleware,
        rx,
        state: QueueRuntimeState::default(),
    };
    tokio::spawn(run_queue_manager(runtime));
    QueueManagerHandle { tx }
}

struct QueueManagerRuntime {
    db: DatabaseManagerHandle,
    middleware: MiddlewareHandle,
    rx: mpsc::Receiver<QueueManagerCommand>,
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
            if let Err(err) = self.db.insert_feed_entry(entry).await {
                warn!(?err, "failed to persist system event");
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
    while let Some(command) = runtime.rx.recv().await {
        if let Err(err) = runtime.handle_command(command).await {
            warn!(?err, "queue manager command failed");
        }
        runtime.flush_system_events().await;
    }
    warn!("queue manager channel closed; exiting loop");
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

#[derive(Debug, thiserror::Error)]
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

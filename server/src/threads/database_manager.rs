use crate::db::feed::{self, NewFeedEntry};
use crate::db::message_queue::{self, MessageFilters, MessageQueueError, RelativePosition};
use openapi::models::{Feed, Message};
use tokio::sync::{mpsc, oneshot};
use tracing::error;

/// Configuration for the database manager loop.
pub struct DatabaseManagerConfig {
    pub command_buffer: usize,
}

impl Default for DatabaseManagerConfig {
    fn default() -> Self {
        Self { command_buffer: 64 }
    }
}

#[derive(Clone)]
pub struct DatabaseManagerHandle {
    tx: mpsc::Sender<DatabaseManagerCommand>,
}

impl DatabaseManagerHandle {
    pub async fn list_messages(
        &self,
        filters: MessageFilters,
    ) -> Result<Vec<Message>, DatabaseManagerError> {
        self.request(|respond_to| DatabaseManagerCommand::ListMessages {
            filters,
            respond_to,
        })
        .await
    }

    pub async fn delete_all_messages(&self) -> Result<(), DatabaseManagerError> {
        self.request(|respond_to| DatabaseManagerCommand::DeleteAllMessages { respond_to })
            .await
    }

    pub async fn delete_message_by_id(
        &self,
        message_id: i64,
    ) -> Result<bool, DatabaseManagerError> {
        self.request(|respond_to| DatabaseManagerCommand::DeleteMessageById {
            message_id,
            respond_to,
        })
        .await
    }

    pub async fn delete_messages_for_recipient(
        &self,
        recipient: String,
    ) -> Result<u64, DatabaseManagerError> {
        self.request(
            |respond_to| DatabaseManagerCommand::DeleteMessagesForRecipient {
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
    ) -> Result<Vec<Message>, DatabaseManagerError> {
        self.request(|respond_to| DatabaseManagerCommand::InsertMessageRelative {
            message_id,
            directive,
            respond_to,
        })
        .await
    }

    pub async fn insert_feed_entry(
        &self,
        entry: NewFeedEntry,
    ) -> Result<Feed, DatabaseManagerError> {
        self.request(|respond_to| DatabaseManagerCommand::InsertFeedEntry { entry, respond_to })
            .await
    }

    pub async fn enqueue_message(
        &self,
        from_actor: String,
        to_actor: String,
        body: String,
    ) -> Result<Message, DatabaseManagerError> {
        self.request(|respond_to| DatabaseManagerCommand::CreateMessage {
            from_actor,
            to_actor,
            body,
            respond_to,
        })
        .await
    }

    async fn request<T>(
        &self,
        build: impl FnOnce(oneshot::Sender<Result<T, DatabaseManagerError>>) -> DatabaseManagerCommand,
    ) -> Result<T, DatabaseManagerError>
    where
        T: Send + 'static,
    {
        let (respond_to, response) = oneshot::channel();
        let command = build(respond_to);
        self.tx
            .send(command)
            .await
            .map_err(|_| DatabaseManagerError::ChannelClosed)?;
        let result = response
            .await
            .map_err(|_| DatabaseManagerError::ResponseDropped)?;
        result
    }
}

enum DatabaseManagerCommand {
    ListMessages {
        filters: MessageFilters,
        respond_to: oneshot::Sender<Result<Vec<Message>, DatabaseManagerError>>,
    },
    DeleteAllMessages {
        respond_to: oneshot::Sender<Result<(), DatabaseManagerError>>,
    },
    DeleteMessageById {
        message_id: i64,
        respond_to: oneshot::Sender<Result<bool, DatabaseManagerError>>,
    },
    DeleteMessagesForRecipient {
        recipient: String,
        respond_to: oneshot::Sender<Result<u64, DatabaseManagerError>>,
    },
    InsertMessageRelative {
        message_id: i64,
        directive: RelativePosition,
        respond_to: oneshot::Sender<Result<Vec<Message>, DatabaseManagerError>>,
    },
    InsertFeedEntry {
        entry: NewFeedEntry,
        respond_to: oneshot::Sender<Result<Feed, DatabaseManagerError>>,
    },
    CreateMessage {
        from_actor: String,
        to_actor: String,
        body: String,
        respond_to: oneshot::Sender<Result<Message, DatabaseManagerError>>,
    },
}

pub fn spawn_database_manager(config: DatabaseManagerConfig) -> DatabaseManagerHandle {
    let (tx, rx) = mpsc::channel(config.command_buffer);
    tokio::spawn(run_manager(rx));
    DatabaseManagerHandle { tx }
}

async fn run_manager(mut rx: mpsc::Receiver<DatabaseManagerCommand>) {
    while let Some(command) = rx.recv().await {
        // Each operation owns the connection work so the rest of the system can
        // keep sending intents without blocking on SQLx calls.
        match command {
            DatabaseManagerCommand::ListMessages {
                filters,
                respond_to,
            } => {
                let result = message_queue::list_messages(filters)
                    .await
                    .map_err(DatabaseManagerError::from);
                let _ = respond_to.send(result);
            }
            DatabaseManagerCommand::DeleteAllMessages { respond_to } => {
                let result = message_queue::delete_all_messages()
                    .await
                    .map_err(DatabaseManagerError::from);
                let _ = respond_to.send(result);
            }
            DatabaseManagerCommand::DeleteMessageById {
                message_id,
                respond_to,
            } => {
                let result = message_queue::delete_message_by_id(message_id)
                    .await
                    .map_err(DatabaseManagerError::from);
                let _ = respond_to.send(result);
            }
            DatabaseManagerCommand::DeleteMessagesForRecipient {
                recipient,
                respond_to,
            } => {
                let result = message_queue::delete_messages_for_recipient(&recipient)
                    .await
                    .map_err(DatabaseManagerError::from);
                let _ = respond_to.send(result);
            }
            DatabaseManagerCommand::InsertMessageRelative {
                message_id,
                directive,
                respond_to,
            } => {
                let result = message_queue::insert_message_relative(message_id, directive)
                    .await
                    .map_err(DatabaseManagerError::from);
                let _ = respond_to.send(result);
            }
            DatabaseManagerCommand::InsertFeedEntry { entry, respond_to } => {
                let result = feed::insert_feed_entry(entry)
                    .await
                    .map_err(DatabaseManagerError::from);
                let _ = respond_to.send(result);
            }
            DatabaseManagerCommand::CreateMessage {
                from_actor,
                to_actor,
                body,
                respond_to,
            } => {
                let result = message_queue::enqueue_message(&from_actor, &to_actor, &body)
                    .await
                    .map_err(DatabaseManagerError::from);
                let _ = respond_to.send(result);
            }
        }
    }
    error!("database manager channel closed; exiting loop");
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseManagerError {
    #[error("database manager channel closed")]
    ChannelClosed,
    #[error("database manager response channel closed")]
    ResponseDropped,
    #[error("database query failed: {0}")]
    Query(#[from] sqlx::Error),
    #[error("message queue error: {0}")]
    MessageQueue(#[from] MessageQueueError),
}

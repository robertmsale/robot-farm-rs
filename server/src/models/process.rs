use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::ExitStatus;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

pub type RunId = Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RunPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl RunPriority {
    pub fn sort_key(&self) -> u8 {
        match self {
            RunPriority::Critical => 0,
            RunPriority::High => 1,
            RunPriority::Normal => 2,
            RunPriority::Low => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RunMetadata {
    pub run_id: RunId,
    pub persona: String,
    pub workspace_root: PathBuf,
    pub tags: Vec<String>,
    pub priority: RunPriority,
    pub issued_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ProcessSpawnIntent {
    pub metadata: RunMetadata,
    pub program: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub working_dir: PathBuf,
    pub stream_stdout: bool,
    pub stream_stderr: bool,
    pub stdin: Option<Vec<u8>>,
}

pub struct SpawnRequest {
    pub intent: ProcessSpawnIntent,
    pub handle_tx: oneshot::Sender<ProcessHandle>,
}

impl std::fmt::Debug for SpawnRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpawnRequest")
            .field("intent", &self.intent)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct ProcessKillIntent {
    pub run_id: RunId,
    pub reason: KillReason,
    pub issued_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum ProcessIntent {
    Spawn(SpawnRequest),
    Kill(ProcessKillIntent),
    AdjustPriority {
        run_id: RunId,
        new_priority: RunPriority,
    },
}

#[derive(Debug)]
pub enum ProcessDirective {
    Launch(ProcessLaunchDirective),
    Kill(ProcessKillDirective),
    UpdatePriority(ProcessPriorityDirective),
}

pub struct ProcessLaunchDirective {
    pub request: ProcessRequest,
    pub handle_tx: oneshot::Sender<ProcessHandle>,
}

impl std::fmt::Debug for ProcessLaunchDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProcessLaunchDirective")
            .field("request", &self.request)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct ProcessKillDirective {
    pub run_id: RunId,
    pub reason: KillReason,
}

#[derive(Debug, Clone)]
pub struct ProcessPriorityDirective {
    pub run_id: RunId,
    pub new_priority: RunPriority,
}

#[derive(Debug, Clone)]
pub struct ProcessRequest {
    pub metadata: RunMetadata,
    pub program: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub working_dir: PathBuf,
    pub stream_stdout: bool,
    pub stream_stderr: bool,
    pub stdin: Option<Vec<u8>>,
}

impl From<ProcessSpawnIntent> for ProcessRequest {
    fn from(value: ProcessSpawnIntent) -> Self {
        Self {
            metadata: value.metadata,
            program: value.program,
            args: value.args,
            env: value.env,
            working_dir: value.working_dir,
            stream_stdout: value.stream_stdout,
            stream_stderr: value.stream_stderr,
            stdin: value.stdin,
        }
    }
}

impl From<SpawnRequest> for ProcessLaunchDirective {
    fn from(value: SpawnRequest) -> Self {
        Self {
            request: value.intent.into(),
            handle_tx: value.handle_tx,
        }
    }
}

#[derive(Debug)]
pub enum ProcessEvent {
    Output(ProcessOutputChunk),
    OutputError(ProcessOutputError),
    Exit(ProcessExit),
    SpawnFailed(ProcessSpawnError),
    Killed(ProcessKilled),
}

#[derive(Debug, Clone)]
pub struct ProcessOutputChunk {
    pub run_id: RunId,
    pub stream: ProcessStream,
    pub bytes: Vec<u8>,
    pub sequence: u64,
    pub captured_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ProcessOutputError {
    pub run_id: RunId,
    pub stream: ProcessStream,
    pub message: String,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStream {
    Stdout,
    Stderr,
}

#[derive(Debug)]
pub struct ProcessExit {
    pub run_id: RunId,
    pub status: ExitStatus,
    pub finished_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ProcessSpawnError {
    pub run_id: RunId,
    pub message: String,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ProcessKilled {
    pub run_id: RunId,
    pub reason: KillReason,
    pub finished_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum KillReason {
    UserRequested,
    Timeout(Duration),
    ReplacedBy { superseding: RunId },
    DependencyFailed { dependency: RunId },
    Shutdown,
}

#[derive(Debug)]
pub struct ProcessHandle {
    pub run_id: RunId,
    pub events: mpsc::Receiver<ProcessEvent>,
    pub kill: ProcessKillHandle,
}

#[derive(Clone, Debug)]
pub struct ProcessKillHandle {
    sender: mpsc::Sender<KillSignal>,
}

impl ProcessKillHandle {
    pub fn new(sender: mpsc::Sender<KillSignal>) -> Self {
        Self { sender }
    }

    pub async fn kill(&self, reason: KillReason) -> Result<(), KillHandleError> {
        self.sender
            .send(KillSignal {
                reason,
                requested_at: Utc::now(),
            })
            .await
            .map_err(|_| KillHandleError::ChannelClosed)
    }
}

#[derive(Debug, Error)]
pub enum KillHandleError {
    #[error("kill channel closed before signal was delivered")]
    ChannelClosed,
}

#[derive(Debug, Clone)]
pub struct KillSignal {
    pub reason: KillReason,
    pub requested_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum ProcessLifecycleEvent {
    Starting { run_id: RunId },
    Finished { run_id: RunId },
    Failed { run_id: RunId },
    Killed { run_id: RunId },
}

#[derive(Debug, Default)]
pub struct MiddlewareState {
    inflight: HashMap<RunId, MiddlewareRunState>,
}

impl MiddlewareState {
    pub fn track_intents(&mut self, intents: &[ProcessIntent]) {
        for intent in intents {
            match intent {
                ProcessIntent::Spawn(spawn) => {
                    self.inflight.insert(
                        spawn.intent.metadata.run_id,
                        MiddlewareRunState {
                            metadata: spawn.intent.metadata.clone(),
                            desired_priority: spawn.intent.metadata.priority,
                            cancel_requested: false,
                        },
                    );
                }
                ProcessIntent::Kill(kill) => {
                    if let Some(state) = self.inflight.get_mut(&kill.run_id) {
                        state.cancel_requested = true;
                    }
                }
                ProcessIntent::AdjustPriority {
                    run_id,
                    new_priority,
                } => {
                    if let Some(state) = self.inflight.get_mut(run_id) {
                        state.desired_priority = *new_priority;
                    }
                }
            }
        }
    }

    pub fn reduce_batch(&mut self, batch: Vec<ProcessIntent>) -> Vec<ProcessDirective> {
        if batch.is_empty() {
            return Vec::new();
        }

        self.track_intents(&batch);

        let mut kill_directives = Vec::new();
        let mut spawn_directives = Vec::new();
        let mut priority_directives = Vec::new();

        for intent in batch {
            match intent {
                ProcessIntent::Spawn(spawn) => {
                    spawn_directives.push(spawn);
                }
                ProcessIntent::Kill(kill) => {
                    kill_directives.push(ProcessDirective::Kill(ProcessKillDirective {
                        run_id: kill.run_id,
                        reason: kill.reason,
                    }));
                }
                ProcessIntent::AdjustPriority {
                    run_id,
                    new_priority,
                } => {
                    priority_directives.push(ProcessDirective::UpdatePriority(
                        ProcessPriorityDirective {
                            run_id,
                            new_priority,
                        },
                    ));
                }
            }
        }

        spawn_directives.sort_by_key(|intent| intent.intent.metadata.priority.sort_key());

        let mut directives = kill_directives;
        directives.extend(
            spawn_directives
                .into_iter()
                .map(|request| ProcessDirective::Launch(ProcessLaunchDirective::from(request))),
        );
        directives.extend(priority_directives);
        directives
    }

    pub fn apply_lifecycle_event(&mut self, event: ProcessLifecycleEvent) {
        match event {
            ProcessLifecycleEvent::Starting { .. } => {}
            ProcessLifecycleEvent::Finished { run_id }
            | ProcessLifecycleEvent::Failed { run_id }
            | ProcessLifecycleEvent::Killed { run_id } => {
                self.inflight.remove(&run_id);
            }
        }
    }

    pub fn inflight_count(&self) -> usize {
        self.inflight.len()
    }
}

#[derive(Debug, Clone)]
pub struct MiddlewareRunState {
    pub metadata: RunMetadata,
    pub desired_priority: RunPriority,
    pub cancel_requested: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_event_drops_inflight_entry() {
        let run_id = RunId::new_v4();
        let metadata = RunMetadata {
            run_id,
            persona: "tester".into(),
            workspace_root: PathBuf::from("."),
            tags: vec![],
            priority: RunPriority::Normal,
            issued_at: Utc::now(),
        };

        let mut state = MiddlewareState::default();
        state.inflight.insert(
            run_id,
            MiddlewareRunState {
                metadata,
                desired_priority: RunPriority::Normal,
                cancel_requested: false,
            },
        );

        state.apply_lifecycle_event(ProcessLifecycleEvent::Finished { run_id });

        assert_eq!(state.inflight_count(), 0);
    }
}

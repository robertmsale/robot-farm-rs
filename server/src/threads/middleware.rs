use crate::models::process::{
    KillReason, MiddlewareState, ProcessDirective, ProcessHandle, ProcessIntent,
    ProcessLifecycleEvent, ProcessSpawnIntent, RunId, RunPriority, SpawnRequest,
};
use chrono::Utc;
use std::time::Duration;
use tokio::sync::{
    mpsc,
    mpsc::{Receiver, error::TryRecvError},
    oneshot,
};
use tokio::time::{Instant, timeout};
use tracing::{info, warn};

pub struct MiddlewareConfig {
    pub batch_window: Duration,
    pub max_batch: usize,
    pub intent_buffer: usize,
    pub directive_buffer: usize,
}

impl Default for MiddlewareConfig {
    fn default() -> Self {
        Self {
            batch_window: Duration::from_millis(500),
            max_batch: 64,
            intent_buffer: 256,
            directive_buffer: 256,
        }
    }
}

#[derive(Clone)]
pub struct MiddlewareHandle {
    tx: mpsc::Sender<ProcessIntent>,
}

impl MiddlewareHandle {
    pub fn sender(&self) -> mpsc::Sender<ProcessIntent> {
        self.tx.clone()
    }

    pub async fn enqueue_spawn(
        &self,
        intent: ProcessSpawnIntent,
    ) -> Result<oneshot::Receiver<ProcessHandle>, mpsc::error::SendError<ProcessIntent>> {
        let (handle_tx, handle_rx) = oneshot::channel();
        let request = SpawnRequest { intent, handle_tx };
        self.tx.send(ProcessIntent::Spawn(request)).await?;
        Ok(handle_rx)
    }

    pub async fn enqueue_kill(
        &self,
        run_id: RunId,
        reason: KillReason,
    ) -> Result<(), mpsc::error::SendError<ProcessIntent>> {
        let intent = crate::models::process::ProcessKillIntent {
            run_id,
            reason,
            issued_at: Utc::now(),
        };
        self.tx.send(ProcessIntent::Kill(intent)).await
    }

    pub async fn adjust_priority(
        &self,
        run_id: crate::models::process::RunId,
        new_priority: RunPriority,
    ) -> Result<(), mpsc::error::SendError<ProcessIntent>> {
        self.tx
            .send(ProcessIntent::AdjustPriority {
                run_id,
                new_priority,
            })
            .await
    }
}

pub fn spawn_middleware(
    config: MiddlewareConfig,
    lifecycle_rx: Receiver<ProcessLifecycleEvent>,
) -> (MiddlewareHandle, mpsc::Receiver<ProcessDirective>) {
    let (intent_tx, intent_rx) = mpsc::channel(config.intent_buffer);
    let (directive_tx, directive_rx) = mpsc::channel(config.directive_buffer);

    tokio::spawn(run_middleware_loop(
        config,
        intent_rx,
        lifecycle_rx,
        directive_tx,
    ));

    (MiddlewareHandle { tx: intent_tx }, directive_rx)
}

async fn run_middleware_loop(
    config: MiddlewareConfig,
    mut intents_rx: mpsc::Receiver<ProcessIntent>,
    mut lifecycle_rx: Receiver<ProcessLifecycleEvent>,
    directives_tx: mpsc::Sender<ProcessDirective>,
) {
    info!("middleware loop started");
    let mut state = MiddlewareState::default();
    let mut batch = Vec::with_capacity(config.max_batch);
    let mut intents_closed = false;
    'outer: loop {
        drain_lifecycle_events(&mut lifecycle_rx, &mut state);

        if intents_closed && batch.is_empty() {
            break;
        }

        if batch.is_empty() {
            match intents_rx.recv().await {
                Some(intent) => {
                    if is_priority_intent(&intent) {
                        dispatch_batch(vec![intent], &mut state, &directives_tx).await;
                        continue;
                    }
                    batch.push(intent)
                }
                None => {
                    intents_closed = true;
                    continue;
                }
            }
        }

        let deadline = Instant::now() + config.batch_window;
        while batch.len() < config.max_batch {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                break;
            }

            match timeout(remaining, intents_rx.recv()).await {
                Ok(Some(intent)) => {
                    if is_priority_intent(&intent) {
                        if !batch.is_empty() {
                            dispatch_batch(std::mem::take(&mut batch), &mut state, &directives_tx)
                                .await;
                        }
                        dispatch_batch(vec![intent], &mut state, &directives_tx).await;
                        continue 'outer;
                    }
                    batch.push(intent)
                }
                Ok(None) => {
                    intents_closed = true;
                    break;
                }
                Err(_) => break,
            }

            drain_lifecycle_events(&mut lifecycle_rx, &mut state);
        }

        if batch.is_empty() {
            continue;
        }

        dispatch_batch(std::mem::take(&mut batch), &mut state, &directives_tx).await;
    }

    info!("middleware loop exited");
}

fn is_priority_intent(intent: &ProcessIntent) -> bool {
    matches!(
        intent,
        ProcessIntent::Kill(_) | ProcessIntent::AdjustPriority { .. }
    )
}

async fn dispatch_batch(
    batch: Vec<ProcessIntent>,
    state: &mut MiddlewareState,
    directives_tx: &mpsc::Sender<ProcessDirective>,
) {
    if batch.is_empty() {
        return;
    }
    info!(
        batch_size = batch.len(),
        "middleware reducing batch of intents"
    );
    let directives = state.reduce_batch(batch);
    info!(count = directives.len(), "middleware produced directives");
    for directive in directives {
        if directives_tx.send(directive).await.is_err() {
            warn!("process manager dropped directive channel");
            break;
        }
    }
}

fn drain_lifecycle_events(
    lifecycle_rx: &mut Receiver<ProcessLifecycleEvent>,
    state: &mut MiddlewareState,
) {
    loop {
        match lifecycle_rx.try_recv() {
            Ok(event) => state.apply_lifecycle_event(event),
            Err(TryRecvError::Empty) => break,
            Err(TryRecvError::Disconnected) => break,
        }
    }
}

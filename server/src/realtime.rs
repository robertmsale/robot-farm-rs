use once_cell::sync::OnceCell;
use openapi::models::Strategy;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum RealtimeEvent {
    FeedEntry(openapi::models::Feed),
    FeedCleared,
    QueueState {
        paused: bool,
    },
    StrategyState {
        id: Strategy,
        focus: Vec<i64>,
    },
    WorkersSnapshot {
        workers: Vec<openapi::models::Worker>,
    },
}

static CHANNEL: OnceCell<broadcast::Sender<RealtimeEvent>> = OnceCell::new();

pub fn init() {
    CHANNEL.get_or_init(|| {
        let (tx, _rx) = broadcast::channel(1024);
        tx
    });
}

pub fn subscribe() -> broadcast::Receiver<RealtimeEvent> {
    CHANNEL
        .get()
        .expect("realtime channel not initialized")
        .subscribe()
}

pub fn publish(event: RealtimeEvent) {
    if let Some(tx) = CHANNEL.get() {
        let _ = tx.send(event);
    }
}

use crate::{
    db,
    realtime::{self, RealtimeEvent},
    system::{queue::QueueCoordinator, strategy::StrategyState},
};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio::select;
use tokio::sync::broadcast;
use tracing::debug;

pub async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    if let Err(error) = socket.send(Message::Text("ready".into())).await {
        debug!(?error, "failed to send websocket greeting");
        return;
    }

    if let Err(error) = send_worker_snapshot(&mut socket).await {
        debug!(?error, "failed to send initial worker snapshot");
    }

    if let Err(error) = send_queue_state(&mut socket).await {
        debug!(?error, "failed to send initial queue state");
    }

    if let Err(error) = send_strategy_state(&mut socket).await {
        debug!(?error, "failed to send initial strategy state");
    }

    let (mut sender, mut receiver) = socket.split();
    let mut feed_rx = realtime::subscribe();

    loop {
        select! {
            Some(message) = receiver.next() => {
                match message {
                    Ok(Message::Close(_)) => break,
                    Ok(_) => continue,
                    Err(_) => break,
                }
            }
            event = feed_rx.recv() => {
                match event {
                    Ok(RealtimeEvent::FeedEntry(entry)) => {
                        let payload = json!({"type": "feed_entry", "entry": entry});
                        if sender
                            .send(Message::Text(payload.to_string().into()))
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                    Ok(RealtimeEvent::QueueState { paused }) => {
                        let payload = json!({"type": "queue_state", "paused": paused});
                        if sender
                            .send(Message::Text(payload.to_string().into()))
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                    Ok(RealtimeEvent::StrategyState { id, focus }) => {
                        let payload = json!({"type": "strategy_state", "strategy": {
                            "id": id,
                            "focus": focus,
                        }});
                        if sender
                            .send(Message::Text(payload.to_string().into()))
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }
}

async fn send_worker_snapshot(socket: &mut WebSocket) -> Result<(), axum::Error> {
    let workers = db::worker::list_workers().await;
    let payload = json!({
        "type": "workers_snapshot",
        "workers": workers,
    });
    socket.send(Message::Text(payload.to_string().into())).await
}

async fn send_queue_state(socket: &mut WebSocket) -> Result<(), axum::Error> {
    let paused = QueueCoordinator::global().is_paused();
    let payload = json!({
        "type": "queue_state",
        "paused": paused,
    });
    socket.send(Message::Text(payload.to_string().into())).await
}

async fn send_strategy_state(socket: &mut WebSocket) -> Result<(), axum::Error> {
    let strategy = StrategyState::global().snapshot();
    let payload = json!({
        "type": "strategy_state",
        "strategy": {
            "id": strategy.id,
            "focus": strategy.focus.unwrap_or_default(),
        }
    });
    socket.send(Message::Text(payload.to_string().into())).await
}

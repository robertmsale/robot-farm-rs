use crate::{
    db,
    realtime::{self, RealtimeEvent},
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

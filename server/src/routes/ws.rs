use crate::db;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use serde_json::json;
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

    while let Some(Ok(Message::Text(text))) = socket.recv().await {
        let _ = socket.send(Message::Text(text)).await;
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

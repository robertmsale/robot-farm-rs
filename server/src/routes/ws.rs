use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use tracing::debug;

pub async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    if let Err(error) = socket.send(Message::Text("ready".into())).await {
        debug!(?error, "failed to send websocket greeting");
        return;
    }

    while let Some(Ok(Message::Text(text))) = socket.recv().await {
        let _ = socket.send(Message::Text(text)).await;
    }
}

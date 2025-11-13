use std::{convert::Infallible, time::Duration};

use async_stream::stream;
use axum::{
    Json,
    body::Bytes,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{
        IntoResponse, Response,
        sse::{Event, KeepAlive, Sse},
    },
};
use serde_json::json;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

use crate::mcp::{self, Agent, AgentHeaderError, session::SessionManager};

pub async fn handle_mcp_request(headers: HeaderMap, body: Bytes) -> impl IntoResponse {
    let agent = match resolve_agent(&headers) {
        Ok(agent) => agent,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };

    let session_id = match parse_session_id(&headers) {
        Ok(value) => value,
        Err(err) => return (StatusCode::BAD_REQUEST, err).into_response(),
    };

    let response = mcp::handle_http_request(agent, session_id, &body).await;
    let mut http_response: Response = match response.body {
        Some(value) => Json(value).into_response(),
        None => StatusCode::NO_CONTENT.into_response(),
    };
    *http_response.status_mut() = response.status;
    if let Some(session_id) = response.session_id {
        if let Ok(value) = HeaderValue::from_str(&session_id) {
            http_response.headers_mut().insert("Mcp-Session-Id", value);
        }
    }
    http_response
}

pub async fn stream_mcp(headers: HeaderMap) -> impl IntoResponse {
    let agent = match resolve_agent(&headers) {
        Ok(agent) => agent,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };

    // agent currently unused but parsed to enforce header correctness
    let _ = agent;

    let session_id = match require_session_id(&headers) {
        Ok(value) => value,
        Err(err) => return (StatusCode::BAD_REQUEST, err).into_response(),
    };

    match SessionManager::global().subscribe(&session_id).await {
        Some(receiver) => {
            let stream = stream! {
                yield Ok::<Event, Infallible>(Event::default().event("ready").data(json!({"status": "listening"}).to_string()));
                let mut rx_stream = BroadcastStream::new(receiver);
                while let Some(message) = rx_stream.next().await {
                    if let Ok(value) = message {
                        if let Ok(text) = serde_json::to_string(&value) {
                            yield Ok::<Event, Infallible>(Event::default().event("message").data(text));
                        }
                    }
                }
            };
            Sse::new(stream)
                .keep_alive(
                    KeepAlive::new()
                        .interval(Duration::from_secs(15))
                        .text(": keep-alive"),
                )
                .into_response()
        }
        None => (StatusCode::NOT_FOUND, "unknown session id").into_response(),
    }
}

fn resolve_agent(headers: &HeaderMap) -> Result<Agent, AgentHeaderError> {
    let raw = headers
        .get("AGENT")
        .ok_or(AgentHeaderError::Missing)?
        .to_str()
        .map_err(|_| AgentHeaderError::Invalid("non-utf8".to_string()))?;
    Agent::parse(raw)
}

fn parse_session_id(headers: &HeaderMap) -> Result<Option<String>, &'static str> {
    match headers.get("Mcp-Session-Id") {
        Some(value) => value
            .to_str()
            .map(|s| Some(s.to_string()))
            .map_err(|_| "invalid Mcp-Session-Id header"),
        None => Ok(None),
    }
}

fn require_session_id(headers: &HeaderMap) -> Result<String, &'static str> {
    headers
        .get("Mcp-Session-Id")
        .ok_or("missing Mcp-Session-Id header")?
        .to_str()
        .map(|s| s.to_string())
        .map_err(|_| "invalid Mcp-Session-Id header")
}

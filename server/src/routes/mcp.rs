use axum::{
    Json,
    body::Bytes,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::mcp::{self, Agent, AgentHeaderError};

pub async fn handle_mcp_request(headers: HeaderMap, body: Bytes) -> impl IntoResponse {
    match resolve_agent(&headers) {
        Ok(agent) => {
            let response = mcp::handle_http_request(agent, &body).await;
            match response.body {
                Some(value) => (response.status, Json(value)).into_response(),
                None => response.status.into_response(),
            }
        }
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
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

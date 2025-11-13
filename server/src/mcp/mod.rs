use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use http::StatusCode;
use once_cell::sync::Lazy;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use thiserror::Error;

use crate::{
    db::{task as task_db, task_group as task_group_db},
    system::strategy::StrategyState,
};
use openapi::models::{ActiveStrategy, Strategy, Task, TaskGroup, TaskGroupStatus, TaskStatus};

pub(crate) mod session;

mod git_diff;
mod git_status;
mod project_commands;
mod project_command_list;
mod project_command_run;
mod tasks_create;
mod tasks_create_group;
mod tasks_delete;
mod tasks_dependencies_get;
mod tasks_dependencies_set;
mod tasks_get;
mod tasks_groups_delete;
mod tasks_groups_get;
mod tasks_groups_list;
mod tasks_groups_update;
mod tasks_list;
mod tasks_set_status;
mod tasks_update;

use project_commands::ProjectCommandRegistry;
use session::SessionManager;

const JSONRPC_VERSION: &str = "2.0";
const PROTOCOL_VERSION: &str = "2025-06-18";
const SERVER_NAME: &str = "robot-farm";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

static TOOL_REGISTRY: Lazy<Vec<Arc<dyn McpTool>>> = Lazy::new(|| {
    vec![
        Arc::new(git_status::GitStatusTool::default()),
        Arc::new(git_diff::GitDiffTool::default()),
        Arc::new(tasks_list::TasksListTool::default()),
        Arc::new(tasks_get::TasksGetTool::default()),
        Arc::new(tasks_create::TasksCreateTool::default()),
        Arc::new(tasks_update::TasksUpdateTool::default()),
        Arc::new(tasks_delete::TasksDeleteTool::default()),
        Arc::new(tasks_set_status::TasksSetStatusTool::default()),
        Arc::new(tasks_create_group::TaskGroupsCreateTool::default()),
        Arc::new(tasks_groups_list::TaskGroupsListTool::default()),
        Arc::new(tasks_groups_get::TaskGroupsGetTool::default()),
        Arc::new(tasks_groups_update::TaskGroupsUpdateTool::default()),
        Arc::new(tasks_groups_delete::TaskGroupsDeleteTool::default()),
        Arc::new(tasks_dependencies_get::TasksDependenciesGetTool::default()),
        Arc::new(tasks_dependencies_set::TasksDependenciesSetTool::default()),
        Arc::new(project_command_list::ProjectCommandListTool::default()),
        Arc::new(project_command_run::ProjectCommandRunTool::default()),
    ]
});

#[derive(Clone, Debug)]
pub enum Agent {
    Orchestrator,
    Worker,
    WorkerWithId(i64),
    Qa,
    Wizard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AgentRole {
    Orchestrator,
    Worker,
    Qa,
    Wizard,
}

impl Agent {
    pub fn parse(raw: &str) -> Result<Self, AgentHeaderError> {
        let value = raw.trim();
        if value.is_empty() {
            return Err(AgentHeaderError::Missing);
        }
        let lower = value.to_ascii_lowercase();
        match lower.as_str() {
            "orchestrator" => Ok(Agent::Orchestrator),
            "worker" => Ok(Agent::Worker),
            "qa" => Ok(Agent::Qa),
            "wizard" => Ok(Agent::Wizard),
            _ => {
                if let Some(id_str) = lower.strip_prefix("ws") {
                    let id: i64 = id_str
                        .parse()
                        .map_err(|_| AgentHeaderError::Invalid(value.to_string()))?;
                    Ok(Agent::WorkerWithId(id))
                } else {
                    Err(AgentHeaderError::Invalid(value.to_string()))
                }
            }
        }
    }

    pub fn role(&self) -> AgentRole {
        match self {
            Agent::Orchestrator => AgentRole::Orchestrator,
            Agent::Worker | Agent::WorkerWithId(_) => AgentRole::Worker,
            Agent::Qa => AgentRole::Qa,
            Agent::Wizard => AgentRole::Wizard,
        }
    }

    pub fn label(&self) -> String {
        match self {
            Agent::Orchestrator => "Orchestrator".to_string(),
            Agent::Worker => "Worker".to_string(),
            Agent::WorkerWithId(id) => format!("ws{id}"),
            Agent::Qa => "Quality Assurance".to_string(),
            Agent::Wizard => "Wizard".to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum AgentHeaderError {
    #[error("missing AGENT header")]
    Missing,
    #[error("invalid AGENT header: {0}")]
    Invalid(String),
}

#[derive(Clone, Debug)]
pub struct ToolContext {
    pub agent: Agent,
    pub strategy: ActiveStrategy,
}

impl ToolContext {
    pub fn role(&self) -> AgentRole {
        self.agent.role()
    }

    pub fn is_planning(&self) -> bool {
        matches!(self.strategy.id, Strategy::Planning)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TaskGroupSummary {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub description: String,
}

impl From<TaskGroup> for TaskGroupSummary {
    fn from(value: TaskGroup) -> Self {
        Self {
            id: value.id,
            slug: value.slug,
            title: value.title,
            description: value.description,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TaskWithGroupPayload {
    #[serde(flatten)]
    pub task: Task,
    pub group: TaskGroupSummary,
}

#[derive(Debug)]
pub enum DispatchOutcome {
    Json(Value),
    JsonWithSession { body: Value, session_id: String },
    NoContent,
}

enum ProcessResult {
    Json(Value),
    JsonWithSession { body: Value, session_id: String },
    NoContent,
}

pub struct DispatchResponse {
    pub status: StatusCode,
    pub body: Option<Value>,
    pub session_id: Option<String>,
}

#[derive(Debug, Error)]
pub enum DispatchError {
    #[error("response error")]
    Response { status: StatusCode, body: Value },
}

#[derive(Debug, Serialize)]
pub struct ToolContent {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ToolInvocationResponse {
    pub content: Vec<ToolContent>,
    #[serde(rename = "isError")]
    pub is_error: bool,
}

impl ToolInvocationResponse {
    pub fn text(message: impl Into<String>) -> Self {
        ToolInvocationResponse {
            content: vec![ToolContent {
                kind: "text".to_string(),
                text: Some(message.into()),
            }],
            is_error: false,
        }
    }

    pub fn text_error(message: impl Into<String>) -> Self {
        ToolInvocationResponse {
            content: vec![ToolContent {
                kind: "text".to_string(),
                text: Some(message.into()),
            }],
            is_error: true,
        }
    }
}

#[derive(Debug, Error)]
pub enum ToolInvocationError {
    #[error("invalid params: {0}")]
    InvalidParams(String),
    #[error("resource not found: {0}")]
    NotFound(String),
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("tool error: {0}")]
    Internal(String),
}

impl ToolInvocationError {
    pub fn code(&self) -> i32 {
        match self {
            ToolInvocationError::InvalidParams(_) => -32602,
            ToolInvocationError::Unauthorized(_) => -32001,
            ToolInvocationError::NotFound(_) => -32004,
            ToolInvocationError::Internal(_) => -32603,
        }
    }

    pub fn message(&self) -> String {
        self.to_string()
    }
}

#[async_trait]
pub trait McpTool: Send + Sync {
    fn name(&self) -> &'static str;
    fn title(&self) -> Option<&'static str> {
        None
    }
    fn description(&self) -> &'static str;
    fn input_schema(&self) -> Value;
    fn allowed_roles(&self) -> &'static [AgentRole];
    fn is_visible(&self, ctx: &ToolContext) -> bool {
        self.allowed_roles().contains(&ctx.role())
    }
    async fn call(
        &self,
        ctx: &ToolContext,
        args: Value,
    ) -> Result<ToolInvocationResponse, ToolInvocationError>;
}

const ROLES_ALL: &[AgentRole] = &[
    AgentRole::Orchestrator,
    AgentRole::Worker,
    AgentRole::Qa,
    AgentRole::Wizard,
];
const ROLES_COORDINATION: &[AgentRole] =
    &[AgentRole::Orchestrator, AgentRole::Qa, AgentRole::Wizard];

pub async fn handle_http_request(
    agent: Agent,
    session_id: Option<String>,
    body: &[u8],
) -> DispatchResponse {
    if body.is_empty() {
        return DispatchResponse {
            status: StatusCode::OK,
            body: Some(make_error_response(None, -32600, "empty body", None)),
            session_id: None,
        };
    }

    let payload: Value = match serde_json::from_slice(body) {
        Ok(value) => value,
        Err(err) => {
            return DispatchResponse {
                status: StatusCode::OK,
                body: Some(make_error_response(
                    None,
                    -32700,
                    format!("failed to parse body: {err}"),
                    None,
                )),
                session_id: None,
            };
        }
    };

    let strategy = StrategyState::global().snapshot();
    let ctx = ToolContext { agent, strategy };

    let outcome = match dispatch_value(&ctx, payload, session_id.as_deref()).await {
        Ok(outcome) => outcome,
        Err(DispatchError::Response { status, body }) => {
            return DispatchResponse {
                status,
                body: Some(body),
                session_id: None,
            };
        }
    };

    match outcome {
        DispatchOutcome::Json(value) => DispatchResponse {
            status: StatusCode::OK,
            body: Some(value),
            session_id: None,
        },
        DispatchOutcome::JsonWithSession { body, session_id } => DispatchResponse {
            status: StatusCode::OK,
            body: Some(body),
            session_id: Some(session_id),
        },
        DispatchOutcome::NoContent => DispatchResponse {
            status: StatusCode::NO_CONTENT,
            body: None,
            session_id: None,
        },
    }
}

async fn dispatch_value(
    ctx: &ToolContext,
    payload: Value,
    session_id: Option<&str>,
) -> Result<DispatchOutcome, DispatchError> {
    match payload {
        Value::Object(_) => match process_request(ctx, payload, session_id).await? {
            ProcessResult::Json(value) => Ok(DispatchOutcome::Json(value)),
            ProcessResult::JsonWithSession { body, session_id } => {
                Ok(DispatchOutcome::JsonWithSession { body, session_id })
            }
            ProcessResult::NoContent => Ok(DispatchOutcome::NoContent),
        },
        Value::Array(items) => {
            let mut responses = Vec::new();
            let mut new_session: Option<String> = None;
            for item in items {
                if !item.is_object() {
                    responses.push(make_error_response(
                        None,
                        -32600,
                        "batch entry must be an object",
                        None,
                    ));
                    continue;
                }
                match process_request(ctx, item, session_id).await? {
                    ProcessResult::Json(value) => responses.push(value),
                    ProcessResult::JsonWithSession { body, session_id } => {
                        if new_session.is_none() {
                            new_session = Some(session_id.clone());
                        }
                        responses.push(body);
                    }
                    ProcessResult::NoContent => {}
                }
            }

            if responses.is_empty() {
                Ok(DispatchOutcome::NoContent)
            } else if let Some(session_id) = new_session {
                Ok(DispatchOutcome::JsonWithSession {
                    body: Value::Array(responses),
                    session_id,
                })
            } else {
                Ok(DispatchOutcome::Json(Value::Array(responses)))
            }
        }
        _ => Err(dispatch_error(
            StatusCode::BAD_REQUEST,
            None,
            -32600,
            "expected JSON object or array",
        )),
    }
}

async fn process_request(
    ctx: &ToolContext,
    value: Value,
    session_id: Option<&str>,
) -> Result<ProcessResult, DispatchError> {
    let req: JsonRpcRequest = match serde_json::from_value(value) {
        Ok(req) => req,
        Err(err) => {
            return Ok(ProcessResult::Json(make_error_response(
                None,
                -32600,
                format!("invalid request: {err}"),
                None,
            )));
        }
    };

    if req.jsonrpc != JSONRPC_VERSION {
        return Ok(ProcessResult::Json(make_error_response(
            req.id,
            -32600,
            format!("unsupported jsonrpc version: {}", req.jsonrpc),
            None,
        )));
    }

    match req.method.as_str() {
        "initialize" => {
            let session_id_value = SessionManager::global().create_session().await;
            Ok(ProcessResult::JsonWithSession {
                body: handle_initialize(req.id),
                session_id: session_id_value,
            })
        }
        "notifications/initialized" => {
            require_session(session_id, req.id.clone()).await?;
            Ok(ProcessResult::NoContent)
        }
        "tools/list" => {
            require_session(session_id, req.id.clone()).await?;
            Ok(ProcessResult::Json(handle_tools_list(ctx, req.id)))
        }
        "tools/call" => {
            require_session(session_id, req.id.clone()).await?;
            Ok(ProcessResult::Json(
                handle_tools_call(ctx, req.id, req.params).await,
            ))
        }
        _ => Ok(ProcessResult::Json(make_error_response(
            req.id,
            -32601,
            format!("unknown method: {}", req.method),
            None,
        ))),
    }
}

fn handle_initialize(id: Option<Value>) -> Value {
    make_result_response(
        id,
        json!({
            "protocolVersion": PROTOCOL_VERSION,
            "serverInfo": {
                "name": SERVER_NAME,
                "version": SERVER_VERSION,
            },
            "capabilities": {
                "tools": { "listChanged": false },
            },
            "instructions": "Use Robot Farm MCP tools via /mcp. Provide the AGENT header (orchestrator, wizard, qa, worker, wsN).",
        }),
    )
}

fn handle_tools_list(ctx: &ToolContext, id: Option<Value>) -> Value {
    let tools: Vec<Value> = TOOL_REGISTRY
        .iter()
        .filter(|tool| tool.is_visible(ctx))
        .map(|tool| {
            let mut entry = json!({
                "name": tool.name(),
                "description": tool.description(),
                "inputSchema": tool.input_schema(),
            });
            if let Some(title) = tool.title() {
                entry["title"] = json!(title);
            }
            entry
        })
        .collect();

    make_result_response(id, json!({ "tools": tools }))
}

async fn handle_tools_call(ctx: &ToolContext, id: Option<Value>, params: Option<Value>) -> Value {
    let parsed: Result<ToolCallParams, _> =
        serde_json::from_value(params.unwrap_or_else(|| json!({})));
    let params = match parsed {
        Ok(p) => p,
        Err(err) => {
            return make_error_response(
                id,
                -32602,
                format!("invalid tools/call params: {err}"),
                None,
            );
        }
    };

    let tool = match TOOL_REGISTRY
        .iter()
        .find(|tool| tool.name() == params.name)
        .cloned()
    {
        Some(tool) => tool,
        None => {
            return make_error_response(id, -32601, format!("unknown tool: {}", params.name), None);
        }
    };

    if !tool.is_visible(ctx) {
        return make_error_response(
            id,
            -32001,
            format!("agent not allowed to call {}", tool.name()),
            None,
        );
    }

    match tool.call(ctx, params.arguments).await {
        Ok(result) => make_result_response(
            id,
            json!({
                "content": result.content,
                "isError": result.is_error,
            }),
        ),
        Err(err) => make_error_response(id, err.code(), err.message(), None),
    }
}

fn make_result_response(id: Option<Value>, result: Value) -> Value {
    json!({
        "jsonrpc": JSONRPC_VERSION,
        "id": id.unwrap_or(Value::Null),
        "result": result,
    })
}

fn make_error_response(
    id: Option<Value>,
    code: i32,
    message: impl Into<String>,
    data: Option<Value>,
) -> Value {
    let mut error = json!({
        "code": code,
        "message": message.into(),
    });
    if let Some(data) = data {
        error["data"] = data;
    }
    json!({
        "jsonrpc": JSONRPC_VERSION,
        "id": id.unwrap_or(Value::Null),
        "error": error,
    })
}

async fn require_session(
    session_id: Option<&str>,
    request_id: Option<Value>,
) -> Result<(), DispatchError> {
    let Some(id) = session_id else {
        return Err(dispatch_error(
            StatusCode::BAD_REQUEST,
            request_id,
            -32002,
            "missing Mcp-Session-Id header",
        ));
    };

    if SessionManager::global().has_session(id).await {
        Ok(())
    } else {
        Err(dispatch_error(
            StatusCode::NOT_FOUND,
            request_id,
            -32004,
            "unknown session id",
        ))
    }
}

fn dispatch_error(
    status: StatusCode,
    id: Option<Value>,
    code: i32,
    message: impl Into<String>,
) -> DispatchError {
    DispatchError::Response {
        status,
        body: make_error_response(id, code, message, None),
    }
}

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    #[serde(default)]
    pub jsonrpc: String,
    #[serde(default)]
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct ToolCallParams {
    pub name: String,
    #[serde(default = "empty_object")]
    pub arguments: Value,
}

fn empty_object() -> Value {
    Value::Object(Default::default())
}

pub fn parse_params<T: DeserializeOwned>(value: Value) -> Result<T, ToolInvocationError> {
    serde_json::from_value(value).map_err(|err| ToolInvocationError::InvalidParams(err.to_string()))
}

pub async fn require_task_by_slug(slug: &str) -> Result<Task, ToolInvocationError> {
    task_db::get_task_by_slug(slug)
        .await
        .map_err(|err| ToolInvocationError::Internal(err.to_string()))?
        .ok_or_else(|| ToolInvocationError::NotFound(format!("task {slug} not found")))
}

pub async fn require_task_by_id(task_id: i64) -> Result<Task, ToolInvocationError> {
    task_db::get_task(task_id)
        .await
        .map_err(|err| ToolInvocationError::Internal(err.to_string()))?
        .ok_or_else(|| ToolInvocationError::NotFound(format!("task id {task_id} not found")))
}

pub async fn require_group_by_slug(slug: &str) -> Result<TaskGroup, ToolInvocationError> {
    task_group_db::get_task_group_by_slug(slug)
        .await
        .map_err(|err| ToolInvocationError::Internal(err.to_string()))?
        .ok_or_else(|| ToolInvocationError::NotFound(format!("task group {slug} not found")))
}

pub async fn summarize_group(group_id: i64) -> Result<TaskGroupSummary, ToolInvocationError> {
    let group = task_group_db::get_task_group(group_id)
        .await
        .map_err(|err| ToolInvocationError::Internal(err.to_string()))?
        .ok_or_else(|| ToolInvocationError::NotFound(format!("task group {group_id} not found")))?;
    Ok(group.into())
}

pub async fn summarize_task(task: Task) -> Result<TaskWithGroupPayload, ToolInvocationError> {
    let summary = summarize_group(task.group_id).await?;
    Ok(TaskWithGroupPayload {
        task,
        group: summary,
    })
}

pub async fn load_group_map() -> Result<HashMap<i64, TaskGroupSummary>, ToolInvocationError> {
    let groups = task_group_db::list_task_groups()
        .await
        .map_err(|err| ToolInvocationError::Internal(err.to_string()))?;
    let mut map = HashMap::new();
    for group in groups {
        map.insert(group.id, group.into());
    }
    Ok(map)
}

pub fn serialize_json(value: &impl Serialize) -> Result<String, ToolInvocationError> {
    serde_json::to_string_pretty(value)
        .map_err(|err| ToolInvocationError::Internal(format!("serialization error: {err}")))
}

pub fn parse_task_status(raw: &str) -> Result<TaskStatus, ToolInvocationError> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "ready" => Ok(TaskStatus::Ready),
        "blocked" => Ok(TaskStatus::Blocked),
        "done" => Ok(TaskStatus::Done),
        other => Err(ToolInvocationError::InvalidParams(format!(
            "unknown task status: {other}"
        ))),
    }
}

pub fn parse_task_group_status(raw: &str) -> Result<TaskGroupStatus, ToolInvocationError> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "ready" => Ok(TaskGroupStatus::Ready),
        "done" => Ok(TaskGroupStatus::Done),
        other => Err(ToolInvocationError::InvalidParams(format!(
            "unknown task group status: {other}"
        ))),
    }
}

pub const fn roles_all() -> &'static [AgentRole] {
    ROLES_ALL
}

pub const fn roles_coordination() -> &'static [AgentRole] {
    ROLES_COORDINATION
}

pub fn schema_for_type<T: JsonSchema>() -> Value {
    serde_json::to_value(schemars::schema_for!(T)).unwrap_or_else(|_| json!({"type": "object"}))
}

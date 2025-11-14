use async_trait::async_trait;
use openapi::models::{TaskCreateInput, TaskStatus};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::db::task as task_db;

use super::{
    AgentRole, McpTool, ToolContext, ToolInvocationError, ToolInvocationResponse, parse_params,
    parse_task_status, require_group_by_slug, roles_coordination, schema_for_type, serialize_json,
    summarize_task,
};

#[derive(Default)]
pub struct TasksCreateTool;

#[async_trait]
impl McpTool for TasksCreateTool {
    fn name(&self) -> &'static str {
        "tasks_create"
    }

    fn title(&self) -> Option<&'static str> {
        Some("Create Task")
    }

    fn description(&self) -> &'static str {
        "Create a new task inside a task group (orchestrator restricted to planning)."
    }

    fn input_schema(&self) -> Value {
        schema_for_type::<TasksCreateInputPayload>()
    }

    fn allowed_roles(&self) -> &'static [AgentRole] {
        roles_coordination()
    }

    fn is_visible(&self, ctx: &ToolContext) -> bool {
        match ctx.role() {
            AgentRole::Wizard => true,
            AgentRole::Orchestrator => ctx.is_planning(),
            AgentRole::Qa => false,
            AgentRole::Worker => false,
        }
    }

    async fn call(
        &self,
        ctx: &ToolContext,
        args: Value,
    ) -> Result<ToolInvocationResponse, ToolInvocationError> {
        let input: TasksCreateInputPayload = parse_params(args)?;
        let group = require_group_by_slug(&input.group_slug).await?;
        let owner = input.owner.unwrap_or_else(|| ctx.agent.label());
        let status = match input.status.as_deref() {
            Some(value) => parse_task_status(value)?,
            None => TaskStatus::Ready,
        };
        let mut payload = TaskCreateInput::new(
            group.id,
            input.slug,
            input.title,
            status,
            owner,
            input.description,
        );
        payload.commit_hash = input.commit_hash;
        let created = task_db::create_task(payload)
            .await
            .map_err(|err| ToolInvocationError::Internal(err.to_string()))?;
        let summary = summarize_task(created).await?;
        let text = serialize_json(&json!({ "task": summary }))?;
        Ok(ToolInvocationResponse::text(text))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
struct TasksCreateInputPayload {
    pub group_slug: String,
    pub slug: String,
    pub title: String,
    pub commit_hash: Option<String>,
    pub status: Option<String>,
    pub owner: Option<String>,
    pub description: String,
}

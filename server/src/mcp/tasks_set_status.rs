use async_trait::async_trait;
use openapi::models::TaskUpdateInput;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::db::task as task_db;

use super::{
    AgentRole, McpTool, ToolContext, ToolInvocationError, ToolInvocationResponse,
    ensure_task_mutation_allowed, parse_params, parse_task_status, require_task_by_slug,
    roles_coordination, schema_for_type, serialize_json, summarize_task,
};

#[derive(Default)]
pub struct TasksSetStatusTool;

#[async_trait]
impl McpTool for TasksSetStatusTool {
    fn name(&self) -> &'static str {
        "tasks_set_status"
    }

    fn description(&self) -> &'static str {
        "Update the status (and optional owner) of a task."
    }

    fn input_schema(&self) -> Value {
        schema_for_type::<TasksSetStatusInput>()
    }

    fn allowed_roles(&self) -> &'static [AgentRole] {
        roles_coordination()
    }

    async fn call(
        &self,
        ctx: &ToolContext,
        args: Value,
    ) -> Result<ToolInvocationResponse, ToolInvocationError> {
        ensure_task_mutation_allowed(ctx)?;
        let input: TasksSetStatusInput = parse_params(args)?;
        let task = require_task_by_slug(&input.slug).await?;
        let mut update = TaskUpdateInput::new();
        update.status = Some(parse_task_status(&input.status)?);
        if let Some(owner) = input.owner {
            update.owner = Some(owner);
        }
        let updated = task_db::update_task(task.id, update)
            .await
            .map_err(|err| ToolInvocationError::Internal(err.to_string()))?
            .ok_or_else(|| ToolInvocationError::NotFound(task.slug))?;
        let summary = summarize_task(updated).await?;
        let text = serialize_json(&json!({ "task": summary }))?;
        Ok(ToolInvocationResponse::text(text))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Set the status (and optional owner) for a task.")]
struct TasksSetStatusInput {
    /// Slug of the task to update.
    pub slug: String,
    /// New status value (READY, IN_PROGRESS, BLOCKED, COMPLETED, etc.).
    pub status: String,
    /// Optional new owner label.
    pub owner: Option<String>,
}

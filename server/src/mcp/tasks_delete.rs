use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::db::task as task_db;

use super::{
    AgentRole, McpTool, ToolContext, ToolInvocationError, ToolInvocationResponse,
    ensure_task_mutation_allowed, parse_params, require_task_by_slug, roles_coordination,
    schema_for_type, serialize_json,
};

#[derive(Default)]
pub struct TasksDeleteTool;

#[async_trait]
impl McpTool for TasksDeleteTool {
    fn name(&self) -> &'static str {
        "tasks_delete"
    }

    fn description(&self) -> &'static str {
        "Delete a task by slug."
    }

    fn input_schema(&self) -> Value {
        schema_for_type::<TasksDeleteInput>()
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
        let input: TasksDeleteInput = parse_params(args)?;
        let task = require_task_by_slug(&input.slug).await?;
        let deleted = task_db::delete_task(task.id)
            .await
            .map_err(|err| ToolInvocationError::Internal(err.to_string()))?;
        if !deleted {
            return Err(ToolInvocationError::NotFound(input.slug));
        }
        let text = serialize_json(&json!({
            "deleted": true,
            "slug": task.slug,
        }))?;
        Ok(ToolInvocationResponse::text(text))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Parameters for deleting a task.")]
struct TasksDeleteInput {
    /// Slug of the task to delete.
    pub slug: String,
}

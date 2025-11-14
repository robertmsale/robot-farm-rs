use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::db::task_group as task_group_db;

use super::{
    AgentRole, McpTool, ToolContext, ToolInvocationError, ToolInvocationResponse,
    ensure_task_mutation_allowed, parse_params, require_group_by_slug, roles_coordination,
    schema_for_type, serialize_json,
};

#[derive(Default)]
pub struct TaskGroupsDeleteTool;

#[async_trait]
impl McpTool for TaskGroupsDeleteTool {
    fn name(&self) -> &'static str {
        "task_groups_delete"
    }

    fn description(&self) -> &'static str {
        "Delete a task group by slug."
    }

    fn input_schema(&self) -> Value {
        schema_for_type::<TaskGroupsDeleteInput>()
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
        let input: TaskGroupsDeleteInput = parse_params(args)?;
        let group = require_group_by_slug(&input.slug).await?;
        let deleted = task_group_db::delete_task_group(group.id)
            .await
            .map_err(|err| ToolInvocationError::Internal(err.to_string()))?;
        if !deleted {
            return Err(ToolInvocationError::NotFound(input.slug));
        }
        let text = serialize_json(&json!({
            "deleted": true,
            "slug": group.slug,
        }))?;
        Ok(ToolInvocationResponse::text(text))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
struct TaskGroupsDeleteInput {
    pub slug: String,
}

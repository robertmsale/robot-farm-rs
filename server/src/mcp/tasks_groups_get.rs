use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{
    AgentRole, McpTool, ToolContext, ToolInvocationError, ToolInvocationResponse, parse_params,
    require_group_by_slug, roles_all, schema_for_type, serialize_json,
};

#[derive(Default)]
pub struct TaskGroupsGetTool;

#[async_trait]
impl McpTool for TaskGroupsGetTool {
    fn name(&self) -> &'static str {
        "task_groups_get"
    }

    fn description(&self) -> &'static str {
        "Fetch a task group by slug."
    }

    fn input_schema(&self) -> Value {
        schema_for_type::<TaskGroupsGetInput>()
    }

    fn allowed_roles(&self) -> &'static [AgentRole] {
        roles_all()
    }

    async fn call(
        &self,
        _ctx: &ToolContext,
        args: Value,
    ) -> Result<ToolInvocationResponse, ToolInvocationError> {
        let input: TaskGroupsGetInput = parse_params(args)?;
        let group = require_group_by_slug(&input.slug).await?;
        let text = serialize_json(&json!({ "group": group }))?;
        Ok(ToolInvocationResponse::text(text))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
struct TaskGroupsGetInput {
    pub slug: String,
}

use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{
    AgentRole, McpTool, ToolContext, ToolInvocationError, ToolInvocationResponse, parse_params,
    require_task_by_slug, roles_all, schema_for_type, serialize_json, summarize_task,
};

#[derive(Default)]
pub struct TasksGetTool;

#[async_trait]
impl McpTool for TasksGetTool {
    fn name(&self) -> &'static str {
        "tasks_get"
    }

    fn title(&self) -> Option<&'static str> {
        Some("Get Task")
    }

    fn description(&self) -> &'static str {
        "Fetch a single task by slug and include its task group metadata."
    }

    fn input_schema(&self) -> Value {
        schema_for_type::<TasksGetInput>()
    }

    fn allowed_roles(&self) -> &'static [AgentRole] {
        roles_all()
    }

    async fn call(
        &self,
        _ctx: &ToolContext,
        args: Value,
    ) -> Result<ToolInvocationResponse, ToolInvocationError> {
        let input: TasksGetInput = parse_params(args)?;
        let task = require_task_by_slug(&input.slug).await?;
        let payload = summarize_task(task).await?;
        let text = serialize_json(&json!({ "task": payload }))?;
        Ok(ToolInvocationResponse::text(text))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
struct TasksGetInput {
    pub slug: String,
}

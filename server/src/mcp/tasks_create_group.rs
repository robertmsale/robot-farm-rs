use async_trait::async_trait;
use openapi::models::TaskGroupCreateInput;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::db::task_group as task_group_db;

use super::{
    AgentRole, McpTool, TaskGroupSummary, ToolContext, ToolInvocationError, ToolInvocationResponse,
    ensure_task_mutation_allowed, parse_params, schema_for_type, serialize_json,
};

#[derive(Default)]
pub struct TaskGroupsCreateTool;

#[async_trait]
impl McpTool for TaskGroupsCreateTool {
    fn name(&self) -> &'static str {
        "task_groups_create"
    }

    fn title(&self) -> Option<&'static str> {
        Some("Create Task Group")
    }

    fn description(&self) -> &'static str {
        "Create a new task group (orchestrator allowed only during planning)."
    }

    fn input_schema(&self) -> Value {
        schema_for_type::<TaskGroupsCreateInput>()
    }

    fn allowed_roles(&self) -> &'static [AgentRole] {
        super::roles_coordination()
    }

    fn is_visible(&self, ctx: &ToolContext) -> bool {
        match ctx.role() {
            AgentRole::Wizard | AgentRole::Qa => true,
            AgentRole::Orchestrator => ctx.is_planning(),
            AgentRole::Worker => false,
        }
    }

    async fn call(
        &self,
        ctx: &ToolContext,
        args: Value,
    ) -> Result<ToolInvocationResponse, ToolInvocationError> {
        ensure_task_mutation_allowed(ctx)?;
        let input: TaskGroupsCreateInput = parse_params(args)?;
        let payload = TaskGroupCreateInput::new(input.slug, input.title, input.description);
        let created = task_group_db::create_task_group(payload)
            .await
            .map_err(|err| ToolInvocationError::Internal(err.to_string()))?;
        let summary: TaskGroupSummary = created.into();
        let text = serialize_json(&json!({ "group": summary }))?;
        Ok(ToolInvocationResponse::text(text))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
struct TaskGroupsCreateInput {
    pub slug: String,
    pub title: String,
    pub description: String,
}

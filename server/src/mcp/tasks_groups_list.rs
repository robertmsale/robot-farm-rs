use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::db::task_group as task_group_db;

use super::{
    AgentRole, McpTool, TaskGroupSummary, ToolContext, ToolInvocationError, ToolInvocationResponse,
    parse_params, parse_task_group_status, roles_all, schema_for_type, serialize_json,
};

#[derive(Default)]
pub struct TaskGroupsListTool;

#[async_trait]
impl McpTool for TaskGroupsListTool {
    fn name(&self) -> &'static str {
        "task_groups_list"
    }

    fn description(&self) -> &'static str {
        "List task groups with optional status filtering."
    }

    fn input_schema(&self) -> Value {
        schema_for_type::<TaskGroupsListInput>()
    }

    fn allowed_roles(&self) -> &'static [AgentRole] {
        roles_all()
    }

    async fn call(
        &self,
        _ctx: &ToolContext,
        args: Value,
    ) -> Result<ToolInvocationResponse, ToolInvocationError> {
        let input: TaskGroupsListInput = parse_params(args)?;
        let groups = task_group_db::list_task_groups()
            .await
            .map_err(|err| ToolInvocationError::Internal(err.to_string()))?;
        let status_filter = input
            .status
            .as_deref()
            .map(parse_task_group_status)
            .transpose()?;
        let mut payload = Vec::new();
        for group in groups {
            if let Some(filter_status) = status_filter {
                if group.status != filter_status {
                    continue;
                }
            }
            payload.push(TaskGroupSummary::from(group));
        }
        let text = serialize_json(&json!({ "groups": payload }))?;
        Ok(ToolInvocationResponse::text(text))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Optional filters for listing task groups.")]
struct TaskGroupsListInput {
    /// Filter by task group status.
    pub status: Option<String>,
}

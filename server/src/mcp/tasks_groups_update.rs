use async_trait::async_trait;
use openapi::models::TaskGroupUpdateInput;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::db::task_group as task_group_db;

use super::{
    AgentRole, McpTool, TaskGroupSummary, ToolContext, ToolInvocationError, ToolInvocationResponse,
    ensure_task_mutation_allowed, parse_params, require_group_by_slug, roles_coordination,
    schema_for_type, serialize_json,
};

#[derive(Default)]
pub struct TaskGroupsUpdateTool;

#[async_trait]
impl McpTool for TaskGroupsUpdateTool {
    fn name(&self) -> &'static str {
        "task_groups_update"
    }

    fn description(&self) -> &'static str {
        "Update the metadata of a task group."
    }

    fn input_schema(&self) -> Value {
        schema_for_type::<TaskGroupsUpdateInput>()
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
        let input: TaskGroupsUpdateInput = parse_params(args)?;
        let group = require_group_by_slug(&input.slug).await?;
        let mut payload = TaskGroupUpdateInput::new();
        let mut touched = false;

        if let Some(slug) = input.new_slug {
            payload.slug = Some(slug);
            touched = true;
        }
        if let Some(title) = input.title {
            payload.title = Some(title);
            touched = true;
        }
        if let Some(description) = input.description {
            payload.description = Some(description);
            touched = true;
        }

        if !touched {
            return Err(ToolInvocationError::InvalidParams(
                "no fields provided for update".to_string(),
            ));
        }

        let updated = task_group_db::update_task_group(group.id, payload)
            .await
            .map_err(|err| ToolInvocationError::Internal(err.to_string()))?
            .ok_or_else(|| ToolInvocationError::NotFound(group.slug))?;
        let summary: TaskGroupSummary = updated.into();
        let text = serialize_json(&json!({ "group": summary }))?;
        Ok(ToolInvocationResponse::text(text))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Fields that may be updated on a task group.")]
struct TaskGroupsUpdateInput {
    /// Current task group slug.
    pub slug: String,
    /// Optional new slug.
    pub new_slug: Option<String>,
    /// Optional new title.
    pub title: Option<String>,
    /// Optional new description.
    pub description: Option<String>,
}

use async_trait::async_trait;
use openapi::models::TaskUpdateInput;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::db::task as task_db;

use super::{
    AgentRole, McpTool, ToolContext, ToolInvocationError, ToolInvocationResponse, parse_params,
    parse_task_status, require_group_by_slug, require_task_by_slug, roles_coordination,
    schema_for_type, serialize_json, summarize_task,
};

#[derive(Default)]
pub struct TasksUpdateTool;

#[async_trait]
impl McpTool for TasksUpdateTool {
    fn name(&self) -> &'static str {
        "tasks_update"
    }

    fn title(&self) -> Option<&'static str> {
        Some("Update Task")
    }

    fn description(&self) -> &'static str {
        "Update metadata for an existing task."
    }

    fn input_schema(&self) -> Value {
        schema_for_type::<TasksUpdateInputPayload>()
    }

    fn allowed_roles(&self) -> &'static [AgentRole] {
        roles_coordination()
    }

    async fn call(
        &self,
        _ctx: &ToolContext,
        args: Value,
    ) -> Result<ToolInvocationResponse, ToolInvocationError> {
        let input: TasksUpdateInputPayload = parse_params(args)?;
        let existing = require_task_by_slug(&input.slug).await?;
        let mut payload = TaskUpdateInput::new();
        let mut touched = false;

        if let Some(group_slug) = input.group_slug {
            let group = require_group_by_slug(&group_slug).await?;
            payload.group_id = Some(group.id);
            touched = true;
        }
        if let Some(new_slug) = input.new_slug {
            payload.slug = Some(new_slug);
            touched = true;
        }
        if let Some(title) = input.title {
            payload.title = Some(title);
            touched = true;
        }
        if let Some(commit_hash) = input.commit_hash {
            payload.commit_hash = Some(commit_hash);
            touched = true;
        }
        if let Some(status) = input.status.as_deref() {
            payload.status = Some(parse_task_status(status)?);
            touched = true;
        }
        if let Some(owner) = input.owner {
            payload.owner = Some(owner);
            touched = true;
        }

        if !touched {
            return Err(ToolInvocationError::InvalidParams(
                "no fields provided for update".to_string(),
            ));
        }

        let updated = task_db::update_task(existing.id, payload)
            .await
            .map_err(|err| ToolInvocationError::Internal(err.to_string()))?
            .ok_or_else(|| ToolInvocationError::NotFound(existing.slug.clone()))?;
        let summary = summarize_task(updated).await?;
        let text = serialize_json(&json!({ "task": summary }))?;
        Ok(ToolInvocationResponse::text(text))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
struct TasksUpdateInputPayload {
    pub slug: String,
    pub new_slug: Option<String>,
    pub group_slug: Option<String>,
    pub title: Option<String>,
    pub commit_hash: Option<String>,
    pub status: Option<String>,
    pub owner: Option<String>,
}

use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::db::task as task_db;

use super::{
    AgentRole, McpTool, TaskWithGroupPayload, ToolContext, ToolInvocationError,
    ToolInvocationResponse, load_group_map, parse_params, parse_task_status, roles_all,
    schema_for_type, serialize_json, task_visible_for,
};

#[derive(Default)]
pub struct TasksListTool;

#[async_trait]
impl McpTool for TasksListTool {
    fn name(&self) -> &'static str {
        "tasks_list"
    }

    fn title(&self) -> Option<&'static str> {
        Some("List Tasks")
    }

    fn description(&self) -> &'static str {
        "List tasks with optional filters (status, slug, group slug, owner)."
    }

    fn input_schema(&self) -> Value {
        schema_for_type::<TasksListInput>()
    }

    fn allowed_roles(&self) -> &'static [AgentRole] {
        roles_all()
    }

    async fn call(
        &self,
        ctx: &ToolContext,
        args: Value,
    ) -> Result<ToolInvocationResponse, ToolInvocationError> {
        let input: TasksListInput = parse_params(args)?;
        let tasks = task_db::list_tasks()
            .await
            .map_err(|err| ToolInvocationError::Internal(err.to_string()))?;
        let groups = load_group_map().await?;
        let status_filter = input.status.as_deref().map(parse_task_status).transpose()?;
        let owner_filter = input.owner.as_deref().map(str::to_ascii_lowercase);
        let title_filter = input.title.as_deref().map(|s| s.to_ascii_lowercase());
        let slug_filter = input.slug.as_deref();
        let group_slug_filter = input.group_slug.as_deref();

        let mut payloads = Vec::new();
        for task in tasks {
            if !task_visible_for(&ctx.agent, &task) {
                continue;
            }
            if let Some(filter_slug) = slug_filter {
                if task.slug != *filter_slug {
                    continue;
                }
            }
            if let Some(filter_status) = status_filter {
                if task.status != filter_status {
                    continue;
                }
            }
            if let Some(filter_title) = title_filter.as_deref() {
                if !task.title.to_ascii_lowercase().contains(filter_title) {
                    continue;
                }
            }
            if let Some(filter_owner) = owner_filter.as_deref() {
                if task.owner.to_ascii_lowercase() != *filter_owner {
                    continue;
                }
            }
            let summary = match groups.get(&task.group_id) {
                Some(group) => group.clone(),
                None => continue,
            };
            if let Some(filter_group_slug) = group_slug_filter {
                if summary.slug != *filter_group_slug {
                    continue;
                }
            }
            payloads.push(TaskWithGroupPayload {
                task,
                group: summary,
            });
        }
        let payload = json!({ "tasks": payloads });
        let text = serialize_json(&payload)?;
        Ok(ToolInvocationResponse::text(text))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Filter parameters for listing tasks.")]
struct TasksListInput {
    /// Optional task group slug to filter by.
    pub group_slug: Option<String>,
    /// Exact task slug to include (exclusive filter).
    pub slug: Option<String>,
    /// Case-insensitive substring match on task title.
    pub title: Option<String>,
    /// Exact owner label to match (e.g., 'orchestrator', 'ws3').
    pub owner: Option<String>,
    /// Task status name (READY, IN_PROGRESS, BLOCKED, COMPLETED, etc.).
    pub status: Option<String>,
}

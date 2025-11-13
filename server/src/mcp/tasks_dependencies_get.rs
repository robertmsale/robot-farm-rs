use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::db::task_dependency as task_dependency_db;

use super::{
    AgentRole, McpTool, ToolContext, ToolInvocationError, ToolInvocationResponse, parse_params,
    require_task_by_id, require_task_by_slug, roles_all, schema_for_type, serialize_json,
};

#[derive(Default)]
pub struct TasksDependenciesGetTool;

#[async_trait]
impl McpTool for TasksDependenciesGetTool {
    fn name(&self) -> &'static str {
        "tasks_dependencies_get"
    }

    fn description(&self) -> &'static str {
        "List the dependencies for a given task (by slug)."
    }

    fn input_schema(&self) -> Value {
        schema_for_type::<TasksDependenciesInput>()
    }

    fn allowed_roles(&self) -> &'static [AgentRole] {
        roles_all()
    }

    async fn call(
        &self,
        _ctx: &ToolContext,
        args: Value,
    ) -> Result<ToolInvocationResponse, ToolInvocationError> {
        let input: TasksDependenciesInput = parse_params(args)?;
        let task = require_task_by_slug(&input.slug).await?;
        let deps = task_dependency_db::list_task_dependencies(task.id)
            .await
            .map_err(|err| ToolInvocationError::Internal(err.to_string()))?;
        let mut entries = Vec::new();
        for dep_id in deps {
            let dep_task = require_task_by_id(dep_id).await?;
            entries.push(json!({
                "slug": dep_task.slug,
                "title": dep_task.title,
            }));
        }
        let text = serialize_json(&json!({
            "task": task.slug,
            "depends_on": entries,
        }))?;
        Ok(ToolInvocationResponse::text(text))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
struct TasksDependenciesInput {
    pub slug: String,
}

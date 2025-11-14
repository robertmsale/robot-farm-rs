use std::collections::BTreeSet;

use async_trait::async_trait;
use openapi::models::TaskDependencyCreateInput;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::db::task_dependency as task_dependency_db;

use super::{
    AgentRole, McpTool, ToolContext, ToolInvocationError, ToolInvocationResponse,
    ensure_task_mutation_allowed, parse_params, require_task_by_id, require_task_by_slug,
    roles_coordination, schema_for_type, serialize_json,
};

#[derive(Default)]
pub struct TasksDependenciesSetTool;

#[async_trait]
impl McpTool for TasksDependenciesSetTool {
    fn name(&self) -> &'static str {
        "tasks_dependencies_set"
    }

    fn description(&self) -> &'static str {
        "Replace the dependency list for a task, using dependency slugs."
    }

    fn input_schema(&self) -> Value {
        schema_for_type::<TasksDependenciesSetInput>()
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
        let input: TasksDependenciesSetInput = parse_params(args)?;
        let task = require_task_by_slug(&input.slug).await?;
        let mut desired = BTreeSet::new();
        for slug in input.depends_on {
            let dep_task = require_task_by_slug(&slug).await?;
            if dep_task.id == task.id {
                return Err(ToolInvocationError::InvalidParams(
                    "task cannot depend on itself".to_string(),
                ));
            }
            desired.insert(dep_task.id);
        }

        let existing: BTreeSet<i64> = task_dependency_db::list_task_dependencies(task.id)
            .await
            .map_err(|err| ToolInvocationError::Internal(err.to_string()))?
            .into_iter()
            .collect();

        for dep_id in existing.difference(&desired) {
            task_dependency_db::delete_task_dependency(task.id, *dep_id)
                .await
                .map_err(|err| ToolInvocationError::Internal(err.to_string()))?;
        }

        for dep_id in desired.difference(&existing) {
            let payload = TaskDependencyCreateInput::new(task.id, *dep_id);
            task_dependency_db::create_task_dependency(payload)
                .await
                .map_err(|err| ToolInvocationError::Internal(err.to_string()))?;
        }

        let refreshed: Vec<i64> = task_dependency_db::list_task_dependencies(task.id)
            .await
            .map_err(|err| ToolInvocationError::Internal(err.to_string()))?;
        let mut entries = Vec::new();
        for dep_id in refreshed {
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
struct TasksDependenciesSetInput {
    pub slug: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
}

use schemars::{JsonSchema, Schema, schema_for};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;

/// Structured payload produced by the orchestrator turn loop.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(
    title = "RobotFarmTurn",
    description = "Structured agent turn payload."
)]
pub struct OrchestratorTurn {
    /// Orchestrator must set their worker_id (e.g., 'ws1') when messaging a worker; workers should provide null.
    #[schemars(required)]
    pub target: Option<String>,
    /// Intent value describing the turn outcome.
    pub intent: OrchestratorIntent,
    /// One-line summary of the turn.
    pub summary: String,
    /// Expanded details supporting the summary.
    #[schemars(required)]
    pub details: Option<String>,
    /// Assignment payload when intent=ASSIGN_TASK.
    #[schemars(required)]
    pub assignments: Option<Assignment>,
}

/// Structured payload produced by a worker turn.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(
    title = "RobotFarmTurn",
    description = "Structured agent turn payload."
)]
pub struct WorkerTurn {
    /// Intent value describing the turn outcome.
    pub intent: WorkerIntent,
    /// One-line summary of the turn.
    pub summary: String,
    /// Expanded details supporting the summary.
    #[schemars(required)]
    pub details: Option<String>,
    /// Completed tasks payload when intent=COMPLETE_TASK.
    #[schemars(required)]
    pub completed: Option<WorkerCompletion>,
    /// Blocked tasks payload when intent=BLOCKED.
    #[schemars(required)]
    pub blocked: Option<BlockedEntry>,
}

/// Intent values for the orchestrator agent.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrchestratorIntent {
    AssignTask,
    StatusUpdate,
    AckPause,
}

/// Intent values for worker agents.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkerIntent {
    StatusUpdate,
    CompleteTask,
    Blocked,
    AckPause,
}

/// Assignment payload detailing tasks for the worker.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Assignment {
    /// Slug of the assigned story/task.
    pub task_slug: String,
    /// Short title describing the assignment.
    #[serde(rename = "task_title")]
    pub task_title: String,
    /// Concrete steps the worker should follow.
    pub steps: Vec<String>,
    /// Acceptance criteria for the assignment.
    #[schemars(required)]
    pub acceptance: Option<String>,
}

/// Completion entry for a worker turn (no answer payload).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkerCompletion {
    /// Slug of the task that was completed.
    pub task_slug: String,
    /// Completion notes or highlights.
    #[schemars(required)]
    pub notes: Option<String>,
    /// Short summary describing the changes for auto-commit messages. Aim for <= 72 characters.
    #[schemars(required, length(min = 3, max = 120))]
    pub commit_summary: String,
}

/// Blocked entry describing impediments.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BlockedEntry {
    /// Slug of the blocked task.
    pub task_slug: String,
    /// Explanation of why work cannot proceed.
    pub reason: String,
    /// Suggestions for unblocking the task.
    pub proposed_unblock_steps: Vec<String>,
}

/// Generate a JSON Schema for `T` with OpenAI-specific cleanup applied.
pub fn generated_schema_for<T: JsonSchema + ?Sized>() -> Schema {
    let mut schema = schema_for!(T);
    strip_keywords_from_refs(&mut schema);
    schema
}

fn strip_keywords_from_refs(schema: &mut Schema) {
    let mut value = serde_json::to_value(&*schema).expect("schema serialization succeeds");
    scrub_ref_nodes(&mut value);
    *schema = Schema::try_from(value).expect("scrubbed schema remains valid");
}

fn scrub_ref_nodes(value: &mut Value) {
    match value {
        Value::Object(map) => {
            if let Some(reference) = map.get("$ref").cloned() {
                map.clear();
                map.insert("$ref".into(), reference);
                return;
            }
            for child in map.values_mut() {
                scrub_ref_nodes(child);
            }
        }
        Value::Array(items) => {
            for child in items {
                scrub_ref_nodes(child);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oai_strict_schema::{StrictProfile, harden_root_schema, validate_schema};

    #[test]
    fn orchestrator_schema_is_strict_valid() {
        assert_schema_is_strict_valid::<OrchestratorTurn>();
    }

    #[test]
    fn worker_schema_is_strict_valid() {
        assert_schema_is_strict_valid::<WorkerTurn>();
    }

    fn assert_schema_is_strict_valid<T: JsonSchema>() {
        let mut schema = generated_schema_for::<T>();
        harden_root_schema(&mut schema);
        let rules = StrictProfile::OpenAI2025.default_rules();
        let report = validate_schema(&schema, &rules);
        assert!(
            report.is_ok(),
            "Strict validation failed with findings: {}",
            report
        );
    }
}

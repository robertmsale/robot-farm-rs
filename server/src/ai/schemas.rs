use schema_strict::{strict_schema_for_type, strip_sibling_keywords_from_ref};
use schemars::{JsonSchema, Schema};
use serde::{Deserialize, Serialize};

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
    /// Optional hint asking the system to re-enqueue the orchestrator to assign the next worker (e.g., \"ws2\").
    #[schemars(required)]
    pub next_worker_assignment: Option<String>,
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
pub fn generated_schema_for<T: JsonSchema>() -> Schema {
    let (mut schema, report) = strict_schema_for_type::<T>();
    debug_assert!(
        report.is_valid(),
        "strict schema for {} reported issues: {:?}",
        std::any::type_name::<T>(),
        report
    );
    strip_sibling_keywords_from_ref(&mut schema);
    schema
}

#[cfg(test)]
mod tests {
    use super::*;
    use schema_strict::strict_schema_for_type;

    #[test]
    fn orchestrator_schema_is_strict_valid() {
        assert_schema_is_strict_valid::<OrchestratorTurn>();
    }

    #[test]
    fn worker_schema_is_strict_valid() {
        assert_schema_is_strict_valid::<WorkerTurn>();
    }

    fn assert_schema_is_strict_valid<T: JsonSchema>() {
        let (_schema, report) = strict_schema_for_type::<T>();
        assert!(report.is_valid(), "Strict validation failed: {:?}", report);
    }
}

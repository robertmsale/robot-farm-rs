use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Strategy {
    Aggressive,
    Moderate,
    Economical,
    BugSmash,
    HotfixSwarm,
    Planning,
    WindDown,
}

impl Strategy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Strategy::Aggressive => "AGGRESSIVE",
            Strategy::Moderate => "MODERATE",
            Strategy::Economical => "ECONOMICAL",
            Strategy::BugSmash => "BUG_SMASH",
            Strategy::HotfixSwarm => "HOTFIX_SWARM",
            Strategy::Planning => "PLANNING",
            Strategy::WindDown => "WIND_DOWN",
        }
    }
    pub fn render_as_hint(&self) -> &'static str {
        match self {
            Strategy::Aggressive => "Assign tasks from any task group besides `chores` or `bugs` to any available worker. Avoid task dependency clashes. Once all the task groups are done, *then* source tasks from `chores` or `bugs` task groups.",
            Strategy::Moderate => "Assign tasks from the focused task group. Avoid task dependency clashes.",
            Strategy::Economical => "Assign tasks from the focused task group.",
            Strategy::BugSmash => "Assign tasks from the `bugs` task group.",
            Strategy::HotfixSwarm => "CODE RED: We have an emergency production issue. You will see these instructions 4 times, one for each worker you will activate. See the tasks in the `hotfix` task group for specifics on the reported issue. You will need to delegate at least 4 workers. One handles frontend, one handles backend, one handles database, and the last one handles package dependencies. Inform each one of their assigned section, collect all 4 of their responses, and assign one of them to apply the fix.",
            Strategy::Planning => "Respond only to Quality Assurance with `STATUS_UPDATE` intent. If you receive messages from active workers during this time, encourage them to finish their assigned task. Do not assign new tasks to workers. Task and Task Group creation and editing is enabled in this mode.",
            Strategy::WindDown => "Respond only to Quality Assurance with `STATUS_UPDATE`. If you receive messages from active workers during this time, encourage them to finish their assigned task. Do not assign new tasks to workers."
        }
    }
}

#[derive(Debug, Clone)]
pub enum OrchestratorHint {
    AssignTask {
        to_worker: i64,
        from_groups: Vec<String>,
    },
    SendNudge {
        to_worker: i64,
    },
    SendSupport {
        to_worker: i64,
    },
    NotifyQa,
}

impl OrchestratorHint {
    pub fn render(&self) -> String {
        match self {
            OrchestratorHint::AssignTask {
                to_worker,
                from_groups,
            } => format!(
                "ws{to_worker} is ready to receive a new task from any of these groups: {from_groups:?}"
            ),
            OrchestratorHint::SendNudge { to_worker } => format!(
                "Remind ws{to_worker} of the task they were given, remind them not to acknowledge your message, and to use `COMPLETE_TASK` intent when they are ready to run code validation."
            ),
            OrchestratorHint::SendSupport { to_worker } => format!(
                "Thank ws{to_worker} for their hard work, tell them not to acknowledge your message, and to use `COMPLETE_TASK` intent when they are ready to run code validation."
            ),
            OrchestratorHint::NotifyQa => "There seems to be an issue. Notify QA immediately."
                .to_string(),
        }
    }
}

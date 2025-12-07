use once_cell::sync::OnceCell;
use openapi::models::CommandConfig;
use parking_lot::RwLock;

use crate::post_turn_checks::PostTurnCheckRegistry;

use super::AgentRole;

pub struct ProjectCommandRegistry {
    commands: RwLock<Vec<CommandConfig>>,
}

static PROJECT_COMMANDS: OnceCell<ProjectCommandRegistry> = OnceCell::new();

impl ProjectCommandRegistry {
    pub fn global() -> &'static Self {
        PROJECT_COMMANDS.get_or_init(|| ProjectCommandRegistry {
            commands: RwLock::new(Vec::new()),
        })
    }

    pub fn replace(&self, commands: Vec<CommandConfig>) {
        *self.commands.write() = commands;
    }

    pub fn list(&self) -> Vec<CommandConfig> {
        self.commands.read().clone()
    }

    pub fn get(&self, id: &str) -> Option<CommandConfig> {
        self.commands
            .read()
            .iter()
            .find(|command| command.id == id)
            .cloned()
    }
}

pub fn command_visible_for_role(role: AgentRole, command: &CommandConfig) -> bool {
    if command.hidden.unwrap_or(false) {
        if matches!(role, AgentRole::Worker | AgentRole::Orchestrator) {
            return is_post_turn_check(&command.id);
        }
        false
    } else {
        true
    }
}

pub fn is_post_turn_check(id: &str) -> bool {
    PostTurnCheckRegistry::global()
        .list()
        .iter()
        .any(|check| check == id)
}

use once_cell::sync::OnceCell;
use openapi::models::CommandConfig;
use parking_lot::RwLock;

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

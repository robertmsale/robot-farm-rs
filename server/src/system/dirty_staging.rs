use std::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirtyStagingAction {
    Commit,
    Stash,
}

impl DirtyStagingAction {
    pub fn from_str(value: &str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "commit" => Some(DirtyStagingAction::Commit),
            "stash" => Some(DirtyStagingAction::Stash),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DirtyStagingAction::Commit => "commit",
            DirtyStagingAction::Stash => "stash",
        }
    }
}

static ACTION: RwLock<DirtyStagingAction> = RwLock::new(DirtyStagingAction::Commit);

pub fn current() -> DirtyStagingAction {
    ACTION
        .read()
        .map(|g| *g)
        .unwrap_or(DirtyStagingAction::Commit)
}

pub fn set(action: DirtyStagingAction) {
    if let Ok(mut slot) = ACTION.write() {
        *slot = action;
    }
}

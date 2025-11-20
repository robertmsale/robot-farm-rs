use std::sync::LazyLock;

use openapi::models::DockerOverrides as ApiDockerOverrides;
use parking_lot::RwLock;

use super::codex_config::AgentKind;

#[derive(Clone, Debug, Default)]
struct OverridesStore {
    orchestrator: Vec<String>,
    worker: Vec<String>,
    wizard: Vec<String>,
}

static STORE: LazyLock<RwLock<OverridesStore>> = LazyLock::new(|| RwLock::new(OverridesStore::default()));

pub fn reset() {
    *STORE.write() = OverridesStore::default();
}

pub fn replace(overrides: ApiDockerOverrides) {
    *STORE.write() = OverridesStore {
        orchestrator: overrides.orchestrator,
        worker: overrides.worker,
        wizard: overrides.wizard,
    };
}

pub fn overrides_for(kind: AgentKind) -> Vec<String> {
    let store = STORE.read();
    match kind {
        AgentKind::Orchestrator => store.orchestrator.clone(),
        AgentKind::Worker => store.worker.clone(),
        AgentKind::Wizard => store.wizard.clone(),
    }
}

pub fn apply_overrides(kind: AgentKind, args: &mut Vec<String>, image: &str) {
    let overrides = overrides_for(kind);
    if overrides.is_empty() {
        return;
    }
    let insert_at = args
        .iter()
        .position(|arg| arg == image)
        .unwrap_or_else(|| args.len());
    args.splice(insert_at..insert_at, overrides);
}

use std::sync::LazyLock;

use openapi::models::{
    AgentModelOverrides, AgentReasoningOverrides, ReasoningEffort,
    agent_model_overrides::{Orchestrator, Wizard, Worker},
};
use parking_lot::RwLock;

pub const MODEL_GPT_5_1_CODEX: &str = "gpt-5.1-codex";
pub const MODEL_GPT_5_1_CODEX_MAX: &str = "gpt-5.1-codex-max";
pub const MODEL_GPT_5_1_CODEX_MINI: &str = "gpt-5.1-codex-mini";
pub const MODEL_GPT_5_1: &str = "gpt-5.1";
pub const DEFAULT_REASONING: &str = "medium";

#[derive(Clone, Copy, Debug)]
pub enum AgentKind {
    Orchestrator,
    Worker,
    Wizard,
}

#[derive(Clone, Debug)]
pub struct PersonaLaunchSettings {
    pub model: String,
    pub reasoning: String,
}

#[derive(Clone, Debug)]
struct CodexSettings {
    models: AgentModelOverrides,
    reasoning: AgentReasoningOverrides,
}

static SETTINGS: LazyLock<RwLock<CodexSettings>> =
    LazyLock::new(|| RwLock::new(CodexSettings::default()));

impl Default for CodexSettings {
    fn default() -> Self {
        Self {
            models: default_models(),
            reasoning: default_reasoning(),
        }
    }
}

pub fn default_models() -> AgentModelOverrides {
    AgentModelOverrides {
        orchestrator: Orchestrator::Gpt5Period1Codex,
        worker: Worker::Gpt5Period1Codex,
        wizard: Wizard::Gpt5Period1Codex,
    }
}

pub fn default_reasoning() -> AgentReasoningOverrides {
    AgentReasoningOverrides {
        orchestrator: ReasoningEffort::Medium,
        worker: ReasoningEffort::Medium,
        wizard: ReasoningEffort::Medium,
    }
}

pub fn reset() {
    *SETTINGS.write() = CodexSettings::default();
}

pub fn replace(models: AgentModelOverrides, reasoning: AgentReasoningOverrides) {
    *SETTINGS.write() = CodexSettings { models, reasoning };
}

pub fn settings_for(kind: AgentKind) -> PersonaLaunchSettings {
    let store = SETTINGS.read();
    match kind {
        AgentKind::Orchestrator => PersonaLaunchSettings {
            model: orchestrator_model_name(&store.models.orchestrator).to_string(),
            reasoning: store.reasoning.orchestrator.to_string(),
        },
        AgentKind::Worker => PersonaLaunchSettings {
            model: worker_model_name(&store.models.worker).to_string(),
            reasoning: store.reasoning.worker.to_string(),
        },
        AgentKind::Wizard => PersonaLaunchSettings {
            model: wizard_model_name(&store.models.wizard).to_string(),
            reasoning: store.reasoning.wizard.to_string(),
        },
    }
}

pub fn validate_preferences(
    models: &AgentModelOverrides,
    reasoning: &AgentReasoningOverrides,
) -> Result<(), String> {
    ensure_supported(
        "orchestrator",
        models.orchestrator.into(),
        reasoning.orchestrator,
    )?;
    ensure_supported("worker", models.worker.into(), reasoning.worker)?;
    ensure_supported("wizard", models.wizard.into(), reasoning.wizard)?;
    Ok(())
}

fn ensure_supported(
    persona: &str,
    model: ModelChoice,
    reasoning: ReasoningEffort,
) -> Result<(), String> {
    if model.is_mini() && matches!(reasoning, ReasoningEffort::Low) {
        return Err(format!(
            "Reasoning level \"low\" is not supported when the {persona} persona uses {model}."
        ));
    }
    Ok(())
}

fn orchestrator_model_name(model: &Orchestrator) -> &'static str {
    match model {
        Orchestrator::Gpt5Period1CodexMax => MODEL_GPT_5_1_CODEX_MAX,
        Orchestrator::Gpt5Period1Codex => MODEL_GPT_5_1_CODEX,
        Orchestrator::Gpt5Period1CodexMini => MODEL_GPT_5_1_CODEX_MINI,
        Orchestrator::Gpt5Period1 => MODEL_GPT_5_1,
    }
}

fn worker_model_name(model: &Worker) -> &'static str {
    match model {
        Worker::Gpt5Period1CodexMax => MODEL_GPT_5_1_CODEX_MAX,
        Worker::Gpt5Period1Codex => MODEL_GPT_5_1_CODEX,
        Worker::Gpt5Period1CodexMini => MODEL_GPT_5_1_CODEX_MINI,
        Worker::Gpt5Period1 => MODEL_GPT_5_1,
    }
}

fn wizard_model_name(model: &Wizard) -> &'static str {
    match model {
        Wizard::Gpt5Period1CodexMax => MODEL_GPT_5_1_CODEX_MAX,
        Wizard::Gpt5Period1Codex => MODEL_GPT_5_1_CODEX,
        Wizard::Gpt5Period1CodexMini => MODEL_GPT_5_1_CODEX_MINI,
        Wizard::Gpt5Period1 => MODEL_GPT_5_1,
    }
}

#[derive(Clone, Copy)]
struct ModelChoice {
    label: &'static str,
    mini: bool,
}

impl ModelChoice {
    fn new(label: &'static str, mini: bool) -> Self {
        Self { label, mini }
    }

    fn is_mini(&self) -> bool {
        self.mini
    }
}

impl std::fmt::Display for ModelChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl From<Orchestrator> for ModelChoice {
    fn from(value: Orchestrator) -> Self {
        match value {
            Orchestrator::Gpt5Period1CodexMax => ModelChoice::new(MODEL_GPT_5_1_CODEX_MAX, false),
            Orchestrator::Gpt5Period1Codex => ModelChoice::new(MODEL_GPT_5_1_CODEX, false),
            Orchestrator::Gpt5Period1CodexMini => ModelChoice::new(MODEL_GPT_5_1_CODEX_MINI, true),
            Orchestrator::Gpt5Period1 => ModelChoice::new(MODEL_GPT_5_1, false),
        }
    }
}

impl From<Worker> for ModelChoice {
    fn from(value: Worker) -> Self {
        match value {
            Worker::Gpt5Period1CodexMax => ModelChoice::new(MODEL_GPT_5_1_CODEX_MAX, false),
            Worker::Gpt5Period1Codex => ModelChoice::new(MODEL_GPT_5_1_CODEX, false),
            Worker::Gpt5Period1CodexMini => ModelChoice::new(MODEL_GPT_5_1_CODEX_MINI, true),
            Worker::Gpt5Period1 => ModelChoice::new(MODEL_GPT_5_1, false),
        }
    }
}

impl From<Wizard> for ModelChoice {
    fn from(value: Wizard) -> Self {
        match value {
            Wizard::Gpt5Period1CodexMax => ModelChoice::new(MODEL_GPT_5_1_CODEX_MAX, false),
            Wizard::Gpt5Period1Codex => ModelChoice::new(MODEL_GPT_5_1_CODEX, false),
            Wizard::Gpt5Period1CodexMini => ModelChoice::new(MODEL_GPT_5_1_CODEX_MINI, true),
            Wizard::Gpt5Period1 => ModelChoice::new(MODEL_GPT_5_1, false),
        }
    }
}

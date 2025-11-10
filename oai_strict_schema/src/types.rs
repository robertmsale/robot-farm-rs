#![allow(dead_code)]

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Example schema used by the self-test to ensure end-to-end validation works.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MathReasoning {
    pub problem: String,
    pub plan: Vec<ReasoningStep>,
    pub final_answer: FinalAnswer,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReasoningStep {
    pub statement: String,
    pub justification: String,
    pub intermediate_result: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FinalAnswer {
    pub result: String,
    #[serde(default)]
    pub confidence: Option<f32>,
}

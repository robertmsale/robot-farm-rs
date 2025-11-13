use chrono::{DateTime, Utc};
use openapi::models::FeedLevel;

#[derive(Debug, Clone)]
pub struct SystemEvent {
    pub level: FeedLevel,
    pub source: SystemActor,
    pub target: SystemActor,
    pub category: SystemEventCategory,
    pub summary: String,
    pub details: serde_json::Value,
    pub emitted_at: DateTime<Utc>,
}

impl SystemEvent {
    pub fn new(
        level: FeedLevel,
        source: SystemActor,
        target: SystemActor,
        category: SystemEventCategory,
        summary: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            level,
            source,
            target,
            category,
            summary: summary.into(),
            details,
            emitted_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemActor {
    System,
    Orchestrator,
    Worker(i64),
    QualityAssurance,
}

impl SystemActor {
    pub fn label(&self) -> String {
        match self {
            SystemActor::System => "System".to_string(),
            SystemActor::Orchestrator => "Orchestrator".to_string(),
            SystemActor::QualityAssurance => "Quality Assurance".to_string(),
            SystemActor::Worker(id) => format!("ws{id}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemEventCategory {
    Strategy,
    Queue,
    Validation,
    User,
    Merge,
    Routing,
}

impl SystemEventCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            SystemEventCategory::Strategy => "strategy",
            SystemEventCategory::Queue => "queue",
            SystemEventCategory::Validation => "validation",
            SystemEventCategory::User => "user",
            SystemEventCategory::Merge => "merge",
            SystemEventCategory::Routing => "routing",
        }
    }
}

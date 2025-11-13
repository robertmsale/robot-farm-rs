use std::collections::{HashMap, HashSet};

use once_cell::sync::OnceCell;
use openapi::models::{ActiveStrategy, FeedLevel, Strategy as ApiStrategy};
use parking_lot::RwLock;
use serde_json::json;

use crate::models::strategy::OrchestratorHint;

use super::events::{SystemActor, SystemEvent, SystemEventCategory};

#[derive(Debug, Clone)]
pub struct AssignedTask {
    pub task_id: i64,
    pub slug: Option<String>,
}

#[derive(Default)]
struct QueueState {
    assignments: HashMap<i64, AssignedTask>,
    known_workers: HashSet<i64>,
    events: Vec<SystemEvent>,
}

pub struct QueueCoordinator {
    inner: RwLock<QueueState>,
}

static QUEUE_COORDINATOR: OnceCell<QueueCoordinator> = OnceCell::new();

impl QueueCoordinator {
    pub fn init_global() -> &'static QueueCoordinator {
        QUEUE_COORDINATOR.get_or_init(|| QueueCoordinator {
            inner: RwLock::new(QueueState::default()),
        })
    }

    pub fn global() -> &'static QueueCoordinator {
        QUEUE_COORDINATOR
            .get()
            .expect("queue coordinator not initialized")
    }

    pub fn register_worker(&self, worker_id: i64) {
        self.inner.write().known_workers.insert(worker_id);
    }

    pub fn unregister_worker(&self, worker_id: i64) {
        let mut guard = self.inner.write();
        guard.known_workers.remove(&worker_id);
        guard.assignments.remove(&worker_id);
    }

    pub fn assigned_task(&self, worker_id: i64) -> Option<AssignedTask> {
        self.inner.read().assignments.get(&worker_id).cloned()
    }

    pub fn assign_task(
        &self,
        worker_id: i64,
        task_id: i64,
        slug: Option<String>,
    ) -> Result<(), QueueError> {
        let mut guard = self.inner.write();
        if guard.assignments.contains_key(&worker_id) {
            return Err(QueueError::WorkerBusy);
        }
        guard
            .assignments
            .insert(worker_id, AssignedTask { task_id, slug });
        guard.known_workers.insert(worker_id);
        Ok(())
    }

    pub fn clear_assignment(&self, worker_id: i64) {
        self.inner.write().assignments.remove(&worker_id);
    }

    pub fn record_event(&self, event: SystemEvent) {
        self.inner.write().events.push(event);
    }

    pub fn drain_events(&self) -> Vec<SystemEvent> {
        let mut guard = self.inner.write();
        std::mem::take(&mut guard.events)
    }

    pub fn orchestrator_hints(&self, strategy: &ActiveStrategy) -> Vec<OrchestratorHint> {
        let guard = self.inner.read();
        let idle_workers: Vec<i64> = guard
            .known_workers
            .iter()
            .copied()
            .filter(|worker| !guard.assignments.contains_key(worker))
            .collect();

        if idle_workers.is_empty() {
            return Vec::new();
        }

        let focus = strategy
            .focus
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|group| group.to_string())
            .collect::<Vec<_>>();

        match strategy.id {
            ApiStrategy::Aggressive | ApiStrategy::HotfixSwarm | ApiStrategy::BugSmash => {
                idle_workers
                    .into_iter()
                    .map(|worker| OrchestratorHint::AssignTask {
                        to_worker: worker,
                        from_groups: focus.clone(),
                    })
                    .collect()
            }
            ApiStrategy::Moderate | ApiStrategy::Economical => idle_workers
                .into_iter()
                .map(|worker| OrchestratorHint::AssignTask {
                    to_worker: worker,
                    from_groups: focus.clone(),
                })
                .collect(),
            ApiStrategy::Planning | ApiStrategy::WindDown => idle_workers
                .into_iter()
                .map(|worker| OrchestratorHint::SendSupport { to_worker: worker })
                .collect(),
        }
    }

    pub fn record_assignment_hint(&self, hints: &[OrchestratorHint]) {
        for hint in hints {
            let event = SystemEvent::new(
                FeedLevel::Info,
                SystemActor::System,
                SystemActor::Orchestrator,
                SystemEventCategory::Strategy,
                hint.render(),
                json!({"hint": hint.render()}),
            );
            self.record_event(event);
        }
    }

    pub fn user_message(
        &self,
        source: SystemActor,
        target: SystemActor,
        message: impl Into<String>,
    ) {
        let summary = message.into();
        let event = SystemEvent::new(
            FeedLevel::Info,
            source,
            target,
            SystemEventCategory::User,
            summary.clone(),
            json!({ "message": summary }),
        );
        self.record_event(event);
    }

    pub fn validation_failed(&self, worker_id: i64, explanation: impl Into<String>) {
        let summary = explanation.into();
        let event = SystemEvent::new(
            FeedLevel::Warning,
            SystemActor::System,
            SystemActor::Worker(worker_id),
            SystemEventCategory::Validation,
            summary.clone(),
            json!({ "worker": worker_id, "details": summary }),
        );
        self.record_event(event);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum QueueError {
    #[error("worker already has an assignment")]
    WorkerBusy,
}

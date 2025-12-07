use std::sync::{LazyLock, RwLock};

#[derive(Debug, Clone, Copy)]
pub struct FeatureToggles {
    pub persistent_threads: bool,
    pub ghost_commits: bool,
    pub drift_manager: bool,
}

impl Default for FeatureToggles {
    fn default() -> Self {
        Self {
            persistent_threads: false,
            ghost_commits: false,
            drift_manager: false,
        }
    }
}

static FEATURES: LazyLock<RwLock<FeatureToggles>> =
    LazyLock::new(|| RwLock::new(FeatureToggles::default()));

pub fn replace(new: FeatureToggles) {
    if let Ok(mut guard) = FEATURES.write() {
        *guard = new;
    }
}

pub fn reset() {
    replace(FeatureToggles::default());
}

pub fn snapshot() -> FeatureToggles {
    FEATURES
        .read()
        .map(|guard| *guard)
        .unwrap_or_else(|_| FeatureToggles::default())
}

pub fn persistent_threads() -> bool {
    snapshot().persistent_threads
}

pub fn ghost_commits() -> bool {
    snapshot().ghost_commits
}

pub fn drift_manager() -> bool {
    snapshot().drift_manager
}

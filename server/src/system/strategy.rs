use once_cell::sync::OnceCell;
use openapi::models::{ActiveStrategy, Strategy};
use parking_lot::RwLock;

pub struct StrategyState {
    inner: RwLock<ActiveStrategy>,
}

static STRATEGY_STATE: OnceCell<StrategyState> = OnceCell::new();

impl StrategyState {
    pub fn init_global() -> &'static StrategyState {
        STRATEGY_STATE.get_or_init(|| StrategyState {
            inner: RwLock::new(ActiveStrategy {
                id: Strategy::Planning,
                focus: Some(vec![]),
            }),
        })
    }

    pub fn global() -> &'static StrategyState {
        STRATEGY_STATE
            .get()
            .expect("strategy state not initialized. call init_system_state()")
    }

    pub fn snapshot(&self) -> ActiveStrategy {
        self.inner.read().clone()
    }

    pub fn update(&self, strategy: ActiveStrategy) {
        *self.inner.write() = strategy;
    }
}

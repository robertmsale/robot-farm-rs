pub mod codex_config;
pub mod dirty_staging;
pub mod docker_overrides;
pub mod events;
pub mod features;
pub mod queue;
pub mod runner;
pub mod staging_hooks;
pub mod strategy;

pub fn init_system_state() {
    strategy::StrategyState::init_global();
    queue::QueueCoordinator::init_global();
}

pub mod events;
pub mod queue;
pub mod strategy;

pub fn init_system_state() {
    strategy::StrategyState::init_global();
    queue::QueueCoordinator::init_global();
}

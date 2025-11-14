use once_cell::sync::OnceCell;
use parking_lot::RwLock;

pub struct PostTurnCheckRegistry {
    checks: RwLock<Vec<String>>,
}

static POST_TURN_CHECKS: OnceCell<PostTurnCheckRegistry> = OnceCell::new();

impl PostTurnCheckRegistry {
    pub fn global() -> &'static Self {
        POST_TURN_CHECKS.get_or_init(|| PostTurnCheckRegistry {
            checks: RwLock::new(Vec::new()),
        })
    }

    pub fn replace(&self, checks: Vec<String>) {
        *self.checks.write() = checks;
    }

    pub fn list(&self) -> Vec<String> {
        self.checks.read().clone()
    }
}

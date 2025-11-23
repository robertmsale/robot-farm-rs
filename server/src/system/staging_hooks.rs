use std::sync::RwLock;

static HOOKS: RwLock<Vec<String>> = RwLock::new(Vec::new());

pub fn replace(list: Vec<String>) {
    if let Ok(mut hooks) = HOOKS.write() {
        *hooks = list;
    }
}

pub fn list() -> Vec<String> {
    HOOKS.read().map(|h| h.clone()).unwrap_or_default()
}

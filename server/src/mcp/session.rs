use std::collections::HashMap;

use once_cell::sync::OnceCell;
use serde_json::Value;
use tokio::sync::{RwLock, broadcast};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SessionSender {
    sender: broadcast::Sender<Value>,
}

impl SessionSender {
    pub fn subscribe(&self) -> broadcast::Receiver<Value> {
        self.sender.subscribe()
    }
}

pub struct SessionManager {
    inner: RwLock<HashMap<String, SessionSender>>,
}

static SESSION_MANAGER: OnceCell<SessionManager> = OnceCell::new();

impl SessionManager {
    pub fn global() -> &'static SessionManager {
        SESSION_MANAGER.get_or_init(|| SessionManager {
            inner: RwLock::new(HashMap::new()),
        })
    }

    pub async fn create_session(&self) -> String {
        let (sender, _rx) = broadcast::channel(128);
        let session_id = Self::generate_session_id();
        let mut guard = self.inner.write().await;
        guard.insert(session_id.clone(), SessionSender { sender });
        session_id
    }

    pub async fn has_session(&self, session_id: &str) -> bool {
        self.inner.read().await.contains_key(session_id)
    }

    pub async fn subscribe(&self, session_id: &str) -> Option<broadcast::Receiver<Value>> {
        let guard = self.inner.read().await;
        guard.get(session_id).map(|sender| sender.subscribe())
    }

    pub async fn remove_session(&self, session_id: &str) {
        self.inner.write().await.remove(session_id);
    }

    fn generate_session_id() -> String {
        loop {
            let id = Uuid::new_v4().to_string();
            if id.chars().all(|ch| ch >= '!' && ch <= '~') {
                return id;
            }
        }
    }
}

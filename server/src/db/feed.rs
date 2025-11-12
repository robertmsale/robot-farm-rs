use openapi::models::{Feed, FeedLevel};

fn sample_feed(id: i64) -> Feed {
    Feed {
        id,
        source: "System".to_string(),
        target: "Orchestrator".to_string(),
        ts: 0,
        level: FeedLevel::Info,
        text: format!("Sample feed event #{id}"),
        raw: "{}".to_string(),
        category: "general".to_string(),
    }
}

pub async fn list_feed() -> Vec<Feed> {
    // TODO: SELECT feed rows with filters.
    vec![sample_feed(1)]
}

pub async fn delete_feed() -> bool {
    // TODO: DELETE FROM feed.
    true
}

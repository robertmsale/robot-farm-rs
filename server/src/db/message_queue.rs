use openapi::models::{InsertMessage, Message};

fn sample_message(id: i64) -> Message {
    Message {
        id,
        from: "Orchestrator".to_string(),
        to: "Quality Assurance".to_string(),
        message: "Placeholder message".to_string(),
        inserted_at: 0,
    }
}

pub async fn list_messages() -> Vec<Message> {
    // TODO: SELECT queue entries.
    vec![sample_message(1)]
}

pub async fn delete_all_messages() -> bool {
    // TODO: TRUNCATE message_queue.
    true
}

pub async fn insert_message_relative(_message_id: i64, _directive: InsertMessage) -> Vec<Message> {
    // TODO: Adjust queue ordering.
    vec![sample_message(1)]
}

pub async fn delete_message_by_id(_message_id: i64) -> bool {
    // TODO: DELETE FROM message_queue WHERE id = $1.
    true
}

pub async fn delete_messages_for_recipient(_recipient: &str) -> bool {
    // TODO: DELETE FROM message_queue WHERE to_actor = $1.
    true
}

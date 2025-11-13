use serde::{Deserialize, Serialize};

/// Top-level event emitted by `codex exec --json`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CodexEvent {
    #[serde(rename = "thread.started")]
    ThreadStarted { thread_id: String },
    #[serde(rename = "turn.started")]
    TurnStarted,
    #[serde(rename = "turn.completed")]
    TurnCompleted { usage: TokenUsage },
    #[serde(rename = "turn.failed")]
    TurnFailed { error: EventError },
    #[serde(rename = "item.started")]
    ItemStarted { item: TurnItem },
    #[serde(rename = "item.updated")]
    ItemUpdated { item: TurnItem },
    #[serde(rename = "item.completed")]
    ItemCompleted { item: TurnItem },
    #[serde(rename = "error")]
    Error { message: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: i64,
    pub cached_input_tokens: i64,
    pub output_tokens: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventError {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TurnItem {
    pub id: String,
    #[serde(flatten)]
    pub detail: TurnItemDetail,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TurnItemDetail {
    #[serde(rename = "agent_message")]
    AgentMessage { text: String },
    #[serde(rename = "reasoning")]
    Reasoning { text: String },
    #[serde(rename = "command_execution")]
    CommandExecution(CommandExecutionItem),
    #[serde(rename = "file_change")]
    FileChange(FileChangeItem),
    #[serde(rename = "mcp_tool_call")]
    McpToolCall(McpToolCallItem),
    #[serde(rename = "web_search")]
    WebSearch { query: String },
    #[serde(rename = "todo_list")]
    TodoList { items: Vec<TodoEntry> },
    #[serde(rename = "error")]
    ItemError { message: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandExecutionItem {
    pub command: String,
    pub aggregated_output: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    pub status: CommandExecutionStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandExecutionStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileChangeItem {
    pub changes: Vec<FileChangeEntry>,
    pub status: PatchApplyStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileChangeEntry {
    pub path: String,
    pub kind: FileChangeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileChangeKind {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "update")]
    Update,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchApplyStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct McpToolCallItem {
    pub server: String,
    pub tool: String,
    pub status: McpToolCallStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum McpToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TodoEntry {
    pub text: String,
    pub completed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sample_events() {
        let json = r#"{ "type": "thread.started", "thread_id": "abc-123" }"#;
        let event: CodexEvent = serde_json::from_str(json).unwrap();
        match event {
            CodexEvent::ThreadStarted { thread_id } => assert_eq!(thread_id, "abc-123"),
            _ => panic!("unexpected variant"),
        }

        let usage_json = r#"
        {
            "type": "turn.completed",
            "usage": { "input_tokens": 10, "cached_input_tokens": 2, "output_tokens": 5 }
        }"#;
        let event: CodexEvent = serde_json::from_str(usage_json).unwrap();
        if let CodexEvent::TurnCompleted { usage } = event {
            assert_eq!(usage.input_tokens, 10);
            assert_eq!(usage.cached_input_tokens, 2);
            assert_eq!(usage.output_tokens, 5);
        } else {
            panic!("unexpected variant");
        }

        let item_json = r#"
        {
            "type": "item.completed",
            "item": {
                "id": "item_0",
                "type": "command_execution",
                "command": "ls -la",
                "aggregated_output": "done",
                "exit_code": 0,
                "status": "completed"
            }
        }"#;
        let event: CodexEvent = serde_json::from_str(item_json).unwrap();
        match event {
            CodexEvent::ItemCompleted { item } => {
                assert_eq!(item.id, "item_0");
                match item.detail {
                    TurnItemDetail::CommandExecution(CommandExecutionItem {
                        command,
                        exit_code,
                        status,
                        ..
                    }) => {
                        assert_eq!(command, "ls -la");
                        assert_eq!(exit_code, Some(0));
                        assert_eq!(status, CommandExecutionStatus::Completed);
                    }
                    other => panic!("unexpected detail: {other:?}"),
                }
            }
            _ => panic!("unexpected event"),
        }
    }
}

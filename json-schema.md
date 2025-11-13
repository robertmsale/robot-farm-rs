# codex exec `--json` event schema

## Output format
- `codex exec --json` streams a JSON Lines (JSONL) feed to stdout; each line serializes a single `ThreadEvent` from `codex-rs/exec/src/exec_events.rs`.
- Events are emitted in real time as Codex works. A typical turn starts with `thread.started`, then `turn.started`, a series of `item.*` events, and finally either `turn.completed` (success) or `turn.failed` (error). A separate top-level `error` event can appear mid-turn when the stream reports an unrecoverable problem.
- Item identifiers (`item.id`) are generated sequentially as `"item_<n>"` for the lifetime of the process; the counter does not reset between turns.
- When `--output-last-message/-o` is supplied, Codex still writes the JSONL feed to stdout. The last agent message is additionally written to the requested file.

## Top-level event envelope
Every line has the shape:

```jsonc
{ "type": "<event-kind>", /* event specific fields */ }
```

Supported `type` values and payloads:

### `thread.started`
- Fields: `thread_id` (`string`) – UUID of the session (`SessionConfiguredEvent`).

### `turn.started`
- No additional fields. The serialized object is simply `{"type":"turn.started"}`; represents `TaskStarted`.

### `turn.completed`
- Fields: `usage` (`object`) with:
  - `input_tokens` (`number`, i64)
  - `cached_input_tokens` (`number`, i64)
  - `output_tokens` (`number`, i64)
- Populated from the most recent token-count snapshot when the turn completes successfully.

### `turn.failed`
- Fields: `error` (`object`) with `message` (`string`).
- Emitted after `turn.started` if a fatal error was seen earlier in the turn.

### `item.started`, `item.updated`, `item.completed`
- Fields: `item` (`object`) – see [Thread items](#thread-items).
- `item.started` is emitted when work begins (e.g., command launch, first plan update).
- `item.updated` surfaces intermediate state changes (currently only for the plan/todo list).
- `item.completed` signals a terminal state.

### `error`
- Fields: `message` (`string`).
- Generated for unrecoverable stream errors; `turn.failed` will follow at turn end.

## Thread items
The nested `item` object shares this envelope:

```jsonc
{
  "id": "item_0",
  "type": "<item-type>",
  /* variant-specific body */
}
```

Supported `type` values:

### `agent_message`
- Fields: `text` (`string`).
- Contains the agent’s natural-language reply. If `--output-schema` is provided, the field contains the structured JSON payload **stringified** (see [Structured output](#structured-output-and-json-mode)).

### `reasoning`
- Fields: `text` (`string`) – reasoning summary chunk.

### `command_execution`
- Fields:
  - `command` (`string`) – shell command with arguments re-joined via `shlex::try_join`.
  - `aggregated_output` (`string`) – combined STDOUT/STDERR captured so far (empty on `item.started`).
  - `exit_code` (`number`, optional) – present on `item.completed`.
  - `status` (`string`) – `"in_progress"`, `"completed"`, or `"failed"` (`CommandExecutionStatus`).

### `file_change`
- Fields:
  - `changes` (`array`) of `{ "path": <string>, "kind": "add"|"delete"|"update" }`.
  - `status` (`string`) – `"completed"` or `"failed"` (`PatchApplyStatus`).
- Emitted once per patch application after Codex finishes applying edits.

### `mcp_tool_call`
- Fields:
  - `server` (`string`)
  - `tool` (`string`)
  - `status` (`string`) – `"in_progress"`, `"completed"`, or `"failed"` (`McpToolCallStatus`).

### `web_search`
- Fields: `query` (`string`).
- Produced when an MCP web-search tool finishes.

### `todo_list`
- Fields: `items` (`array`) of `{ "text": <string>, "completed": <boolean> }`.
- `item.started` introduces the plan, `item.updated` reflects step status changes, and the same item id is marked `item.completed` when the turn ends.

### `error`
- Fields: `message` (`string`).
- Reserved for non-fatal error items. The current JSON event processor does not emit this variant yet, but it is part of the schema.

## Structured output and JSON mode
- `--output-schema <FILE>` forwards the JSON Schema to Codex (`final_output_json_schema`) but does **not** change the event structure. The final reply is still delivered as `{"type":"item.completed","item":{"id":"item_n","type":"agent_message","text": ...}}`.
- When the model returns structured JSON, Codex serializes that object to a string before emitting the event. Consumers must parse `item.text` if they need the structured object.

Example final event with structured output (pretty-printed for clarity):

```json
{
  "type": "item.completed",
  "item": {
    "id": "item_5",
    "type": "agent_message",
    "text": "{\"project_name\":\"codex\",\"languages\":[\"Rust\",\"TypeScript\"]}"
  }
}
```

To recover the JSON payload you must JSON-decode the line, then parse `item.text` separately.

## Error handling summary
- A fatal error triggers:
  1. `{"type":"error","message": "..."}`
  2. Subsequent `{"type":"turn.failed","error":{"message":"..."}}` when the turn completes.
- The `turn.failed` event replaces `turn.completed`; there is no usage block in the failure case.

## Implementation references
- Event shapes: `codex-rs/exec/src/exec_events.rs`
- JSON streaming: `codex-rs/exec/src/event_processor_with_jsonl_output.rs`
- Structured output comment for agent messages: `AgentMessageItem` in `exec_events.rs` and `parse_turn_item` in `codex-rs/core/src/event_mapping.rs`.

import 'package:flutter/material.dart';

enum CodexEventType {
  threadStarted,
  turnStarted,
  turnCompleted,
  turnFailed,
  itemStarted,
  itemUpdated,
  itemCompleted,
  error,
}

enum CodexItemType {
  agentMessage,
  reasoning,
  commandExecution,
  fileChange,
  mcpToolCall,
  webSearch,
  todoList,
  error,
}

class CodexEvent {
  const CodexEvent({
    required this.type,
    this.threadId,
    this.usage,
    this.error,
    this.item,
  });

  final CodexEventType type;
  final String? threadId;
  final CodexTokenUsage? usage;
  final String? error;
  final CodexItem? item;

  factory CodexEvent.threadStarted(String threadId) => CodexEvent(
        type: CodexEventType.threadStarted,
        threadId: threadId,
      );

  factory CodexEvent.turnStarted() =>
      const CodexEvent(type: CodexEventType.turnStarted);

  factory CodexEvent.turnCompleted(CodexTokenUsage usage) => CodexEvent(
        type: CodexEventType.turnCompleted,
        usage: usage,
      );

  factory CodexEvent.turnFailed(String message) => CodexEvent(
        type: CodexEventType.turnFailed,
        error: message,
      );

  factory CodexEvent.item({
    required CodexEventType phase,
    required CodexItem item,
  }) {
    assert(phase == CodexEventType.itemStarted ||
        phase == CodexEventType.itemUpdated ||
        phase == CodexEventType.itemCompleted);
    return CodexEvent(type: phase, item: item);
  }

  factory CodexEvent.streamError(String message) =>
      CodexEvent(type: CodexEventType.error, error: message);
}

class CodexTokenUsage {
  const CodexTokenUsage({
    required this.inputTokens,
    required this.cachedInputTokens,
    required this.outputTokens,
  });

  final int inputTokens;
  final int cachedInputTokens;
  final int outputTokens;
}

class CodexItem {
  const CodexItem({
    required this.id,
    required this.type,
    this.message,
    this.command,
    this.output,
    this.exitCode,
    this.status,
    this.fileChanges,
    this.toolCall,
    this.query,
    this.todos,
  });

  final String id;
  final CodexItemType type;
  final String? message;
  final String? command;
  final String? output;
  final int? exitCode;
  final String? status;
  final List<CodexFileChange>? fileChanges;
  final CodexToolCall? toolCall;
  final String? query;
  final List<CodexTodo>? todos;
}

class CodexFileChange {
  const CodexFileChange({
    required this.path,
    required this.kind,
  });

  final String path;
  final CodexFileChangeKind kind;
}

enum CodexFileChangeKind { add, delete, update }

class CodexToolCall {
  const CodexToolCall({
    required this.server,
    required this.tool,
    required this.status,
  });

  final String server;
  final String tool;
  final String status;
}

class CodexTodo {
  const CodexTodo({required this.text, required this.completed});

  final String text;
  final bool completed;
}

final List<CodexEvent> mockCodexEvents = [
  CodexEvent.threadStarted('e5821b4c-101a-4a0d-8f2c-123456789abc'),
  CodexEvent.turnStarted(),
  CodexEvent.item(
    phase: CodexEventType.itemStarted,
    item: CodexItem(
      id: 'item_reasoning',
      type: CodexItemType.reasoning,
      message: 'Need to validate the worker list strategy before editing settings.',
    ),
  ),
  CodexEvent.item(
    phase: CodexEventType.itemStarted,
    item: CodexItem(
      id: 'item_0',
      type: CodexItemType.todoList,
      todos: const [
        CodexTodo(text: 'Read config.json', completed: true),
        CodexTodo(text: 'Plan backend updates', completed: false),
      ],
    ),
  ),
  CodexEvent.item(
    phase: CodexEventType.itemCompleted,
    item: CodexItem(
      id: 'item_1',
      type: CodexItemType.mcpToolCall,
      toolCall: const CodexToolCall(
        server: 'web-search',
        tool: 'search',
        status: 'completed',
      ),
    ),
  ),
  CodexEvent.item(
    phase: CodexEventType.itemCompleted,
    item: CodexItem(
      id: 'item_2',
      type: CodexItemType.webSearch,
      query: 'Tokio bounded channel examples',
    ),
  ),
  CodexEvent.item(
    phase: CodexEventType.itemCompleted,
    item: CodexItem(
      id: 'item_3',
      type: CodexItemType.commandExecution,
      command: 'rg TODO server/src',
      output: 'Found 4 TODOs\nserver/src/main.rs:42: TODO',
      exitCode: 0,
      status: 'completed',
    ),
  ),
  CodexEvent.item(
    phase: CodexEventType.itemCompleted,
    item: CodexItem(
      id: 'item_4',
      type: CodexItemType.fileChange,
      status: 'completed',
      fileChanges: const [
        CodexFileChange(path: 'server/src/db/task.rs', kind: CodexFileChangeKind.update),
        CodexFileChange(path: 'client/lib/main.dart', kind: CodexFileChangeKind.update),
      ],
    ),
  ),
  CodexEvent.item(
    phase: CodexEventType.itemCompleted,
    item: CodexItem(
      id: 'item_5',
      type: CodexItemType.agentMessage,
      message:
          'Updated the DB layer with real SQLx queries. Next I will wire the feed UI.',
    ),
  ),
  CodexEvent.item(
    phase: CodexEventType.itemCompleted,
    item: CodexItem(
      id: 'item_6',
      type: CodexItemType.error,
      message: 'Failed to parse cargo metadata. Falling back to manual run.',
    ),
  ),
  CodexEvent.turnCompleted(
    const CodexTokenUsage(
      inputTokens: 1420,
      cachedInputTokens: 80,
      outputTokens: 512,
    ),
  ),
  CodexEvent.turnFailed('Network hiccup while contacting the MCP server.'),
  CodexEvent.item(
    phase: CodexEventType.itemStarted,
    item: CodexItem(
      id: 'item_retry',
      type: CodexItemType.commandExecution,
      command: 'git status --short',
      output: '',
      status: 'in_progress',
    ),
  ),
  CodexEvent.item(
    phase: CodexEventType.itemUpdated,
    item: CodexItem(
      id: 'item_retry',
      type: CodexItemType.commandExecution,
      command: 'git status --short',
      output: ' M server/src/main.rs',
      status: 'in_progress',
    ),
  ),
  CodexEvent.item(
    phase: CodexEventType.itemCompleted,
    item: CodexItem(
      id: 'item_retry',
      type: CodexItemType.commandExecution,
      command: 'git status --short',
      output: ' M server/src/main.rs\n?? server/src/models/codex_events.rs',
      exitCode: 0,
      status: 'completed',
    ),
  ),
  CodexEvent.item(
    phase: CodexEventType.itemCompleted,
    item: CodexItem(
      id: 'item_final_agent',
      type: CodexItemType.agentMessage,
      message: 'Retry succeeded. Ready for the next command.',
    ),
  ),
  CodexEvent.turnCompleted(
    const CodexTokenUsage(
      inputTokens: 600,
      cachedInputTokens: 0,
      outputTokens: 210,
    ),
  ),
];

IconData iconForEvent(CodexEvent event) {
  switch (event.type) {
    case CodexEventType.threadStarted:
      return Icons.bolt;
    case CodexEventType.turnStarted:
      return Icons.play_arrow;
    case CodexEventType.turnCompleted:
      return Icons.check_circle;
    case CodexEventType.turnFailed:
      return Icons.error;
    case CodexEventType.itemStarted:
    case CodexEventType.itemUpdated:
    case CodexEventType.itemCompleted:
      return Icons.list_alt;
    case CodexEventType.error:
      return Icons.warning;
  }
}

Color? colorForEvent(BuildContext context, CodexEvent event) {
  final colors = Theme.of(context).colorScheme;
  switch (event.type) {
    case CodexEventType.turnCompleted:
      return colors.primary;
    case CodexEventType.turnFailed:
    case CodexEventType.error:
      return colors.error;
    default:
      return colors.secondary;
  }
}

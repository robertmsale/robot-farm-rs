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

/// Mock system events (non-Codex) so the feed can preview the system payload style.
class SystemFeedEvent {
  const SystemFeedEvent({
    required this.level,
    required this.source,
    required this.target,
    required this.category,
    required this.summary,
    required this.details,
  });

  final FeedLevel level;
  final String source;
  final String target;
  final String category;
  final String summary;
  final String details;

  Color badgeColor(ColorScheme scheme) {
    switch (level) {
      case FeedLevel.error:
        return scheme.error;
      case FeedLevel.warning:
        return scheme.tertiary;
      case FeedLevel.info:
        return scheme.primary;
    }
  }
}

enum FeedLevel { info, warning, error }

final List<SystemFeedEvent> mockSystemEvents = [
  SystemFeedEvent(
    level: FeedLevel.info,
    source: 'System',
    target: 'Orchestrator',
    category: 'strategy',
    summary: 'ws2 is idle — assign a task from `chores` or `bugs`.',
    details:
        'Strategy Planner (Planning) detected idle workers: ws2. Focus groups: chores, bugs.',
  ),
  SystemFeedEvent(
    level: FeedLevel.warning,
    source: 'System',
    target: 'ws3',
    category: 'validation',
    summary: 'Post-turn validation failed.',
    details:
        'Tests failed: `cargo test` exited with code 101. The worktree was not merged into staging.',
  ),
  SystemFeedEvent(
    level: FeedLevel.info,
    source: 'User',
    target: 'Orchestrator',
    category: 'user',
    summary: '“Remember to keep worker ws1 focused on CLI improvements.”',
    details:
        'Manual message injected into orchestrator feed via system message endpoint.',
  ),
  SystemFeedEvent(
    level: FeedLevel.error,
    source: 'System',
    target: 'Orchestrator',
    category: 'merge',
    summary: 'Merge conflict detected while merging ws4 → staging.',
    details:
        'Conflict files: server/src/routes/task.rs, client/lib/main.dart. Worker notified to resolve and re-run COMPLETE_TASK.',
  ),
  SystemFeedEvent(
    level: FeedLevel.info,
    source: 'System',
    target: 'Orchestrator',
    category: 'queue',
    summary: 'HotfixSwarm: four workers activated.',
    details:
        'Workers ws1 (frontend), ws2 (backend), ws3 (database), ws4 (deps) are now assigned per strategy.',
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

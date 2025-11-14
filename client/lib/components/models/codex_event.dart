import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;

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

CodexItemType _itemTypeFromString(String? value) {
  switch (value) {
    case 'agent_message':
      return CodexItemType.agentMessage;
    case 'reasoning':
      return CodexItemType.reasoning;
    case 'command_execution':
      return CodexItemType.commandExecution;
    case 'file_change':
      return CodexItemType.fileChange;
    case 'mcp_tool_call':
      return CodexItemType.mcpToolCall;
    case 'web_search':
      return CodexItemType.webSearch;
    case 'todo_list':
      return CodexItemType.todoList;
    case 'error':
      return CodexItemType.error;
    default:
      return CodexItemType.agentMessage;
  }
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

  factory CodexEvent.threadStarted(String threadId) =>
      CodexEvent(type: CodexEventType.threadStarted, threadId: threadId);

  factory CodexEvent.turnStarted() =>
      const CodexEvent(type: CodexEventType.turnStarted);

  factory CodexEvent.turnCompleted(CodexTokenUsage usage) =>
      CodexEvent(type: CodexEventType.turnCompleted, usage: usage);

  factory CodexEvent.turnFailed(String message) =>
      CodexEvent(type: CodexEventType.turnFailed, error: message);

  factory CodexEvent.item({
    required CodexEventType phase,
    required CodexItem item,
  }) {
    assert(
      phase == CodexEventType.itemStarted ||
          phase == CodexEventType.itemUpdated ||
          phase == CodexEventType.itemCompleted,
    );
    return CodexEvent(type: phase, item: item);
  }

  factory CodexEvent.streamError(String message) =>
      CodexEvent(type: CodexEventType.error, error: message);

  factory CodexEvent.fromJson(Map<String, dynamic> json) {
    final typeString = json['type'] as String? ?? '';
    switch (typeString) {
      case 'thread.started':
        return CodexEvent.threadStarted(json['thread_id']?.toString() ?? '');
      case 'turn.started':
        return CodexEvent.turnStarted();
      case 'turn.completed':
        return CodexEvent.turnCompleted(
          CodexTokenUsage.fromJson(json['usage'] as Map<String, dynamic>?),
        );
      case 'turn.failed':
        final message = json['error'] is Map<String, dynamic>
            ? ((json['error'] as Map<String, dynamic>)['message']?.toString() ??
                  'Unknown error')
            : json['error']?.toString() ?? 'Unknown error';
        return CodexEvent.turnFailed(message);
      case 'item.started':
        return CodexEvent.item(
          phase: CodexEventType.itemStarted,
          item: CodexItem.fromJson(json['item'] as Map<String, dynamic>?),
        );
      case 'item.updated':
        return CodexEvent.item(
          phase: CodexEventType.itemUpdated,
          item: CodexItem.fromJson(json['item'] as Map<String, dynamic>?),
        );
      case 'item.completed':
        return CodexEvent.item(
          phase: CodexEventType.itemCompleted,
          item: CodexItem.fromJson(json['item'] as Map<String, dynamic>?),
        );
      case 'error':
        return CodexEvent.streamError(
          json['message']?.toString() ?? 'Unknown error',
        );
      default:
        return CodexEvent.streamError('Unhandled event: $typeString');
    }
  }

  String describe() {
    switch (type) {
      case CodexEventType.threadStarted:
        return 'Thread started: ${threadId ?? 'unknown'}';
      case CodexEventType.turnStarted:
        return 'Turn started';
      case CodexEventType.turnCompleted:
        final usage = this.usage;
        if (usage == null) {
          return 'Turn completed';
        }
        return 'Turn completed (${usage.outputTokens} output tokens)';
      case CodexEventType.turnFailed:
        return 'Turn failed: ${error ?? 'Unknown error'}';
      case CodexEventType.itemStarted:
      case CodexEventType.itemUpdated:
      case CodexEventType.itemCompleted:
        return item?.describe(type) ?? 'Item event';
      case CodexEventType.error:
        return 'Stream error: ${error ?? 'Unknown error'}';
    }
  }
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

  factory CodexTokenUsage.fromJson(Map<String, dynamic>? json) {
    if (json == null) {
      return const CodexTokenUsage(
        inputTokens: 0,
        cachedInputTokens: 0,
        outputTokens: 0,
      );
    }
    return CodexTokenUsage(
      inputTokens: (json['input_tokens'] as num?)?.toInt() ?? 0,
      cachedInputTokens: (json['cached_input_tokens'] as num?)?.toInt() ?? 0,
      outputTokens: (json['output_tokens'] as num?)?.toInt() ?? 0,
    );
  }
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

  factory CodexItem.fromJson(Map<String, dynamic>? json) {
    if (json == null) {
      return const CodexItem(id: 'item', type: CodexItemType.agentMessage);
    }
    final changes = (json['changes'] as List<dynamic>?)
        ?.map(
          (entry) => CodexFileChange.fromJson(entry as Map<String, dynamic>?),
        )
        .whereType<CodexFileChange>()
        .toList();
    final todos = (json['items'] as List<dynamic>?)
        ?.map((entry) => CodexTodo.fromJson(entry as Map<String, dynamic>?))
        .whereType<CodexTodo>()
        .toList();
    return CodexItem(
      id: json['id']?.toString() ?? 'item',
      type: _itemTypeFromString(json['type']?.toString()),
      message: json['text']?.toString(),
      command: json['command']?.toString(),
      output: json['aggregated_output']?.toString(),
      exitCode: (json['exit_code'] as num?)?.toInt(),
      status: json['status']?.toString(),
      fileChanges: changes,
      toolCall: CodexToolCall.fromJson(
        json['tool_call'] as Map<String, dynamic>?,
      ),
      query: json['query']?.toString(),
      todos: todos,
    );
  }

  String describe(CodexEventType phase) {
    switch (type) {
      case CodexItemType.agentMessage:
        return message ?? 'Agent response';
      case CodexItemType.reasoning:
        return message ?? 'Reasoning update';
      case CodexItemType.commandExecution:
        return '${_phaseLabel(phase)} command: ${command ?? 'unknown'}';
      case CodexItemType.fileChange:
        return 'File changes (${fileChanges?.length ?? 0})';
      case CodexItemType.mcpToolCall:
        return 'Tool ${toolCall?.tool ?? 'unknown'} (${toolCall?.status ?? 'status'})';
      case CodexItemType.webSearch:
        return 'Web search: ${query ?? 'unknown'}';
      case CodexItemType.todoList:
        return 'Todo list updated';
      case CodexItemType.error:
        return message ?? 'Item error';
    }
  }

  String _phaseLabel(CodexEventType phase) {
    switch (phase) {
      case CodexEventType.itemStarted:
        return 'Running';
      case CodexEventType.itemUpdated:
        return 'Updating';
      case CodexEventType.itemCompleted:
        return 'Completed';
      default:
        return '';
    }
  }
}

class CodexFileChange {
  const CodexFileChange({required this.path, required this.kind});

  final String path;
  final CodexFileChangeKind kind;

  factory CodexFileChange.fromJson(Map<String, dynamic>? json) {
    if (json == null) {
      return const CodexFileChange(
        path: 'unknown',
        kind: CodexFileChangeKind.update,
      );
    }
    return CodexFileChange(
      path: json['path']?.toString() ?? 'unknown',
      kind: _fileKindFromString(json['kind']?.toString()),
    );
  }
}

enum CodexFileChangeKind { add, delete, update }

CodexFileChangeKind _fileKindFromString(String? value) {
  switch (value) {
    case 'add':
      return CodexFileChangeKind.add;
    case 'delete':
      return CodexFileChangeKind.delete;
    default:
      return CodexFileChangeKind.update;
  }
}

class CodexToolCall {
  const CodexToolCall({
    required this.server,
    required this.tool,
    required this.status,
  });

  final String server;
  final String tool;
  final String status;

  factory CodexToolCall.fromJson(Map<String, dynamic>? json) {
    if (json == null) {
      return const CodexToolCall(
        server: 'robot_farm',
        tool: 'unknown',
        status: 'completed',
      );
    }
    return CodexToolCall(
      server: json['server']?.toString() ?? 'robot_farm',
      tool: json['tool']?.toString() ?? 'unknown',
      status: json['status']?.toString() ?? 'unknown',
    );
  }
}

class CodexTodo {
  const CodexTodo({required this.text, required this.completed});

  final String text;
  final bool completed;

  factory CodexTodo.fromJson(Map<String, dynamic>? json) {
    if (json == null) {
      return const CodexTodo(text: '', completed: false);
    }
    return CodexTodo(
      text: json['text']?.toString() ?? '',
      completed: json['completed'] == true,
    );
  }
}

class SystemFeedEvent {
  const SystemFeedEvent({
    required this.level,
    required this.source,
    required this.target,
    required this.category,
    required this.summary,
    required this.details,
    required this.timestamp,
    this.feed,
  });

  final SystemFeedLevel level;
  final String source;
  final String target;
  final String category;
  final String summary;
  final String details;
  final DateTime timestamp;
  final robot_farm_api.Feed? feed;

  factory SystemFeedEvent.fromFeed(robot_farm_api.Feed entry) {
    return SystemFeedEvent(
      level: _mapFeedLevel(entry.level),
      source: entry.source_,
      target: entry.target,
      category: entry.category,
      summary: entry.text,
      details: _formatFeedDetails(entry.raw),
      timestamp: DateTime.fromMillisecondsSinceEpoch(
        entry.ts * 1000,
        isUtc: true,
      ),
      feed: entry,
    );
  }

  Color badgeColor(ColorScheme scheme) {
    switch (level) {
      case SystemFeedLevel.error:
        return scheme.error;
      case SystemFeedLevel.warning:
        return scheme.tertiary;
      case SystemFeedLevel.info:
        return scheme.primary;
    }
  }
}

enum SystemFeedLevel { info, warning, error }

SystemFeedLevel _mapFeedLevel(robot_farm_api.FeedLevel level) {
  switch (level) {
    case robot_farm_api.FeedLevel.info:
      return SystemFeedLevel.info;
    case robot_farm_api.FeedLevel.warning:
      return SystemFeedLevel.warning;
    case robot_farm_api.FeedLevel.error:
      return SystemFeedLevel.error;
  }
  return SystemFeedLevel.info;
}

String _formatFeedDetails(String raw) {
  final trimmed = raw.trim();
  if (trimmed.isEmpty) {
    return 'No additional details.';
  }
  try {
    final decoded = jsonDecode(trimmed);
    const encoder = JsonEncoder.withIndent('  ');
    return encoder.convert(decoded);
  } catch (_) {
    return raw;
  }
}

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

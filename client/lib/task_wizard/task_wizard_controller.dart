import 'dart:async';
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;
import 'package:web_socket/web_socket.dart' as ws;

import '../components/models/codex_event.dart';
import '../main.dart' show ConnectionController, kDefaultApiPort;

class TaskWizardController extends GetxController {
  TaskWizardController(this._connection);

  final ConnectionController _connection;

  final TextEditingController promptController = TextEditingController();
  final RxBool isRunning = false.obs;
  final RxBool hasPrompt = false.obs;
  final RxBool isConnected = false.obs;
  final RxList<TaskWizardFeedEntry> feed = <TaskWizardFeedEntry>[].obs;
  final RxnString error = RxnString();

  ws.WebSocket? _socket;
  StreamSubscription<ws.WebSocketEvent>? _subscription;
  String? _sessionId;
  String? _threadId;

  String? get threadId => _threadId;
  String? get sessionId => _sessionId;

  @override
  void onReady() {
    super.onReady();
    promptController.addListener(_handlePromptChanged);
    _connect();
  }

  @override
  void onClose() {
    promptController.removeListener(_handlePromptChanged);
    promptController.dispose();
    _subscription?.cancel();
    _socket?.close();
    super.onClose();
  }

  bool get canSendPrompt =>
      isConnected.value && !isRunning.value && hasPrompt.value;

  Future<void> sendPrompt() async {
    final prompt = promptController.text.trim();
    if (prompt.isEmpty) {
      Get.snackbar('Missing input', 'Enter instructions before sending.');
      return;
    }

    if (!isConnected.value || _socket == null) {
      Get.snackbar('Not connected', 'Connect to the server first.');
      return;
    }

    try {
      _socket!.sendText(jsonEncode({'type': 'prompt', 'prompt': prompt}));
      feed.add(TaskWizardFeedEntry.user(prompt));
      promptController.clear();
      hasPrompt.value = false;
      isRunning.value = true;
    } catch (err) {
      Get.snackbar('Failed to send prompt', '$err');
    }
  }

  Future<void> cancelRun() async {
    if (!isRunning.value || _socket == null) {
      return;
    }
    try {
      _socket!.sendText(jsonEncode({'type': 'cancel'}));
    } catch (err) {
      Get.snackbar('Failed to cancel run', '$err');
    }
  }

  Future<void> _connect() async {
    final url = _connection.currentBaseUrl;
    if (url == null) {
      error.value = 'Connect to the main server before opening the wizard.';
      return;
    }

    final wsUrl = _buildWizardWebsocket(url);
    if (wsUrl == null) {
      error.value = 'Invalid base URL: $url';
      return;
    }

    try {
      _subscription?.cancel();
      _socket?.close();
      _socket = await ws.WebSocket.connect(Uri.parse(wsUrl));
      isConnected.value = true;
      error.value = null;
      _subscription = _socket!.events.listen(
        _handleWebsocketEvent,
        onDone: () {
          isConnected.value = false;
          isRunning.value = false;
          _threadId = null;
        },
        onError: (Object err) {
          error.value = 'Wizard socket error: $err';
          isConnected.value = false;
          isRunning.value = false;
        },
      );
    } catch (err) {
      error.value = 'Failed to open wizard socket: $err';
      isConnected.value = false;
    }
  }

  void _handleWebsocketEvent(ws.WebSocketEvent event) {
    switch (event) {
      case ws.TextDataReceived(text: final text):
        _handleServerMessage(text);
      case ws.CloseReceived(:final code, :final reason):
        error.value =
            'Wizard socket closed: ${code ?? 1005}${reason.isNotEmpty ? ' ($reason)' : ''}';
        isConnected.value = false;
        isRunning.value = false;
      default:
        break;
    }
  }

  void _handleServerMessage(String text) {
    if (text.trim().isEmpty) {
      return;
    }

    dynamic decoded;
    try {
      decoded = jsonDecode(text);
    } catch (err) {
      feed.add(TaskWizardFeedEntry.system('Malformed wizard event: $err'));
      return;
    }

    if (decoded is! Map<String, dynamic>) {
      return;
    }

    final type = decoded['type'] as String?;
    switch (type) {
      case 'welcome':
        _sessionId = decoded['sessionId'] as String?;
        _threadId = decoded['threadId'] as String?;
        break;
      case 'status':
        final state = decoded['state'] as String?;
        isRunning.value = state == 'running' || state == 'cancelling';
        break;
      case 'thread':
        _threadId = decoded['threadId'] as String?;
        break;
      case 'codex_event':
        final eventJson = decoded['event'];
        if (eventJson is Map<String, dynamic>) {
          final event = CodexEvent.fromJson(eventJson);
          feed.add(TaskWizardFeedEntry.event(event));
        }
        break;
      case 'log':
        final stream = decoded['stream'] ?? 'stdout';
        final line = decoded['line'] ?? '';
        feed.add(TaskWizardFeedEntry.system('[$stream] $line'));
        break;
      case 'error':
        final message = decoded['message']?.toString() ?? 'Unknown error';
        feed.add(TaskWizardFeedEntry.system('Error: $message'));
        Get.snackbar('Task Wizard', message);
        isRunning.value = false;
        break;
      case 'final':
        isRunning.value = false;
        final status = decoded['status']?.toString() ?? 'completed';
        final response = decoded['response']?.toString();
        robot_farm_api.Feed? feedEntry;
        if (decoded['feedEntry'] is Map<String, dynamic>) {
          feedEntry = robot_farm_api.Feed.fromJson(
            decoded['feedEntry'] as Map<String, dynamic>,
          );
        }
        if (response != null) {
          feed.add(
            TaskWizardFeedEntry.finalMessage(
              response,
              status: status,
              feedEntry: feedEntry,
            ),
          );
        } else if (decoded['error'] != null) {
          feed.add(
            TaskWizardFeedEntry.system('Wizard $status: ${decoded['error']}'),
          );
        }
        break;
      case 'pong':
        break;
      default:
        break;
    }
  }

  void _handlePromptChanged() {
    hasPrompt.value = promptController.text.trim().isNotEmpty;
  }

  String? _buildWizardWebsocket(String baseUrl) {
    Uri? uri;
    if (baseUrl.startsWith('http')) {
      uri = Uri.tryParse(baseUrl);
    } else {
      uri = Uri.tryParse('http://$baseUrl');
    }
    if (uri == null || uri.host.isEmpty) {
      return null;
    }
    final scheme = uri.scheme == 'https' ? 'wss' : 'ws';
    final port = uri.hasPort ? uri.port : kDefaultApiPort;
    final resolved = Uri(
      scheme: scheme,
      host: uri.host,
      port: port,
      path: '/task-wizard/ws',
    );
    return resolved.toString();
  }
}

enum TaskWizardFeedEntryType { userPrompt, wizardEvent, system, finalSummary }

class TaskWizardFeedEntry {
  const TaskWizardFeedEntry._({
    required this.type,
    required this.message,
    this.event,
    this.status,
    this.feedEntry,
  });

  factory TaskWizardFeedEntry.user(String text) => TaskWizardFeedEntry._(
    type: TaskWizardFeedEntryType.userPrompt,
    message: text,
  );

  factory TaskWizardFeedEntry.system(String text) => TaskWizardFeedEntry._(
    type: TaskWizardFeedEntryType.system,
    message: text,
  );

  factory TaskWizardFeedEntry.event(CodexEvent event) => TaskWizardFeedEntry._(
    type: TaskWizardFeedEntryType.wizardEvent,
    message: event.describe(),
    event: event,
  );

  factory TaskWizardFeedEntry.finalMessage(
    String text, {
    required String status,
    robot_farm_api.Feed? feedEntry,
  }) => TaskWizardFeedEntry._(
    type: TaskWizardFeedEntryType.finalSummary,
    message: text,
    status: status,
    feedEntry: feedEntry,
  );

  final TaskWizardFeedEntryType type;
  final String message;
  final CodexEvent? event;
  final String? status;
  final robot_farm_api.Feed? feedEntry;
}

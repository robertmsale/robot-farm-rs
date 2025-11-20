import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;
import 'package:shared_preferences/shared_preferences.dart';
import 'package:web_socket/web_socket.dart' as ws;

import 'components/models/codex_event.dart';
import 'git_status/git_status_controller.dart';
import 'git_status/git_status_screen.dart';
import 'sheets/command_sheet.dart';
import 'sheets/message/message_sheet.dart';
import 'sheets/queue/queue_sheet.dart';
import 'sheets/strategy/strategy_sheet.dart';
import 'task_wizard/task_wizard_controller.dart';
import 'task_wizard/task_wizard_screen.dart';
import 'tasks/tasks_controller.dart';
import 'tasks/tasks_screen.dart';

const int kDefaultApiPort = 8080;
const String kWebsocketPath = '/ws';
const String kPastLoginsKey = 'past_logins';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  Get.put(ConnectionController(), permanent: true);
  runApp(const RobotFarmApp());
}

class RobotFarmApp extends StatelessWidget {
  const RobotFarmApp({super.key});

  @override
  Widget build(BuildContext context) {
    return GetMaterialApp(
      title: 'Robot Farm Client',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.greenAccent),
        useMaterial3: true,
      ),
      initialRoute: '/',
      getPages: [
        GetPage(name: '/', page: () => const ConnectionScreen()),
        GetPage(name: '/home', page: () => const HomeScreen()),
        GetPage(name: '/settings', page: () => const SettingsScreen()),
        GetPage(
          name: '/tasks',
          page: () => const TasksScreen(),
          binding: BindingsBuilder(() {
            Get.put(
              TasksController(
                () => Get.find<ConnectionController>().currentBaseUrl,
                () => Get.find<ConnectionController>().workers
                    .map((worker) => 'ws${worker.id}')
                    .toList(),
              ),
            );
          }),
        ),
        GetPage(
          name: '/git-status',
          page: () => const GitStatusScreen(),
          binding: BindingsBuilder(() {
            Get.put(
              GitStatusController(
                () => Get.find<ConnectionController>().currentBaseUrl,
              ),
            );
          }),
        ),
        GetPage(
          name: '/task-wizard',
          page: () => const TaskWizardScreen(),
          binding: BindingsBuilder(() {
            Get.put(TaskWizardController(Get.find<ConnectionController>()));
          }),
        ),
      ],
    );
  }
}

enum HealthStatus { idle, checking, ok, down }

enum WebsocketStatus { idle, connecting, good, failed }

class ConnectionController extends GetxController {
  final TextEditingController urlController = TextEditingController();
  final Rx<HealthStatus> healthStatus = HealthStatus.idle.obs;
  final Rx<WebsocketStatus> websocketStatus = WebsocketStatus.idle.obs;
  final RxnString errorMessage = RxnString();
  final RxnString websocketError = RxnString();
  final RxnString orchestratorThreadId = RxnString();
  final RxList<String> pastLogins = <String>[].obs;
  final RxList<robot_farm_api.Worker> workers = <robot_farm_api.Worker>[].obs;
  final RxBool isPlaying = true.obs;
  final Rxn<robot_farm_api.ActiveStrategy> activeStrategy = Rxn(null);
  final StreamController<robot_farm_api.Feed> _feedController =
      StreamController<robot_farm_api.Feed>.broadcast();
  final StreamController<void> _feedClearedController =
      StreamController<void>.broadcast();
  ws.WebSocket? _webSocket;
  StreamSubscription<ws.WebSocketEvent>? _webSocketSubscription;
  SharedPreferences? _prefs;
  String? _currentBaseUrl;
  Timer? _reconnectTimer;

  @override
  void onInit() {
    super.onInit();
    _loadPastLogins();
  }

  Future<void> connect() async {
    final rawUrl = urlController.text.trim();
    websocketStatus.value = WebsocketStatus.idle;
    websocketError.value = null;
    _currentBaseUrl = null;

    if (rawUrl.isEmpty) {
      errorMessage.value = 'Please enter a server host.';
      healthStatus.value = HealthStatus.down;
      websocketStatus.value = WebsocketStatus.failed;
      _closeWebSocket();
      return;
    }

    final baseUrl = _buildBaseUrl(rawUrl);
    if (baseUrl == null) {
      errorMessage.value =
          'Please enter a host or host:port (paths and schemes are not required).';
      healthStatus.value = HealthStatus.down;
      websocketStatus.value = WebsocketStatus.failed;
      _closeWebSocket();
      return;
    }

    final healthy = await _performHealthCheck(baseUrl);
    if (!healthy) {
      websocketStatus.value = WebsocketStatus.failed;
      _closeWebSocket();
      return;
    }

    final socketConnected = await _connectWebsocket(baseUrl);
    if (!socketConnected) {
      return;
    }

    final hostPort = _hostPortFromBase(baseUrl);
    await _recordLogin(hostPort);

    _currentBaseUrl = baseUrl;
    _scheduleReconnect();
    await refreshQueueState();
    await refreshStrategy();
    Get.offAllNamed('/home');
  }

  Future<bool> _performHealthCheck(String baseUrl) async {
    try {
      healthStatus.value = HealthStatus.checking;
      errorMessage.value = null;

      final client = robot_farm_api.ApiClient(basePath: baseUrl);
      final api = robot_farm_api.DefaultApi(client);
      final response = await api.getHealthz();

      if (response != null && response.status.toLowerCase() == 'ok') {
        healthStatus.value = HealthStatus.ok;
        return true;
      } else {
        healthStatus.value = HealthStatus.down;
        errorMessage.value = 'Server responded but did not return OK.';
      }
    } on robot_farm_api.ApiException catch (error) {
      errorMessage.value =
          error.message ?? 'Request failed with status ${error.code}.';
      healthStatus.value = HealthStatus.down;
    } catch (error) {
      errorMessage.value = 'Failed to contact server: $error';
      healthStatus.value = HealthStatus.down;
    }

    return false;
  }

  Future<bool> _connectWebsocket(String baseUrl) async {
    websocketStatus.value = WebsocketStatus.connecting;
    websocketError.value = null;

    final wsUri = _buildWebsocketUri(baseUrl);

    try {
      _closeWebSocket();
      _webSocket = await ws.WebSocket.connect(wsUri);
      websocketStatus.value = WebsocketStatus.good;
      websocketError.value = null;
      _webSocketSubscription = _webSocket!.events.listen((event) {
        switch (event) {
          case ws.TextDataReceived(text: final text):
            _handleWebsocketText(text);
          case ws.CloseReceived(code: final code, reason: final reason):
            websocketStatus.value = WebsocketStatus.failed;
            final statusCode = code ?? 1005;
            final suffix = reason.isNotEmpty ? ' ($reason)' : '';
            websocketError.value = 'WebSocket closed: $statusCode$suffix';
            _scheduleReconnect();
          default:
            break;
        }
      });
      return true;
    } catch (error) {
      websocketStatus.value = WebsocketStatus.failed;
      websocketError.value = 'WebSocket failed: $error';
      _scheduleReconnect();
      return false;
    }
  }

  void _closeWebSocket() {
    _reconnectTimer?.cancel();
    _webSocketSubscription?.cancel();
    _webSocket?.close();
    _webSocketSubscription = null;
    _webSocket = null;
  }

  void _scheduleReconnect() {
    _reconnectTimer?.cancel();
    final baseUrl = _currentBaseUrl;
    if (baseUrl == null || websocketStatus.value == WebsocketStatus.good) {
      return;
    }
    _reconnectTimer = Timer(const Duration(seconds: 5), () {
      if (_currentBaseUrl == null) {
        return;
      }
      _connectWebsocket(baseUrl);
    });
  }

  void _handleWebsocketText(String text) {
    if (text.trim().isEmpty || text == 'ready') {
      return;
    }

    try {
      final decoded = jsonDecode(text);
      if (decoded is! Map<String, dynamic>) {
        return;
      }

      final type = decoded['type'];
      if (type == 'workers_snapshot') {
        _updateWorkers(decoded['workers']);
      } else if (type == 'worker_thread') {
        _handleWorkerThread(decoded);
      } else if (type == 'orchestrator_thread') {
        _handleOrchestratorThread(decoded);
      } else if (type == 'feed_entry') {
        _handleFeedEntry(decoded['entry']);
      } else if (type == 'feed_cleared') {
        _handleFeedCleared();
      } else if (type == 'queue_state') {
        _applyQueueState(decoded['paused']);
      } else if (type == 'strategy_state') {
        _applyStrategyState(decoded['strategy']);
      } else if (type == 'websocket_server_closed') {
        // server hint to reconnect? none expected but placeholder
      }
    } catch (error) {
      debugPrint('Failed to parse WebSocket message: $error');
    }
  }

  Stream<robot_farm_api.Feed> get feedEvents => _feedController.stream;
  Stream<void> get feedCleared => _feedClearedController.stream;

  void _updateWorkers(dynamic payload) {
    if (payload is! List) {
      return;
    }

    final parsed = payload
        .map((item) => robot_farm_api.Worker.fromJson(item))
        .whereType<robot_farm_api.Worker>()
        .toList();
    workers.assignAll(parsed);
  }

  void _handleWorkerThread(dynamic payload) {
    if (payload is! Map<String, dynamic>) {
      return;
    }
    final workerId = payload['worker_id'];
    if (workerId is! int) {
      return;
    }
    final rawThread = payload['thread_id'];
    final threadId = rawThread == null
        ? null
        : rawThread.toString().trim().isEmpty
            ? null
            : rawThread.toString();
    final index = workers.indexWhere((w) => w.id == workerId);
    if (index == -1) {
      return;
    }
    final current = workers[index];
    workers[index] = robot_farm_api.Worker(
      id: current.id,
      lastSeen: current.lastSeen,
      state: current.state,
      threadId: threadId,
    );
    workers.refresh();
  }

  void _handleOrchestratorThread(dynamic payload) {
    if (payload is! Map<String, dynamic>) {
      return;
    }
    final rawThread = payload['thread_id'];
    final value = rawThread == null
        ? null
        : rawThread.toString().trim().isEmpty
            ? null
            : rawThread.toString();
    orchestratorThreadId.value = value;
  }

  void _handleFeedEntry(dynamic payload) {
    if (payload is! Map<String, dynamic>) {
      return;
    }
    final entry = robot_farm_api.Feed.fromJson(payload);
    if (entry != null) {
      _feedController.add(entry);
    }
  }

  void _handleFeedCleared() {
    _feedClearedController.add(null);
    orchestratorThreadId.value = null;
  }

  void _applyQueueState(dynamic rawPaused) {
    if (rawPaused is bool) {
      isPlaying.value = !rawPaused;
    }
  }

  void _applyStrategyState(dynamic payload) {
    if (payload is Map<String, dynamic>) {
      final strategy = robot_farm_api.ActiveStrategy.fromJson(payload);
      if (strategy != null) {
        activeStrategy.value = strategy;
      }
    }
  }

  Future<void> togglePlayPause() async {
    final baseUrl = _currentBaseUrl;
    if (baseUrl == null) {
      Get.snackbar(
        'Not connected',
        'Connect to a server before toggling the queue.',
      );
      return;
    }

    final targetPaused = isPlaying.value;
    try {
      final updatedPaused = await _sendQueueStateRequest(
        baseUrl,
        method: 'PUT',
        body: {'paused': targetPaused},
      );
      isPlaying.value = !(updatedPaused ?? targetPaused);
    } on robot_farm_api.ApiException catch (error) {
      Get.snackbar(
        'Failed to update queue',
        error.message ?? 'Status ${error.code}',
        snackPosition: SnackPosition.BOTTOM,
      );
    } catch (error) {
      Get.snackbar(
        'Failed to update queue',
        '$error',
        snackPosition: SnackPosition.BOTTOM,
      );
    }
  }

  Future<void> refreshQueueState() async {
    final baseUrl = _currentBaseUrl;
    if (baseUrl == null) {
      return;
    }
    try {
      final paused = await _sendQueueStateRequest(baseUrl, method: 'GET');
      if (paused != null) {
        isPlaying.value = !paused;
      }
    } catch (error) {
      debugPrint('Failed to refresh queue state: $error');
    }
  }

  Future<void> refreshStrategy() async {
    final baseUrl = _currentBaseUrl;
    if (baseUrl == null) {
      return;
    }
    try {
      final api = robot_farm_api.DefaultApi(
        robot_farm_api.ApiClient(basePath: baseUrl),
      );
      final state = await api.getActiveStrategy();
      if (state != null) {
        activeStrategy.value = state;
      }
    } catch (error) {
      debugPrint('Failed to refresh strategy: $error');
    }
  }

  Future<bool?> _sendQueueStateRequest(
    String baseUrl, {
    required String method,
    Map<String, dynamic>? body,
  }) async {
    final client = robot_farm_api.ApiClient(basePath: baseUrl);
    final response = await client.invokeAPI(
      '/queue',
      method,
      const <robot_farm_api.QueryParam>[],
      body,
      <String, String>{},
      <String, String>{},
      body == null ? null : 'application/json',
    );
    if (response.statusCode >= HttpStatus.badRequest) {
      throw robot_farm_api.ApiException(response.statusCode, response.body);
    }
    if (response.body.isEmpty) {
      return null;
    }
    final decoded = jsonDecode(response.body);
    if (decoded is Map<String, dynamic>) {
      final paused = decoded['paused'];
      if (paused is bool) {
        return paused;
      }
    }
    return null;
  }

  Future<void> clearFeeds() async {
    final baseUrl = _currentBaseUrl;
    if (baseUrl == null) {
      Get.snackbar('Not connected', 'Connect to a server first.');
      return;
    }

    try {
      final client = robot_farm_api.ApiClient(basePath: baseUrl);
      final api = robot_farm_api.DefaultApi(client);
      await api.deleteFeed();
    } on robot_farm_api.ApiException catch (err) {
      Get.snackbar(
        'Failed to clear feeds',
        err.message ?? 'Status ${err.code}',
      );
    } catch (err) {
      Get.snackbar('Failed to clear feeds', '$err');
    }
  }

  Future<void> createWorker() async {
    final baseUrl = _currentBaseUrl;
    if (baseUrl == null) {
      Get.snackbar(
        'Not connected',
        'Connect to a server before adding workers.',
      );
      return;
    }

    final api = robot_farm_api.DefaultApi(
      robot_farm_api.ApiClient(basePath: baseUrl),
    );

    try {
      final worker = await api.createWorker();
      if (worker == null) {
        Get.snackbar(
          'Failed to add worker',
          'Server returned an empty response.',
        );
      }
    } on robot_farm_api.ApiException catch (err) {
      Get.snackbar('Failed to add worker', err.message ?? 'Status ${err.code}');
    } catch (err) {
      Get.snackbar('Failed to add worker', '$err');
    }
  }

  Future<void> terminateWorker(int workerId) async {
    final baseUrl = _currentBaseUrl;
    if (baseUrl == null) {
      Get.snackbar('Not connected', 'Connect to a server before terminating workers.');
      return;
    }

    final client = robot_farm_api.ApiClient(basePath: baseUrl);
    try {
      final response = await client.invokeAPI(
        '/workers/$workerId/terminate',
        'POST',
        const <robot_farm_api.QueryParam>[],
        null,
        <String, String>{},
        <String, String>{},
        null,
      );
      if (response.statusCode >= HttpStatus.badRequest) {
        throw robot_farm_api.ApiException(response.statusCode, response.body);
      }
    } on robot_farm_api.ApiException catch (err) {
      Get.snackbar(
        'Failed to terminate worker',
        err.message ?? 'Status ${err.code}',
      );
    } catch (err) {
      Get.snackbar('Failed to terminate worker', '$err');
    }
  }

  Future<void> terminateOrchestrator() async {
    final baseUrl = _currentBaseUrl;
    if (baseUrl == null) {
      Get.snackbar('Not connected', 'Connect to a server before terminating the orchestrator.');
      return;
    }

    final client = robot_farm_api.ApiClient(basePath: baseUrl);
    try {
      final response = await client.invokeAPI(
        '/orchestrator/terminate',
        'POST',
        const <robot_farm_api.QueryParam>[],
        null,
        <String, String>{},
        <String, String>{},
        null,
      );
      if (response.statusCode >= HttpStatus.badRequest) {
        throw robot_farm_api.ApiException(response.statusCode, response.body);
      }
    } on robot_farm_api.ApiException catch (err) {
      Get.snackbar(
        'Failed to terminate orchestrator',
        err.message ?? 'Status ${err.code}',
      );
    } catch (err) {
      Get.snackbar('Failed to terminate orchestrator', '$err');
    }
  }

  Future<robot_farm_api.Feed?> fetchFeedEntry(int feedId) async {
    final baseUrl = _currentBaseUrl;
    if (baseUrl == null) {
      throw StateError('Not connected to server');
    }
    final client = robot_farm_api.ApiClient(basePath: baseUrl);
    final response = await client.invokeAPI(
      '/feed/$feedId',
      'GET',
      const <robot_farm_api.QueryParam>[],
      null,
      <String, String>{},
      <String, String>{},
      null,
    );
    if (response.statusCode >= HttpStatus.badRequest) {
      throw robot_farm_api.ApiException(response.statusCode, response.body);
    }
    if (response.body.isEmpty) {
      return null;
    }
    final decoded = jsonDecode(response.body);
    if (decoded is Map<String, dynamic>) {
      return robot_farm_api.Feed.fromJson(decoded);
    }
    return null;
  }

  String? get currentBaseUrl => _currentBaseUrl;

  String get healthStatusLabel {
    switch (healthStatus.value) {
      case HealthStatus.ok:
        return 'Health OK';
      case HealthStatus.down:
        return 'Health down';
      case HealthStatus.checking:
        return 'Checking health...';
      case HealthStatus.idle:
        return 'Health unknown';
    }
  }

  String get websocketStatusLabel {
    switch (websocketStatus.value) {
      case WebsocketStatus.good:
        return 'WebSocket good';
      case WebsocketStatus.failed:
        return 'WebSocket down';
      case WebsocketStatus.connecting:
        return 'WebSocket connecting...';
      case WebsocketStatus.idle:
        return 'WebSocket idle';
    }
  }

  Color healthStatusColor(ThemeData theme) {
    switch (healthStatus.value) {
      case HealthStatus.ok:
        return Colors.green;
      case HealthStatus.down:
        return theme.colorScheme.error;
      case HealthStatus.checking:
        return theme.colorScheme.primary;
      case HealthStatus.idle:
        return theme.colorScheme.outline;
    }
  }

  Color websocketStatusColor(ThemeData theme) {
    switch (websocketStatus.value) {
      case WebsocketStatus.good:
        return Colors.green;
      case WebsocketStatus.failed:
        return theme.colorScheme.error;
      case WebsocketStatus.connecting:
        return theme.colorScheme.primary;
      case WebsocketStatus.idle:
        return theme.colorScheme.outline;
    }
  }

  String? _buildBaseUrl(String input) {
    final trimmed = input.trim();
    if (trimmed.isEmpty) {
      return null;
    }

    final hasScheme = trimmed.contains('://');
    final candidate = hasScheme ? trimmed : 'http://$trimmed';
    final uri = Uri.tryParse(candidate);

    if (uri == null || uri.host.isEmpty) {
      return null;
    }

    if ((uri.path.isNotEmpty && uri.path != '/') ||
        uri.hasQuery ||
        uri.hasFragment) {
      return null;
    }

    final scheme = uri.scheme.isEmpty ? 'http' : uri.scheme;
    final port = uri.hasPort ? uri.port : kDefaultApiPort;

    return Uri(scheme: scheme, host: uri.host, port: port).toString();
  }

  Uri _buildWebsocketUri(String baseUrl) {
    final httpUri = Uri.parse(baseUrl);
    final scheme = httpUri.scheme == 'https' ? 'wss' : 'ws';
    return Uri(
      scheme: scheme,
      host: httpUri.host,
      port: httpUri.hasPort ? httpUri.port : kDefaultApiPort,
      path: kWebsocketPath,
    );
  }

  String _hostPortFromBase(String baseUrl) {
    final uri = Uri.parse(baseUrl);
    final port = uri.hasPort ? uri.port : kDefaultApiPort;
    return '${uri.host}:$port';
  }

  bool get isConnecting =>
      healthStatus.value == HealthStatus.checking ||
      websocketStatus.value == WebsocketStatus.connecting;

  Future<void> _loadPastLogins() async {
    _prefs ??= await SharedPreferences.getInstance();
    final stored = _prefs!.getStringList(kPastLoginsKey) ?? <String>[];
    pastLogins.assignAll(stored);
  }

  Future<void> _recordLogin(String hostPort) async {
    _prefs ??= await SharedPreferences.getInstance();
    final updated = <String>[
      hostPort,
      ...pastLogins.where((v) => v != hostPort),
    ];
    if (updated.length > 5) {
      updated.removeRange(5, updated.length);
    }
    pastLogins.assignAll(updated);
    await _prefs!.setStringList(kPastLoginsKey, updated);
  }

  void usePastLogin(String hostPort) {
    urlController.text = hostPort;
  }

  @override
  void onClose() {
    urlController.dispose();
    _closeWebSocket();
    _currentBaseUrl = null;
    workers.clear();
    _feedController.close();
    _feedClearedController.close();
    super.onClose();
  }
}

class ConnectionScreen extends GetView<ConnectionController> {
  const ConnectionScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Scaffold(
      appBar: AppBar(title: const Text('Robot Farm')),
      body: SafeArea(
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 500),
            child: Padding(
              padding: const EdgeInsets.all(24),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text(
                    'Robot Farm Client',
                    style: theme.textTheme.headlineSmall,
                  ),
                  const SizedBox(height: 12),
                  Text(
                    'Enter the API server host (host[:port]). Defaults to port $kDefaultApiPort.',
                    textAlign: TextAlign.center,
                    style: theme.textTheme.bodyMedium,
                  ),
                  const SizedBox(height: 24),
                  TextField(
                    controller: controller.urlController,
                    keyboardType: TextInputType.url,
                    decoration: const InputDecoration(
                      labelText: 'Server host',
                      hintText: 'localhost:8080',
                      helperText: 'Format: hostname[:port], default port 8080',
                      border: OutlineInputBorder(),
                    ),
                  ),
                  const SizedBox(height: 16),
                  Obx(
                    () => SizedBox(
                      width: double.infinity,
                      child: FilledButton(
                        onPressed: controller.isConnecting
                            ? null
                            : controller.connect,
                        child: controller.isConnecting
                            ? const SizedBox(
                                width: 18,
                                height: 18,
                                child: CircularProgressIndicator(
                                  strokeWidth: 2,
                                  valueColor: AlwaysStoppedAnimation<Color>(
                                    Colors.white,
                                  ),
                                ),
                              )
                            : const Text('Connect'),
                      ),
                    ),
                  ),
                  const SizedBox(height: 24),
                  Obx(() {
                    if (controller.pastLogins.isEmpty) {
                      return const SizedBox.shrink();
                    }
                    return Column(
                      crossAxisAlignment: CrossAxisAlignment.stretch,
                      children: [
                        Text(
                          'Recent connections',
                          style: theme.textTheme.labelLarge,
                        ),
                        const SizedBox(height: 8),
                        Wrap(
                          spacing: 8,
                          runSpacing: 8,
                          children: controller.pastLogins
                              .map(
                                (host) => ActionChip(
                                  label: Text(host),
                                  onPressed: () =>
                                      controller.usePastLogin(host),
                                ),
                              )
                              .toList(),
                        ),
                      ],
                    );
                  }),
                  const SizedBox(height: 24),
                  Obx(
                    () => Column(
                      children: [
                        Text(
                          controller.healthStatusLabel,
                          style: theme.textTheme.titleLarge?.copyWith(
                            color: controller.healthStatusColor(theme),
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        if (controller.errorMessage.value != null) ...[
                          const SizedBox(height: 8),
                          Text(
                            controller.errorMessage.value!,
                            textAlign: TextAlign.center,
                            style: theme.textTheme.bodyMedium?.copyWith(
                              color: theme.colorScheme.error,
                            ),
                          ),
                        ],
                      ],
                    ),
                  ),
                  const SizedBox(height: 16),
                  Obx(
                    () => Column(
                      children: [
                        Text(
                          controller.websocketStatusLabel,
                          style: theme.textTheme.titleMedium?.copyWith(
                            color: controller.websocketStatusColor(theme),
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        if (controller.websocketError.value != null) ...[
                          const SizedBox(height: 8),
                          Text(
                            controller.websocketError.value!,
                            textAlign: TextAlign.center,
                            style: theme.textTheme.bodyMedium?.copyWith(
                              color: theme.colorScheme.error,
                            ),
                          ),
                        ],
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class HomeScreen extends GetView<ConnectionController> {
  const HomeScreen({super.key});

  void _openCommandSheet(BuildContext context, {int? workerId}) {
    final baseUrl = controller.currentBaseUrl;
    if (baseUrl == null) {
      Get.snackbar(
        'Not connected',
        'Connect to a server before running commands.',
      );
      return;
    }

    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (_) => CommandSheet(baseUrl: baseUrl, workerId: workerId),
    );
  }

  void _openStrategySheet(BuildContext context) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (_) =>
          StrategySheet(baseUrlProvider: () => controller.currentBaseUrl),
    );
  }

  void _openQueueSheet(BuildContext context, {int? workerId}) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (_) =>
          QueueSheet(baseUrlProvider: () => controller.currentBaseUrl),
    );
  }

  void _openMessageSheet(BuildContext context, {int? workerId}) {
    final target = workerId == null ? 'Orchestrator' : 'ws$workerId';
    final defaultSender = workerId == null
        ? 'Quality Assurance'
        : 'Orchestrator';
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (_) => EnqueueMessageSheet(
        baseUrlProvider: () => controller.currentBaseUrl,
        initialTarget: target,
        initialSender: defaultSender,
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final isPhone = context.isPhone;
    final orchestratorPane = OrchestratorPane(
      connection: controller,
      onRunCommand: () => _openCommandSheet(context),
      onEnqueueMessage: () => _openMessageSheet(context),
      onEditQueue: () => _openQueueSheet(context),
    );
    final workerPane = WorkerFeedPane(
      connection: controller,
      onRunCommand: (workerId) =>
          _openCommandSheet(context, workerId: workerId),
      onEnqueueMessage: (workerId) =>
          _openMessageSheet(context, workerId: workerId),
      onEditQueue: (workerId) => _openQueueSheet(context, workerId: workerId),
      onAddWorker: () => controller.createWorker(),
    );

    final child = isPhone
        ? Column(
            children: [
              Expanded(child: orchestratorPane),
              const SizedBox(height: 16),
              Expanded(child: workerPane),
            ],
          )
        : Row(
            children: [
              Expanded(child: orchestratorPane),
              const SizedBox(width: 16),
              Expanded(child: workerPane),
            ],
          );

    return Scaffold(
      appBar: AppBar(
        title: const Text('Robot Farm 🌾'),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Get.offAllNamed('/'),
        ),
        actions: [
          Obx(
            () => IconButton(
              tooltip: controller.isPlaying.value ? 'Pause' : 'Resume',
              icon: Icon(
                controller.isPlaying.value
                    ? Icons.pause_circle
                    : Icons.play_circle,
              ),
              onPressed: controller.togglePlayPause,
            ),
          ),
          PopupMenuButton<_HomeMenuAction>(
            tooltip: 'More actions',
            onSelected: (action) {
              switch (action) {
                case _HomeMenuAction.tasksView:
                  Get.toNamed('/tasks');
                  break;
                case _HomeMenuAction.gitStatuses:
                  Get.toNamed('/git-status');
                  break;
                case _HomeMenuAction.taskWizard:
                  Get.toNamed('/task-wizard');
                  break;
                case _HomeMenuAction.changeStrategy:
                  _openStrategySheet(context);
                  break;
                case _HomeMenuAction.clearFeeds:
                  controller.clearFeeds();
                  break;
              }
            },
            itemBuilder: (context) => const [
              PopupMenuItem(
                value: _HomeMenuAction.tasksView,
                child: ListTile(
                  leading: Icon(Icons.list_alt),
                  title: Text('Tasks View'),
                ),
              ),
              PopupMenuItem(
                value: _HomeMenuAction.gitStatuses,
                child: ListTile(
                  leading: Icon(Icons.code),
                  title: Text('Git Statuses'),
                ),
              ),
              PopupMenuItem(
                value: _HomeMenuAction.taskWizard,
                child: ListTile(
                  leading: Icon(Icons.auto_fix_high),
                  title: Text('Task Wizard View'),
                ),
              ),
              PopupMenuItem(
                value: _HomeMenuAction.changeStrategy,
                child: ListTile(
                  leading: Icon(Icons.tune),
                  title: Text('Change Strategy'),
                ),
              ),
              PopupMenuItem(
                value: _HomeMenuAction.clearFeeds,
                child: ListTile(
                  leading: Icon(Icons.delete_sweep),
                  title: Text('Clear Feeds'),
                ),
              ),
            ],
          ),
          IconButton(
            icon: const Icon(Icons.settings),
            tooltip: 'Settings',
            onPressed: () => Get.toNamed('/settings'),
          ),
        ],
      ),
      body: Column(
        children: [
          Expanded(
            child: Padding(padding: const EdgeInsets.all(24), child: child),
          ),
          _HomeStatusBar(controller: controller),
        ],
      ),
    );
  }
}

enum _HomeMenuAction {
  tasksView,
  gitStatuses,
  taskWizard,
  changeStrategy,
  clearFeeds,
}

class OrchestratorPane extends StatelessWidget {
  const OrchestratorPane({
    required this.connection,
    required this.onRunCommand,
    required this.onEnqueueMessage,
    required this.onEditQueue,
    super.key,
  });

  final ConnectionController connection;
  final VoidCallback onRunCommand;
  final VoidCallback onEnqueueMessage;
  final VoidCallback onEditQueue;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return DecoratedBox(
      decoration: BoxDecoration(
        border: Border.all(color: theme.colorScheme.outlineVariant),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: GetBuilder<OrchestratorFeedController>(
          init: OrchestratorFeedController(connection: connection),
          global: false,
          builder: (feedController) {
            final events = feedController.events;
            final Widget feedBody;
            if (feedController.isLoading) {
              feedBody = const Expanded(
                child: Center(child: CircularProgressIndicator()),
              );
            } else {
              feedBody = Expanded(
                child: _SystemFeed(
                  connection: connection,
                  events: events,
                  emptyMessage:
                      'Turn-by-turn orchestrator output will appear here.',
                ),
              );
            }

            return Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Row(
                  children: [
                    Text(
                      'Orchestrator Feed',
                      style: theme.textTheme.titleLarge,
                    ),
                    const Spacer(),
                    _FeedActionsMenu(
                      onRunCommand: onRunCommand,
                      onEnqueueMessage: onEnqueueMessage,
                      onEditQueue: onEditQueue,
                      onAddWorker: null,
                      onTerminate: () => connection.terminateOrchestrator(),
                      terminateLabel: 'Terminate orchestrator',
                    ),
                  ],
                ),
                const SizedBox(height: 12),
                feedBody,
              ],
            );
          },
        ),
      ),
    );
  }
}

class _SystemFeed extends StatelessWidget {
  const _SystemFeed({
    required this.connection,
    required this.events,
    this.emptyMessage,
  });

  final ConnectionController connection;
  final List<SystemFeedEvent> events;
  final String? emptyMessage;

  @override
  Widget build(BuildContext context) {
    if (events.isEmpty) {
      return Center(
        child: Text(
          emptyMessage ?? 'No feed entries yet.',
          textAlign: TextAlign.center,
        ),
      );
    }
    final radius = BorderRadius.circular(12);
    return ListView.separated(
      itemCount: events.length,
      separatorBuilder: (_, __) => const SizedBox(height: 8),
      itemBuilder: (context, index) {
        final event = events[index];
        final viewModel = _SystemEventViewModel.fromEvent(
          context,
          event,
          fullDetail: false,
        );

        return Card(
          margin: EdgeInsets.zero,
          child: InkWell(
            borderRadius: radius,
            onTap: () => _showEventDetails(context, event),
            child: Padding(
              padding: const EdgeInsets.all(12),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      CircleAvatar(
                        radius: 18,
                        backgroundColor: viewModel.color.withValues(
                          alpha: 0.15,
                        ),
                        child: Icon(viewModel.icon, color: viewModel.color),
                      ),
                      const SizedBox(width: 12),
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(
                              viewModel.title,
                              style: Theme.of(context).textTheme.titleMedium
                                  ?.copyWith(fontWeight: FontWeight.bold),
                            ),
                            if (viewModel.subtitle != null) ...[
                              const SizedBox(height: 4),
                              Text(
                                viewModel.subtitle!,
                                style: Theme.of(context).textTheme.bodyMedium,
                              ),
                            ],
                          ],
                        ),
                      ),
                    ],
                  ),
                  if (viewModel.body != null) ...[
                    const SizedBox(height: 8),
                    viewModel.body!,
                  ],
                ],
              ),
            ),
          ),
        );
      },
    );
  }

  void _showEventDetails(BuildContext context, SystemFeedEvent event) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (ctx) => FractionallySizedBox(
        heightFactor: 0.85,
        child: _FeedDetailSheet(connection: connection, event: event),
      ),
    );
  }
}

class _FeedDetailSheet extends StatefulWidget {
  const _FeedDetailSheet({required this.connection, required this.event});

  final ConnectionController connection;
  final SystemFeedEvent event;

  @override
  State<_FeedDetailSheet> createState() => _FeedDetailSheetState();
}

class _FeedDetailSheetState extends State<_FeedDetailSheet> {
  late String _details;
  bool _loading = false;
  String? _error;

  @override
  void initState() {
    super.initState();
    _details = widget.event.details;
    if (_shouldFetchDetails()) {
      _loadDetails();
    }
  }

  bool _shouldFetchDetails() {
    final feed = widget.event.feed;
    if (feed == null) {
      return false;
    }
    final trimmed = _details.trim();
    return trimmed.isEmpty || trimmed == SystemFeedEvent.noDetailsLabel;
  }

  Future<void> _loadDetails() async {
    final feed = widget.event.feed;
    if (feed == null) {
      return;
    }
    setState(() {
      _loading = true;
      _error = null;
    });
    try {
      final entry = await widget.connection.fetchFeedEntry(feed.id);
      if (entry != null) {
        final formatted = SystemFeedEvent.formatDetails(entry.raw);
        setState(() {
          _details = formatted;
        });
      } else {
        setState(() {
          _error = 'Feed entry not found.';
        });
      }
    } on robot_farm_api.ApiException catch (err) {
      setState(() {
        _error = err.message ?? 'Failed to load feed entry (${err.code}).';
      });
    } catch (err) {
      setState(() {
        _error = 'Failed to load feed entry: $err';
      });
    } finally {
      setState(() {
        _loading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final viewModel = _SystemEventViewModel.fromEvent(
      context,
      widget.event,
      fullDetail: true,
    );
    Widget body;
    if (_loading) {
      body = const Center(child: CircularProgressIndicator());
    } else if (_error != null) {
      body = Center(child: SelectionArea(child: Text(_error!)));
    } else if (_details.trim().isEmpty ||
        _details == SystemFeedEvent.noDetailsLabel) {
      body = const Center(
        child: SelectionArea(
          child: Text('No additional details for this event.'),
        ),
      );
    } else {
      body = SingleChildScrollView(
        child: SelectionArea(child: _OutputBubble(text: _details)),
      );
    }

    return SafeArea(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Row(
              children: [
                CircleAvatar(
                  radius: 24,
                  backgroundColor: viewModel.color.withValues(alpha: 0.15),
                  child: Icon(viewModel.icon, color: viewModel.color),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: SelectionArea(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          viewModel.title,
                          style: Theme.of(context).textTheme.titleLarge
                              ?.copyWith(fontWeight: FontWeight.bold),
                        ),
                        if (viewModel.subtitle != null) ...[
                          const SizedBox(height: 4),
                          Text(
                            viewModel.subtitle!,
                            style: Theme.of(context).textTheme.bodyMedium,
                          ),
                        ],
                      ],
                    ),
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.close),
                  onPressed: () => Navigator.of(context).maybePop(),
                ),
              ],
            ),
            const SizedBox(height: 16),
            Expanded(child: body),
          ],
        ),
      ),
    );
  }
}

class _SystemEventViewModel {
  _SystemEventViewModel({
    required this.title,
    this.subtitle,
    this.body,
    required this.icon,
    required this.color,
  });

  final String title;
  final String? subtitle;
  final Widget? body;
  final IconData icon;
  final Color color;

  factory _SystemEventViewModel.fromEvent(
    BuildContext context,
    SystemFeedEvent event, {
    required bool fullDetail,
  }) {
    final scheme = Theme.of(context).colorScheme;
    final color = event.badgeColor(scheme);
    final icon = event.iconForCategory();

    Widget? body;
    final subtitle =
        'Source: ${event.source} • Target: ${event.target} • ${event.category}';

    if (fullDetail) {
      body = _OutputBubble(text: event.details);
    } else if (event.details.length > 160) {
      body = _OutputBubble(text: event.details, maxLines: 4);
    }

    return _SystemEventViewModel(
      title: event.summary,
      subtitle: subtitle,
      body: body,
      icon: icon,
      color: color,
    );
  }
}

class _OutputBubble extends StatelessWidget {
  const _OutputBubble({required this.text, this.maxLines});

  final String text;
  final int? maxLines;

  @override
  Widget build(BuildContext context) {
    return Container(
      width: double.infinity,
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surfaceContainerHighest,
        borderRadius: BorderRadius.circular(8),
      ),
      padding: const EdgeInsets.all(12),
      child: Text(
        text,
        style: Theme.of(
          context,
        ).textTheme.bodySmall?.copyWith(fontFamily: 'monospace'),
        maxLines: maxLines,
        overflow: maxLines == null
            ? TextOverflow.visible
            : TextOverflow.ellipsis,
      ),
    );
  }
}

class WorkerFeedPane extends StatelessWidget {
  const WorkerFeedPane({
    required this.connection,
    required this.onRunCommand,
    required this.onEnqueueMessage,
    required this.onEditQueue,
    required this.onAddWorker,
    super.key,
  });

  final ConnectionController connection;
  final void Function(int workerId) onRunCommand;
  final void Function(int workerId) onEnqueueMessage;
  final void Function(int workerId) onEditQueue;
  final VoidCallback onAddWorker;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return GetBuilder<WorkerFeedController>(
      init: WorkerFeedController(connection: connection),
      global: false,
      builder: (controller) {
        final onWorkerRun = controller.activeWorkerId == null
            ? null
            : () => onRunCommand(controller.activeWorkerId!);
        final onWorkerMessage = controller.activeWorkerId == null
            ? null
            : () => onEnqueueMessage(controller.activeWorkerId!);
        final onWorkerQueue = controller.activeWorkerId == null
            ? null
            : () => onEditQueue(controller.activeWorkerId!);
        final onWorkerTerminate = controller.activeWorkerId == null
            ? null
            : () => controller.terminateWorker(controller.activeWorkerId!);

        final workers = controller.connection.workers.toList();
        final tabs = workers.isEmpty
            ? const [Tab(text: 'Workers')]
            : workers.map((w) => Tab(text: 'Worker ${w.id}')).toList();

        return DecoratedBox(
          decoration: BoxDecoration(
            border: Border.all(color: theme.colorScheme.outlineVariant),
            borderRadius: BorderRadius.circular(12),
          ),
          child: Padding(
            padding: const EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Row(
                  children: [
                    Text('Worker Feeds', style: theme.textTheme.titleLarge),
                    const Spacer(),
                    _FeedActionsMenu(
                      onRunCommand: onWorkerRun,
                      onEnqueueMessage: onWorkerMessage,
                      onEditQueue: onWorkerQueue,
                      onAddWorker: onAddWorker,
                      onTerminate: onWorkerTerminate,
                      terminateLabel: 'Terminate worker',
                    ),
                  ],
                ),
                const SizedBox(height: 12),
                if (!controller.isReady)
                  const Expanded(
                    child: Center(child: CircularProgressIndicator()),
                  )
                else
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.stretch,
                      children: [
                        TabBar(
                          controller: controller.tabController,
                          isScrollable: true,
                          tabs: tabs,
                        ),
                        const SizedBox(height: 12),
                        Expanded(
                          child: TabBarView(
                            controller: controller.tabController,
                            children: workers.isEmpty
                                ? const [
                                    Center(
                                      child: Text(
                                        'No workers detected. Add a git worktree to see it here.',
                                        textAlign: TextAlign.center,
                                      ),
                                    ),
                                  ]
                                : workers.map((worker) {
                                    final threadStyle = theme
                                        .textTheme
                                        .bodySmall
                                        ?.copyWith(fontFamily: 'RobotoMono');
                                    final threadValue = worker.threadId?.trim();
                                    final threadLabel =
                                        (threadValue == null ||
                                            threadValue.isEmpty)
                                        ? 'Thread ID: not started'
                                        : 'Thread ID: ${worker.threadId}';
                                    return Column(
                                      crossAxisAlignment:
                                          CrossAxisAlignment.stretch,
                                      children: [
                                        SelectableText(
                                          threadLabel,
                                          style: threadStyle,
                                        ),
                                        const SizedBox(height: 8),
                                        Expanded(
                                          child: _SystemFeed(
                                            connection: controller.connection,
                                            events: controller.eventsForWorker(
                                              worker.id,
                                            ),
                                            emptyMessage:
                                                'No feed entries yet for worker ${worker.id}.',
                                          ),
                                        ),
                                      ],
                                    );
                                  }).toList(),
                          ),
                        ),
                      ],
                    ),
                  ),
              ],
            ),
          ),
        );
      },
    );
  }
}

class OrchestratorFeedController extends GetxController {
  OrchestratorFeedController({required this.connection});

  final ConnectionController connection;
  final List<SystemFeedEvent> events = <SystemFeedEvent>[];
  bool isLoading = true;
  StreamSubscription<robot_farm_api.Feed>? _subscription;
  StreamSubscription<void>? _clearedSubscription;

  @override
  void onInit() {
    super.onInit();
    _loadInitial();
    _subscription = connection.feedEvents.listen(_handleFeedEntry);
    _clearedSubscription = connection.feedCleared.listen(
      (_) => _handleFeedCleared(),
    );
  }

  Future<void> _loadInitial() async {
    isLoading = true;
    update();
    final api = _buildApi();
    if (api == null) {
      isLoading = false;
      update();
      return;
    }
    try {
      final feed = await api.listFeed(target: 'Orchestrator');
      events
        ..clear()
        ..addAll(
          (feed ?? const <robot_farm_api.Feed>[]).map(SystemFeedEvent.fromFeed),
        );
    } catch (error) {
      debugPrint('Failed to load orchestrator feed: $error');
    } finally {
      isLoading = false;
      update();
    }
  }

  void _handleFeedEntry(robot_farm_api.Feed feed) {
    if (feed.target.toLowerCase() != 'orchestrator') {
      return;
    }
    events.insert(0, SystemFeedEvent.fromFeed(feed));
    _trim();
    update();
  }

  void _trim() {
    const maxEntries = 200;
    if (events.length > maxEntries) {
      events.removeRange(maxEntries, events.length);
    }
  }

  robot_farm_api.DefaultApi? _buildApi() {
    final baseUrl = connection.currentBaseUrl;
    if (baseUrl == null) {
      return null;
    }
    final client = robot_farm_api.ApiClient(basePath: baseUrl);
    return robot_farm_api.DefaultApi(client);
  }

  @override
  void onClose() {
    _subscription?.cancel();
    _clearedSubscription?.cancel();
    super.onClose();
  }

  void _handleFeedCleared() {
    events.clear();
    update();
  }
}

class WorkerFeedController extends GetxController
    with GetTickerProviderStateMixin {
  WorkerFeedController({required this.connection});

  final ConnectionController connection;
  late TabController tabController;
  bool isReady = false;
  final Map<int, List<SystemFeedEvent>> _workerEvents =
      <int, List<SystemFeedEvent>>{};
  StreamSubscription<robot_farm_api.Feed>? _feedSubscription;
  StreamSubscription<void>? _feedClearedSubscription;

  @override
  void onInit() {
    super.onInit();
    tabController = TabController(length: _length, vsync: this);
    ever(connection.workers, (_) => _syncTabs());
    _loadInitialFeeds();
    _feedSubscription = connection.feedEvents.listen(_handleFeedEntry);
    _feedClearedSubscription = connection.feedCleared.listen(
      (_) => _handleFeedCleared(),
    );
  }

  int get _length => connection.workers.isEmpty ? 1 : connection.workers.length;

  int? get activeWorkerId {
    if (connection.workers.isEmpty) {
      return null;
    }
    final index = tabController.index.clamp(0, connection.workers.length - 1);
    return connection.workers[index].id;
  }

  Future<void> terminateWorker(int workerId) async {
    await connection.terminateWorker(workerId);
  }

  void _syncTabs() {
    final newLength = _length;
    if (tabController.length == newLength) {
      update();
      return;
    }
    final previousIndex = tabController.index;
    tabController.dispose();
    tabController = TabController(length: newLength, vsync: this);
    if (newLength > 0) {
      tabController.index = previousIndex.clamp(0, newLength - 1);
    }
    update();
  }

  List<SystemFeedEvent> eventsForWorker(int workerId) {
    return _workerEvents[workerId] ?? const <SystemFeedEvent>[];
  }

  Future<void> _loadInitialFeeds() async {
    isReady = false;
    update();
    final baseUrl = connection.currentBaseUrl;
    if (baseUrl == null) {
      isReady = true;
      update();
      return;
    }
    try {
      final client = robot_farm_api.ApiClient(basePath: baseUrl);
      final api = robot_farm_api.DefaultApi(client);
      final feed = await api.listFeed();
      _workerEvents.clear();
      for (final entry in feed ?? const <robot_farm_api.Feed>[]) {
        final workerId = _workerIdFromTarget(entry.target);
        if (workerId == null) {
          continue;
        }
        final bucket = _workerEvents.putIfAbsent(
          workerId,
          () => <SystemFeedEvent>[],
        );
        bucket.add(SystemFeedEvent.fromFeed(entry));
      }
      for (final bucket in _workerEvents.values) {
        bucket.sort((a, b) => b.timestamp.compareTo(a.timestamp));
      }
    } catch (error) {
      debugPrint('Failed to load worker feeds: $error');
    } finally {
      isReady = true;
      update();
    }
  }

  void _handleFeedEntry(robot_farm_api.Feed feed) {
    final workerId = _workerIdFromTarget(feed.target);
    if (workerId == null) {
      return;
    }
    final bucket = _workerEvents.putIfAbsent(
      workerId,
      () => <SystemFeedEvent>[],
    );
    bucket.insert(0, SystemFeedEvent.fromFeed(feed));
    const maxEntries = 200;
    if (bucket.length > maxEntries) {
      bucket.removeRange(maxEntries, bucket.length);
    }
    update();
  }

  int? _workerIdFromTarget(String target) {
    final lower = target.toLowerCase();
    if (!lower.startsWith('ws')) {
      return null;
    }
    return int.tryParse(lower.substring(2));
  }

  @override
  void onClose() {
    tabController.dispose();
    _feedSubscription?.cancel();
    _feedClearedSubscription?.cancel();
    super.onClose();
  }

  void _handleFeedCleared() {
    for (final bucket in _workerEvents.values) {
      bucket.clear();
    }
    update();
  }
}

enum CodexPersona { orchestrator, worker, wizard }

extension CodexPersonaX on CodexPersona {
  String get label {
    switch (this) {
      case CodexPersona.orchestrator:
        return 'Orchestrator';
      case CodexPersona.worker:
        return 'Worker';
      case CodexPersona.wizard:
        return 'Wizard';
    }
  }
}

class SettingsController extends GetxController {
  SettingsController(this._connection);

  final ConnectionController _connection;
  final Rx<robot_farm_api.Config?> config = Rx<robot_farm_api.Config?>(null);
  final RxBool isLoading = false.obs;
  final RxnString error = RxnString();
  final TextEditingController orchestratorController = TextEditingController();
  final TextEditingController workerController = TextEditingController();
  final TextEditingController orchestratorDockerController =
      TextEditingController();
  final TextEditingController workerDockerController = TextEditingController();
  final TextEditingController wizardDockerController = TextEditingController();
  static const String _modelCodex = 'gpt-5.1-codex';
  static const String _modelCodexMini = 'gpt-5.1-codex-mini';
  static const String _modelGpt51 = 'gpt-5.1';
  static const String _defaultReasoning = 'medium';
  static const List<String> modelOptions = <String>[
    _modelCodex,
    _modelCodexMini,
    _modelGpt51,
  ];
  static const List<String> _reasoningLevels = <String>[
    'low',
    'medium',
    'high',
  ];
  static const List<String> _reasoningMediumHigh = <String>['medium', 'high'];

  List<robot_farm_api.CommandConfig> get commands =>
      List<robot_farm_api.CommandConfig>.unmodifiable(
        config.value?.commands ?? const <robot_farm_api.CommandConfig>[],
      );

  List<String> get postTurnChecks => List<String>.unmodifiable(
    config.value?.postTurnChecks ?? const <String>[],
  );

  String modelFor(CodexPersona persona) {
    final models = _currentModels();
    switch (persona) {
      case CodexPersona.orchestrator:
        return models.orchestrator.value;
      case CodexPersona.worker:
        return models.worker.value;
      case CodexPersona.wizard:
        return models.wizard.value;
    }
  }

  TextEditingController dockerControllerFor(CodexPersona persona) {
    switch (persona) {
      case CodexPersona.orchestrator:
        return orchestratorDockerController;
      case CodexPersona.worker:
        return workerDockerController;
      case CodexPersona.wizard:
        return wizardDockerController;
    }
  }

  String reasoningFor(CodexPersona persona) {
    final reasoning = _currentReasoning();
    switch (persona) {
      case CodexPersona.orchestrator:
        return reasoning.orchestrator.value;
      case CodexPersona.worker:
        return reasoning.worker.value;
      case CodexPersona.wizard:
        return reasoning.wizard.value;
    }
  }

  List<String> reasoningOptionsFor(CodexPersona persona) {
    final model = modelFor(persona);
    return model == _modelCodexMini ? _reasoningMediumHigh : _reasoningLevels;
  }

  @override
  void onReady() {
    super.onReady();
    loadConfig();
  }

  Future<void> loadConfig() async {
    final baseUrl = _connection.currentBaseUrl;
    if (baseUrl == null) {
      error.value = 'Not connected to a server.';
      return;
    }

    isLoading.value = true;
    error.value = null;
    try {
      final client = robot_farm_api.ApiClient(basePath: baseUrl);
      final api = robot_farm_api.DefaultApi(client);
      final result = await api.getConfig();
      config.value = result;
      if (result != null) {
        orchestratorController.text = result.appendAgentsFile.orchestrator.join(
          ', ',
        );
        workerController.text = result.appendAgentsFile.worker.join(', ');
        orchestratorDockerController.text =
            result.dockerOverrides.orchestrator.join('\n');
        workerDockerController.text =
            result.dockerOverrides.worker.join('\n');
        wizardDockerController.text =
            result.dockerOverrides.wizard.join('\n');
      }
    } on robot_farm_api.ApiException catch (err) {
      error.value = err.message ?? 'Failed to load config: ${err.code}';
    } catch (err) {
      error.value = 'Failed to load config: $err';
    } finally {
      isLoading.value = false;
    }
  }

  Future<void> saveConfig() async {
    final current = config.value;
    if (current == null) return;
    final baseUrl = _connection.currentBaseUrl;
    if (baseUrl == null) {
      error.value = 'Not connected to a server.';
      return;
    }

    isLoading.value = true;
    error.value = null;
    try {
      final client = robot_farm_api.ApiClient(basePath: baseUrl);
      final api = robot_farm_api.DefaultApi(client);
      await api.updateConfig(current);
    } on robot_farm_api.ApiException catch (err) {
      error.value = err.message ?? 'Failed to save config: ${err.code}';
    } catch (err) {
      error.value = 'Failed to save config: $err';
    } finally {
      isLoading.value = false;
    }
  }

  void updateOrchestratorPaths(String value) {
    final current = config.value;
    if (current == null) return;
    final updated = robot_farm_api.AppendFilesConfig(
      orchestrator: _splitPaths(value),
      worker: List<String>.from(current.appendAgentsFile.worker),
    );
    _assignConfig(appendAgentsFile: updated);
  }

  void updateWorkerPaths(String value) {
    final current = config.value;
    if (current == null) return;
    final updated = robot_farm_api.AppendFilesConfig(
      orchestrator: List<String>.from(current.appendAgentsFile.orchestrator),
      worker: _splitPaths(value),
    );
    _assignConfig(appendAgentsFile: updated);
  }

  void updateDockerOverrides(CodexPersona persona, String value) {
    final current = config.value;
    if (current == null) return;
    final args = value
        .split('\n')
        .map((line) => line.trim())
        .where((line) => line.isNotEmpty)
        .toList();
    final overrides = robot_farm_api.DockerOverrides(
      orchestrator: persona == CodexPersona.orchestrator
          ? args
          : List<String>.from(current.dockerOverrides.orchestrator),
      worker: persona == CodexPersona.worker
          ? args
          : List<String>.from(current.dockerOverrides.worker),
      wizard: persona == CodexPersona.wizard
          ? args
          : List<String>.from(current.dockerOverrides.wizard),
    );
    _assignConfig(dockerOverrides: overrides);
  }

  void updateModel(CodexPersona persona, String value) {
    if (config.value == null) return;
    final updatedModels = _modelsWith(
      orchestrator: persona == CodexPersona.orchestrator
          ? _parseOrchestratorModel(value)
          : null,
      worker: persona == CodexPersona.worker ? _parseWorkerModel(value) : null,
      wizard: persona == CodexPersona.wizard ? _parseWizardModel(value) : null,
    );
    final adjustedReasoning = _ensureReasoningCompatibility(
      persona,
      updatedModels,
      _currentReasoning(),
    );
    _assignConfig(models: updatedModels, reasoning: adjustedReasoning);
  }

  void updateReasoning(CodexPersona persona, String value) {
    if (config.value == null) return;
    if (_modelDisallowsLow(modelFor(persona)) && value == 'low') {
      return;
    }
    final effort = _parseReasoning(value);
    final updatedReasoning = _reasoningWith(
      orchestrator: persona == CodexPersona.orchestrator ? effort : null,
      worker: persona == CodexPersona.worker ? effort : null,
      wizard: persona == CodexPersona.wizard ? effort : null,
    );
    _assignConfig(reasoning: updatedReasoning);
  }

  bool isCommandSelected(String id) => postTurnChecks.contains(id);

  void toggleCommandSelection(String id, bool selected) {
    final current = config.value;
    if (current == null) return;
    final updated = List<String>.from(current.postTurnChecks);
    if (selected) {
      if (!updated.contains(id)) {
        updated.add(id);
      }
    } else {
      updated.removeWhere((entry) => entry == id);
    }
    _assignConfig(postTurnChecks: updated);
  }

  void reorderPostChecks(int oldIndex, int newIndex) {
    final current = config.value;
    if (current == null) return;
    final updated = List<String>.from(current.postTurnChecks);
    if (newIndex > oldIndex) newIndex -= 1;
    final item = updated.removeAt(oldIndex);
    updated.insert(newIndex, item);
    _assignConfig(postTurnChecks: updated);
  }

  String? addCommand(robot_farm_api.CommandConfig command) {
    final current = config.value;
    if (current == null) return 'Configuration not loaded.';
    if (current.commands.any((c) => c.id == command.id)) {
      error.value = 'Command "${command.id}" already exists.';
      return error.value;
    }
    final updatedCommands = List<robot_farm_api.CommandConfig>.from(
      current.commands,
    )..add(command);
    _assignConfig(commands: updatedCommands);
    error.value = null;
    return null;
  }

  String? updateCommand(
    String originalId,
    robot_farm_api.CommandConfig updated,
  ) {
    final current = config.value;
    if (current == null) return 'Configuration not loaded.';
    final updatedCommands = List<robot_farm_api.CommandConfig>.from(
      current.commands,
    );
    final index = updatedCommands.indexWhere(
      (command) => command.id == originalId,
    );
    if (index == -1) {
      error.value = 'Command "$originalId" not found.';
      return error.value;
    }
    if (originalId != updated.id &&
        updatedCommands.any((command) => command.id == updated.id)) {
      error.value = 'Command "${updated.id}" already exists.';
      return error.value;
    }
    updatedCommands[index] = updated;

    final updatedChecks = List<String>.from(current.postTurnChecks);
    if (originalId != updated.id) {
      for (var i = 0; i < updatedChecks.length; i++) {
        if (updatedChecks[i] == originalId) {
          updatedChecks[i] = updated.id;
        }
      }
    }

    _assignConfig(commands: updatedCommands, postTurnChecks: updatedChecks);
    error.value = null;
    return null;
  }

  void removeCommand(String id) {
    final current = config.value;
    if (current == null) return;
    final updatedCommands = List<robot_farm_api.CommandConfig>.from(
      current.commands,
    )..removeWhere((command) => command.id == id);
    final updatedChecks = List<String>.from(current.postTurnChecks)
      ..removeWhere((entry) => entry == id);
    _assignConfig(commands: updatedCommands, postTurnChecks: updatedChecks);
  }

  robot_farm_api.CommandConfig? commandById(String id) {
    return config.value?.commands.firstWhereOrNull(
      (command) => command.id == id,
    );
  }

  List<String> _splitPaths(String value) => value
      .split(',')
      .map((path) => path.trim())
      .where((path) => path.isNotEmpty)
      .toList();

  void _assignConfig({
    robot_farm_api.AppendFilesConfig? appendAgentsFile,
    List<robot_farm_api.CommandConfig>? commands,
    List<String>? postTurnChecks,
    robot_farm_api.AgentModelOverrides? models,
    robot_farm_api.AgentReasoningOverrides? reasoning,
    robot_farm_api.DockerOverrides? dockerOverrides,
  }) {
    final current = config.value;
    if (current == null) return;
    final append =
        appendAgentsFile ??
        robot_farm_api.AppendFilesConfig(
          orchestrator: List<String>.from(
            current.appendAgentsFile.orchestrator,
          ),
          worker: List<String>.from(current.appendAgentsFile.worker),
        );
    final appliedModels =
        models ??
        robot_farm_api.AgentModelOverrides(
          orchestrator: current.models.orchestrator,
          worker: current.models.worker,
          wizard: current.models.wizard,
        );
    final appliedReasoning =
        reasoning ??
        robot_farm_api.AgentReasoningOverrides(
          orchestrator: current.reasoning.orchestrator,
          worker: current.reasoning.worker,
          wizard: current.reasoning.wizard,
        );
    final appliedDockerOverrides =
        dockerOverrides ??
        robot_farm_api.DockerOverrides(
          orchestrator: List<String>.from(current.dockerOverrides.orchestrator),
          worker: List<String>.from(current.dockerOverrides.worker),
          wizard: List<String>.from(current.dockerOverrides.wizard),
        );
    final cmds =
        commands ?? List<robot_farm_api.CommandConfig>.from(current.commands);
    final checks = postTurnChecks ?? List<String>.from(current.postTurnChecks);
    config.value = robot_farm_api.Config(
      appendAgentsFile: append,
      models: appliedModels,
      reasoning: appliedReasoning,
      commands: cmds,
      postTurnChecks: checks,
      dockerOverrides: appliedDockerOverrides,
    );
    config.refresh();
  }

  robot_farm_api.AgentModelOverrides _currentModels() {
    final current = config.value?.models;
    if (current == null) {
      return _defaultModels();
    }
    return robot_farm_api.AgentModelOverrides(
      orchestrator: current.orchestrator,
      worker: current.worker,
      wizard: current.wizard,
    );
  }

  robot_farm_api.AgentReasoningOverrides _currentReasoning() {
    final current = config.value?.reasoning;
    if (current == null) {
      return _defaultReasoningConfig();
    }
    return robot_farm_api.AgentReasoningOverrides(
      orchestrator: current.orchestrator,
      worker: current.worker,
      wizard: current.wizard,
    );
  }

  robot_farm_api.AgentModelOverrides _defaultModels() =>
      robot_farm_api.AgentModelOverrides(
        orchestrator:
            robot_farm_api.AgentModelOverridesOrchestratorEnum.gpt5Period1Codex,
        worker: robot_farm_api.AgentModelOverridesWorkerEnum.gpt5Period1Codex,
        wizard: robot_farm_api.AgentModelOverridesWizardEnum.gpt5Period1Codex,
      );

  robot_farm_api.AgentReasoningOverrides _defaultReasoningConfig() =>
      robot_farm_api.AgentReasoningOverrides(
        orchestrator: robot_farm_api.ReasoningEffort.medium,
        worker: robot_farm_api.ReasoningEffort.medium,
        wizard: robot_farm_api.ReasoningEffort.medium,
      );

  robot_farm_api.AgentModelOverrides _modelsWith({
    robot_farm_api.AgentModelOverridesOrchestratorEnum? orchestrator,
    robot_farm_api.AgentModelOverridesWorkerEnum? worker,
    robot_farm_api.AgentModelOverridesWizardEnum? wizard,
    robot_farm_api.AgentModelOverrides? existing,
  }) {
    final current = existing ?? _currentModels();
    return robot_farm_api.AgentModelOverrides(
      orchestrator: orchestrator ?? current.orchestrator,
      worker: worker ?? current.worker,
      wizard: wizard ?? current.wizard,
    );
  }

  robot_farm_api.AgentReasoningOverrides _reasoningWith({
    robot_farm_api.ReasoningEffort? orchestrator,
    robot_farm_api.ReasoningEffort? worker,
    robot_farm_api.ReasoningEffort? wizard,
    robot_farm_api.AgentReasoningOverrides? existing,
  }) {
    final current = existing ?? _currentReasoning();
    return robot_farm_api.AgentReasoningOverrides(
      orchestrator: orchestrator ?? current.orchestrator,
      worker: worker ?? current.worker,
      wizard: wizard ?? current.wizard,
    );
  }

  robot_farm_api.AgentModelOverridesOrchestratorEnum _parseOrchestratorModel(
    String value,
  ) {
    final transformer =
        robot_farm_api.AgentModelOverridesOrchestratorEnumTypeTransformer();
    return transformer.decode(value, allowNull: false)!;
  }

  robot_farm_api.AgentModelOverridesWorkerEnum _parseWorkerModel(String value) {
    final transformer =
        robot_farm_api.AgentModelOverridesWorkerEnumTypeTransformer();
    return transformer.decode(value, allowNull: false)!;
  }

  robot_farm_api.AgentModelOverridesWizardEnum _parseWizardModel(String value) {
    final transformer =
        robot_farm_api.AgentModelOverridesWizardEnumTypeTransformer();
    return transformer.decode(value, allowNull: false)!;
  }

  robot_farm_api.ReasoningEffort _parseReasoning(String value) {
    final transformer = robot_farm_api.ReasoningEffortTypeTransformer();
    return transformer.decode(value, allowNull: false)!;
  }

  bool _modelDisallowsLow(String model) => model == _modelCodexMini;

  robot_farm_api.AgentReasoningOverrides _ensureReasoningCompatibility(
    CodexPersona persona,
    robot_farm_api.AgentModelOverrides models,
    robot_farm_api.AgentReasoningOverrides reasoning,
  ) {
    if (!_modelDisallowsLow(_modelValueFrom(persona, models))) {
      return reasoning;
    }
    if (_reasoningValueFrom(persona, reasoning) != 'low') {
      return reasoning;
    }
    final fallback = _parseReasoning(_defaultReasoning);
    switch (persona) {
      case CodexPersona.orchestrator:
        return _reasoningWith(orchestrator: fallback, existing: reasoning);
      case CodexPersona.worker:
        return _reasoningWith(worker: fallback, existing: reasoning);
      case CodexPersona.wizard:
        return _reasoningWith(wizard: fallback, existing: reasoning);
    }
  }

  String _modelValueFrom(
    CodexPersona persona,
    robot_farm_api.AgentModelOverrides models,
  ) {
    switch (persona) {
      case CodexPersona.orchestrator:
        return models.orchestrator.value;
      case CodexPersona.worker:
        return models.worker.value;
      case CodexPersona.wizard:
        return models.wizard.value;
    }
  }

  String _reasoningValueFrom(
    CodexPersona persona,
    robot_farm_api.AgentReasoningOverrides reasoning,
  ) {
    switch (persona) {
      case CodexPersona.orchestrator:
        return reasoning.orchestrator.value;
      case CodexPersona.worker:
        return reasoning.worker.value;
      case CodexPersona.wizard:
        return reasoning.wizard.value;
    }
  }

  @override
  void onClose() {
    orchestratorController.dispose();
    workerController.dispose();
    super.onClose();
  }
}

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final connection = Get.find<ConnectionController>();
    final controller = Get.isRegistered<SettingsController>()
        ? Get.find<SettingsController>()
        : Get.put(SettingsController(connection));

    return Scaffold(
      appBar: AppBar(title: const Text('Settings')),
      body: Obx(() {
        if (controller.isLoading.value) {
          return const Center(child: CircularProgressIndicator());
        }

        if (controller.error.value != null) {
          return Center(
            child: Text(
              controller.error.value!,
              style: Theme.of(context).textTheme.bodyLarge?.copyWith(
                color: Theme.of(context).colorScheme.error,
              ),
            ),
          );
        }

        final config = controller.config.value;
        if (config == null) {
          return const Center(child: Text('No configuration loaded.'));
        }

        final theme = Theme.of(context);

        return Padding(
          padding: const EdgeInsets.all(16),
          child: ListView(
            children: [
              Text('Append Agent Files', style: theme.textTheme.titleLarge),
              const SizedBox(height: 12),
              TextField(
                controller: controller.orchestratorController,
                decoration: const InputDecoration(
                  labelText: 'Orchestrator files (comma separated)',
                ),
                onChanged: controller.updateOrchestratorPaths,
              ),
              const SizedBox(height: 12),
              TextField(
                controller: controller.workerController,
                decoration: const InputDecoration(
                  labelText: 'Worker files (comma separated)',
                ),
                onChanged: controller.updateWorkerPaths,
              ),
              const SizedBox(height: 24),
              Text('Docker Overrides', style: theme.textTheme.titleLarge),
              const SizedBox(height: 12),
              ...CodexPersona.values.map((persona) {
                final dockerController =
                    controller.dockerControllerFor(persona);
                return Padding(
                  padding: const EdgeInsets.only(bottom: 12),
                  child: TextField(
                    controller: dockerController,
                    minLines: 2,
                    maxLines: null,
                    decoration: InputDecoration(
                      labelText: '${persona.label} docker args (one per line)',
                      helperText:
                          'Each line becomes a separate argument inserted before the docker image.',
                    ),
                    keyboardType: TextInputType.multiline,
                    onChanged: (value) =>
                        controller.updateDockerOverrides(persona, value),
                  ),
                );
              }),
              const SizedBox(height: 24),
              Text(
                'Codex Models & Reasoning',
                style: theme.textTheme.titleLarge,
              ),
              const SizedBox(height: 12),
              ...CodexPersona.values.map((persona) {
                final modelValue = controller.modelFor(persona);
                final reasoningValue = controller.reasoningFor(persona);
                final reasoningOptions = controller.reasoningOptionsFor(
                  persona,
                );
                return Card(
                  child: Padding(
                    padding: const EdgeInsets.all(12),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(persona.label, style: theme.textTheme.titleMedium),
                        const SizedBox(height: 8),
                        DropdownButtonFormField<String>(
                          initialValue: modelValue,
                          decoration: const InputDecoration(labelText: 'Model'),
                          items: SettingsController.modelOptions
                              .map(
                                (model) => DropdownMenuItem(
                                  value: model,
                                  child: Text(model),
                                ),
                              )
                              .toList(),
                          onChanged: (value) {
                            if (value != null) {
                              controller.updateModel(persona, value);
                            }
                          },
                        ),
                        const SizedBox(height: 8),
                        DropdownButtonFormField<String>(
                          initialValue: reasoningValue,
                          decoration: const InputDecoration(
                            labelText: 'Reasoning effort',
                          ),
                          items: reasoningOptions
                              .map(
                                (level) => DropdownMenuItem(
                                  value: level,
                                  child: Text(_formatReasoningLabel(level)),
                                ),
                              )
                              .toList(),
                          onChanged: (value) {
                            if (value != null) {
                              controller.updateReasoning(persona, value);
                            }
                          },
                        ),
                      ],
                    ),
                  ),
                );
              }),
              const SizedBox(height: 24),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text('Commands', style: theme.textTheme.titleLarge),
                  FilledButton.icon(
                    onPressed: () => _openCommandEditor(context, controller),
                    icon: const Icon(Icons.add),
                    label: const Text('Add'),
                  ),
                ],
              ),
              const SizedBox(height: 12),
              if (controller.commands.isEmpty)
                const Text('No commands defined yet.')
              else
                ...controller.commands.map(
                  (command) => Card(
                    child: Padding(
                      padding: const EdgeInsets.symmetric(
                        horizontal: 8,
                        vertical: 4,
                      ),
                      child: Row(
                        children: [
                          Checkbox(
                            value: controller.isCommandSelected(command.id),
                            onChanged: (checked) =>
                                controller.toggleCommandSelection(
                                  command.id,
                                  checked ?? false,
                                ),
                          ),
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Text(
                                  command.id,
                                  style: theme.textTheme.titleMedium,
                                ),
                                const SizedBox(height: 4),
                                Text(
                                  command.exec.join(' '),
                                  style: theme.textTheme.bodySmall,
                                ),
                              ],
                            ),
                          ),
                          IconButton(
                            tooltip: 'Edit command',
                            icon: const Icon(Icons.edit),
                            onPressed: () => _openCommandEditor(
                              context,
                              controller,
                              command: command,
                            ),
                          ),
                          IconButton(
                            tooltip: 'Remove command',
                            icon: const Icon(Icons.delete_outline),
                            onPressed: () =>
                                controller.removeCommand(command.id),
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              const SizedBox(height: 24),
              Text('Post-turn checks order', style: theme.textTheme.titleLarge),
              const SizedBox(height: 8),
              if (controller.postTurnChecks.isEmpty)
                const Text(
                  'Select commands above to include them in post-turn checks.',
                )
              else
                SizedBox(
                  height: math.min(
                    360,
                    controller.postTurnChecks.length * 70.0 + 24,
                  ),
                  child: ReorderableListView.builder(
                    itemCount: controller.postTurnChecks.length,
                    shrinkWrap: true,
                    physics: const NeverScrollableScrollPhysics(),
                    onReorder: controller.reorderPostChecks,
                    itemBuilder: (context, index) {
                      final id = controller.postTurnChecks[index];
                      final command = controller.commandById(id);
                      return ListTile(
                        key: ValueKey(id),
                        title: Text(id),
                        subtitle: Text(
                          command?.exec.join(' ') ?? 'Command missing',
                        ),
                        trailing: const Icon(Icons.drag_handle),
                      );
                    },
                  ),
                ),
              const SizedBox(height: 24),
              FilledButton.icon(
                onPressed: controller.saveConfig,
                icon: const Icon(Icons.save),
                label: const Text('Save Config'),
              ),
            ],
          ),
        );
      }),
    );
  }

  Future<void> _openCommandEditor(
    BuildContext context,
    SettingsController controller, {
    robot_farm_api.CommandConfig? command,
  }) async {
    final result = await showModalBottomSheet<robot_farm_api.CommandConfig>(
      context: context,
      isScrollControlled: true,
      builder: (_) => CommandEditorSheet(initial: command),
    );

    if (result == null) {
      return;
    }

    if (command == null) {
      controller.addCommand(result);
    } else {
      controller.updateCommand(command.id, result);
    }
  }
}

String _formatReasoningLabel(String value) {
  if (value.isEmpty) return value;
  return '${value[0].toUpperCase()}${value.substring(1)}';
}

class CommandEditorSheet extends StatefulWidget {
  const CommandEditorSheet({super.key, this.initial});

  final robot_farm_api.CommandConfig? initial;

  @override
  State<CommandEditorSheet> createState() => _CommandEditorSheetState();
}

class _CommandEditorSheetState extends State<CommandEditorSheet> {
  late final TextEditingController _idController;
  late final TextEditingController _execController;
  late final TextEditingController _stdoutController;
  late final TextEditingController _timeoutController;
  late final TextEditingController _cwdController;
  bool _hidden = false;
  String? _error;

  @override
  void initState() {
    super.initState();
    final initial = widget.initial;
    _idController = TextEditingController(text: initial?.id ?? '');
    _execController = TextEditingController(
      text: initial?.exec.join('\n') ?? '',
    );
    _stdoutController = TextEditingController(
      text: initial?.stdoutSuccessMessage ?? '',
    );
    _timeoutController = TextEditingController(
      text: initial?.timeoutSeconds?.toString() ?? '',
    );
    _cwdController = TextEditingController(text: initial?.cwd ?? '');
    _hidden = initial?.hidden ?? false;
  }

  @override
  void dispose() {
    _idController.dispose();
    _execController.dispose();
    _stdoutController.dispose();
    _timeoutController.dispose();
    _cwdController.dispose();
    super.dispose();
  }

  void _submit() {
    final id = _idController.text.trim();
    if (id.isEmpty) {
      setState(() => _error = 'Command ID is required.');
      return;
    }

    final exec = _execController.text
        .split('\n')
        .map((line) => line.trim())
        .where((line) => line.isNotEmpty)
        .toList();

    if (exec.isEmpty) {
      setState(() => _error = 'At least one exec line is required.');
      return;
    }

    final timeout = _timeoutController.text.trim().isEmpty
        ? null
        : int.tryParse(_timeoutController.text.trim());

    final command = robot_farm_api.CommandConfig(
      id: id,
      exec: exec,
      stdoutSuccessMessage: _stdoutController.text.trim().isEmpty
          ? null
          : _stdoutController.text.trim(),
      hidden: _hidden,
      timeoutSeconds: timeout,
      cwd: _cwdController.text.trim().isEmpty
          ? null
          : _cwdController.text.trim(),
    );

    Navigator.of(context).pop(command);
  }

  @override
  Widget build(BuildContext context) {
    final inset = MediaQuery.of(context).viewInsets.bottom;
    return Padding(
      padding: EdgeInsets.only(bottom: inset),
      child: SingleChildScrollView(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(
              widget.initial == null ? 'Add Command' : 'Edit Command',
              style: Theme.of(context).textTheme.titleLarge,
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _idController,
              decoration: const InputDecoration(labelText: 'Command ID'),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _execController,
              maxLines: 4,
              decoration: const InputDecoration(
                labelText: 'Exec (one line per argument)',
              ),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _stdoutController,
              decoration: const InputDecoration(
                labelText: 'Stdout success message',
              ),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _cwdController,
              decoration: const InputDecoration(
                labelText: 'Working directory (optional)',
              ),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _timeoutController,
              keyboardType: TextInputType.number,
              decoration: const InputDecoration(labelText: 'Timeout seconds'),
            ),
            SwitchListTile(
              value: _hidden,
              onChanged: (value) => setState(() => _hidden = value),
              title: const Text('Hidden'),
            ),
            if (_error != null) ...[
              Text(
                _error!,
                style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                  color: Theme.of(context).colorScheme.error,
                ),
              ),
              const SizedBox(height: 12),
            ],
            FilledButton(onPressed: _submit, child: const Text('Save')),
          ],
        ),
      ),
    );
  }
}

class _FeedActionsMenu extends StatelessWidget {
  const _FeedActionsMenu({
    required this.onRunCommand,
    required this.onEnqueueMessage,
    required this.onEditQueue,
    this.onAddWorker,
    this.onTerminate,
    this.terminateLabel,
  });

  final VoidCallback? onRunCommand;
  final VoidCallback? onEnqueueMessage;
  final VoidCallback? onEditQueue;
  final VoidCallback? onAddWorker;
  final VoidCallback? onTerminate;
  final String? terminateLabel;

  @override
  Widget build(BuildContext context) {
    final entries = <_FeedActionItem>[
      _FeedActionItem(
        action: _FeedMenuAction.runCommand,
        label: 'Run command',
        icon: Icons.terminal,
        handler: onRunCommand,
      ),
      _FeedActionItem(
        action: _FeedMenuAction.enqueueMessage,
        label: 'Enqueue message',
        icon: Icons.mail,
        handler: onEnqueueMessage,
      ),
      _FeedActionItem(
        action: _FeedMenuAction.editQueue,
        label: 'Modify queue',
        icon: Icons.list_alt,
        handler: onEditQueue,
      ),
      _FeedActionItem(
        action: _FeedMenuAction.addWorker,
        label: 'Add worker',
        icon: Icons.person_add,
        handler: onAddWorker,
      ),
    ];

    if (onTerminate != null) {
      entries.add(
        _FeedActionItem(
          action: _FeedMenuAction.terminate,
          label: terminateLabel ?? 'Terminate',
          icon: Icons.stop_circle_outlined,
          handler: onTerminate,
        ),
      );
    }

    if (entries.every((item) => item.handler == null)) {
      return const SizedBox.shrink();
    }

    final theme = Theme.of(context);
    return PopupMenuButton<_FeedMenuAction>(
      tooltip: 'Feed actions',
      icon: const Icon(Icons.more_vert),
      onSelected: (action) {
        switch (action) {
          case _FeedMenuAction.runCommand:
            onRunCommand?.call();
            break;
          case _FeedMenuAction.enqueueMessage:
            onEnqueueMessage?.call();
            break;
          case _FeedMenuAction.editQueue:
            onEditQueue?.call();
            break;
          case _FeedMenuAction.addWorker:
            onAddWorker?.call();
            break;
          case _FeedMenuAction.terminate:
            onTerminate?.call();
            break;
        }
      },
      itemBuilder: (context) {
        return entries
            .map(
              (entry) => PopupMenuItem<_FeedMenuAction>(
                value: entry.action,
                enabled: entry.handler != null,
                child: Row(
                  children: [
                    Icon(
                      entry.icon,
                      color: entry.handler != null
                          ? theme.colorScheme.onSurface
                          : theme.disabledColor,
                    ),
                    const SizedBox(width: 12),
                    Text(entry.label),
                  ],
                ),
              ),
            )
            .toList();
      },
    );
  }
}

enum _FeedMenuAction { runCommand, enqueueMessage, editQueue, addWorker, terminate }

class _FeedActionItem {
  const _FeedActionItem({
    required this.action,
    required this.label,
    required this.icon,
    this.handler,
  });

  final _FeedMenuAction action;
  final String label;
  final IconData icon;
  final VoidCallback? handler;
}

class _HomeStatusBar extends StatelessWidget {
  const _HomeStatusBar({required this.controller});

  final ConnectionController controller;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Obx(() {
      final strategy = controller.activeStrategy.value;
      final name = strategy?.id.value ?? 'Unknown';
      final focusList = strategy?.focus ?? const <int>[];
      final focus = focusList.isEmpty
          ? 'none'
          : focusList.map((id) => id.toString()).join(', ');
      return Container(
        width: double.infinity,
        color: theme.colorScheme.surfaceContainerHighest,
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
        child: Row(
          children: [
            Expanded(
              child: Text(
                'Active Strategy: $name | Focus: [$focus]',
                style: theme.textTheme.labelSmall,
              ),
            ),
            const SizedBox(width: 8),
            Text(
              'Websocket: ${controller.websocketStatusLabel}',
              style: theme.textTheme.labelSmall?.copyWith(
                color: controller.websocketStatusColor(theme),
                fontWeight: FontWeight.bold,
              ),
            ),
          ],
        ),
      );
    });
  }
}

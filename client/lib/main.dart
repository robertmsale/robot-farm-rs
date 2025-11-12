import 'dart:async';
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;
import 'package:shared_preferences/shared_preferences.dart';
import 'package:web_socket/web_socket.dart' as ws;

import 'sheets/command_sheet.dart';

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
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.greenAccent),
        useMaterial3: true,
      ),
      initialRoute: '/',
      getPages: [
        GetPage(name: '/', page: () => const ConnectionScreen()),
        GetPage(name: '/home', page: () => const HomeScreen()),
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
  final RxList<String> pastLogins = <String>[].obs;
  final RxList<robot_farm_api.Worker> workers = <robot_farm_api.Worker>[].obs;
  ws.WebSocket? _webSocket;
  StreamSubscription<ws.WebSocketEvent>? _webSocketSubscription;
  SharedPreferences? _prefs;
  String? _currentBaseUrl;

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
          default:
            break;
        }
      });
      return true;
    } catch (error) {
      websocketStatus.value = WebsocketStatus.failed;
      websocketError.value = 'WebSocket failed: $error';
      return false;
    }
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
      }
    } catch (error) {
      debugPrint('Failed to parse WebSocket message: $error');
    }
  }

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

  void _closeWebSocket() {
    _webSocketSubscription?.cancel();
    _webSocketSubscription = null;
    _webSocket?.close();
    _webSocket = null;
    _currentBaseUrl = null;
    workers.clear();
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

  @override
  Widget build(BuildContext context) {
    final isPhone = context.isPhone;

    final orchestratorPane = OrchestratorPane(
      onRunCommand: () => _openCommandSheet(context),
    );
    final workerPane = WorkerFeedPane(
      onRunCommand: (workerId) =>
          _openCommandSheet(context, workerId: workerId),
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
        title: const Text('Robot Farm'),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Get.offAllNamed('/'),
        ),
      ),
      body: Padding(padding: const EdgeInsets.all(24), child: child),
    );
  }
}

class OrchestratorPane extends StatelessWidget {
  const OrchestratorPane({required this.onRunCommand, super.key});

  final VoidCallback onRunCommand;

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
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text('Orchestrator Feed', style: theme.textTheme.titleLarge),
            const SizedBox(height: 12),
            const Expanded(
              child: Center(
                child: Text(
                  'Turn-by-turn orchestrator output will appear here.',
                  textAlign: TextAlign.center,
                ),
              ),
            ),
            FilledButton.icon(
              icon: const Icon(Icons.terminal),
              label: const Text('Run staging command'),
              onPressed: onRunCommand,
            ),
          ],
        ),
      ),
    );
  }
}

class WorkerFeedPane extends StatefulWidget {
  const WorkerFeedPane({required this.onRunCommand, super.key});

  final void Function(int workerId) onRunCommand;

  @override
  State<WorkerFeedPane> createState() => _WorkerFeedPaneState();
}

class _WorkerFeedPaneState extends State<WorkerFeedPane>
    with SingleTickerProviderStateMixin {
  TabController? _controller;
  late final ConnectionController _connectionController;

  @override
  void initState() {
    super.initState();
    _connectionController = Get.find<ConnectionController>();
  }

  @override
  void dispose() {
    _controller?.dispose();
    super.dispose();
  }

  void _ensureController(int length) {
    if (_controller == null || _controller!.length != length) {
      _controller?.dispose();
      _controller = TabController(length: length, vsync: this);
    }
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Obx(() {
      final workers = _connectionController.workers.toList();
      final tabCount = workers.isEmpty ? 1 : workers.length;
      _ensureController(tabCount);
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
          child: DefaultTabController(
            length: tabCount,
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                TabBar(controller: _controller, isScrollable: true, tabs: tabs),
                const SizedBox(height: 12),
                Expanded(
                  child: TabBarView(
                    controller: _controller,
                    children: workers.isEmpty
                        ? const [
                            Center(
                              child: Text(
                                'No workers detected. Add a git worktree to see it here.',
                              ),
                            ),
                          ]
                        : workers
                              .map(
                                (worker) => Center(
                                  child: Text(
                                    'Feed for worker ${worker.id} (stub).',
                                    style: theme.textTheme.bodyLarge,
                                  ),
                                ),
                              )
                              .toList(),
                  ),
                ),
                const SizedBox(height: 12),
                FilledButton.icon(
                  icon: const Icon(Icons.terminal),
                  label: const Text('Run workspace command'),
                  onPressed: workers.isEmpty
                      ? null
                      : () {
                          final index = _controller?.index ?? 0;
                          final worker = workers[index];
                          widget.onRunCommand(worker.id);
                        },
                ),
              ],
            ),
          ),
        ),
      );
    });
  }
}

import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;

const int kDefaultApiPort = 8080;

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
      home: const ConnectionScreen(),
    );
  }
}

enum HealthStatus { idle, checking, ok, down }

class ConnectionController extends GetxController {
  final TextEditingController urlController = TextEditingController();
  final Rx<HealthStatus> healthStatus = HealthStatus.idle.obs;
  final RxnString errorMessage = RxnString();

  Future<void> checkHealth() async {
    final rawUrl = urlController.text.trim();
    if (rawUrl.isEmpty) {
      errorMessage.value = 'Please enter a server URL.';
      healthStatus.value = HealthStatus.down;
      return;
    }

    final baseUrl = _buildBaseUrl(rawUrl);
    if (baseUrl == null) {
      errorMessage.value =
          'Please enter a host or host:port (paths and schemes are not required).';
      healthStatus.value = HealthStatus.down;
      return;
    }

    try {
      healthStatus.value = HealthStatus.checking;
      errorMessage.value = null;

      final client = robot_farm_api.ApiClient(basePath: baseUrl);
      final api = robot_farm_api.DefaultApi(client);
      final response = await api.getHealthz();

      if (response != null && response.status.toLowerCase() == 'ok') {
        healthStatus.value = HealthStatus.ok;
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
  }

  String get statusLabel {
    switch (healthStatus.value) {
      case HealthStatus.ok:
        return 'OK';
      case HealthStatus.down:
        return 'Down';
      case HealthStatus.checking:
        return 'Checking...';
      case HealthStatus.idle:
        return 'Unknown';
    }
  }

  Color statusColor(ThemeData theme) {
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

    return Uri(
      scheme: scheme,
      host: uri.host,
      port: port,
    ).toString();
  }

  @override
  void onClose() {
    urlController.dispose();
    super.onClose();
  }
}

class ConnectionScreen extends GetView<ConnectionController> {
  const ConnectionScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Scaffold(
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
                        onPressed: controller.healthStatus.value ==
                                HealthStatus.checking
                            ? null
                            : controller.checkHealth,
                        child: controller.healthStatus.value ==
                                HealthStatus.checking
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
                            : const Text('Check Health'),
                      ),
                    ),
                  ),
                  const SizedBox(height: 24),
                  Obx(
                    () => Column(
                      children: [
                        Text(
                          controller.statusLabel,
                          style: theme.textTheme.displaySmall?.copyWith(
                            color: controller.statusColor(theme),
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
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}

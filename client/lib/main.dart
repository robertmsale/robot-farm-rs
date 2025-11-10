import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

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

class ConnectionController extends GetxController {
  final TextEditingController urlController = TextEditingController();
  final RxBool isConnecting = false.obs;
  final RxnString errorMessage = RxnString();
  final RxnString connectionStatus = RxnString();
  WebSocketChannel? _channel;

  Future<void> connect() async {
    final url = urlController.text.trim();
    if (url.isEmpty) {
      errorMessage.value = 'Please enter a server URL.';
      return;
    }

    try {
      isConnecting.value = true;
      errorMessage.value = null;
      connectionStatus.value = null;
      _channel?.sink.close();

      final parsed = Uri.parse(url);
      _channel = WebSocketChannel.connect(parsed);
      connectionStatus.value = 'Connected to $url';
    } catch (error) {
      errorMessage.value = 'Failed to connect: $error';
    } finally {
      isConnecting.value = false;
    }
  }

  @override
  void onClose() {
    urlController.dispose();
    _channel?.sink.close();
    super.onClose();
  }
}

class ConnectionScreen extends GetView<ConnectionController> {
  const ConnectionScreen({super.key});

  @override
  Widget build(BuildContext context) {
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
                    style: Theme.of(context).textTheme.headlineSmall,
                  ),
                  const SizedBox(height: 12),
                  Text(
                    'Enter the websocket endpoint you want to connect to.',
                    textAlign: TextAlign.center,
                    style: Theme.of(context).textTheme.bodyMedium,
                  ),
                  const SizedBox(height: 24),
                  TextField(
                    controller: controller.urlController,
                    keyboardType: TextInputType.url,
                    decoration: const InputDecoration(
                      labelText: 'Connection URL',
                      hintText: 'wss://robot-farm.example/ws',
                      border: OutlineInputBorder(),
                    ),
                  ),
                  const SizedBox(height: 16),
                  Obx(
                    () => SizedBox(
                      width: double.infinity,
                      child: FilledButton(
                        onPressed: controller.isConnecting.value
                            ? null
                            : controller.connect,
                        child: controller.isConnecting.value
                            ? const SizedBox(
                                width: 18,
                                height: 18,
                                child: CircularProgressIndicator(
                                  strokeWidth: 2,
                                  valueColor:
                                      AlwaysStoppedAnimation<Color>(Colors.white),
                                ),
                              )
                            : const Text('Connect'),
                      ),
                    ),
                  ),
                  const SizedBox(height: 16),
                  Obx(
                    () {
                      final error = controller.errorMessage.value;
                      if (error == null) {
                        return const SizedBox.shrink();
                      }
                      return Text(
                        error,
                        style: TextStyle(
                          color: Theme.of(context).colorScheme.error,
                        ),
                      );
                    },
                  ),
                  const SizedBox(height: 8),
                  Obx(
                    () {
                      final status = controller.connectionStatus.value;
                      if (status == null) {
                        return const SizedBox.shrink();
                      }
                      return Text(
                        status,
                        style: const TextStyle(color: Colors.green),
                      );
                    },
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

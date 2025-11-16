import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;

class EnqueueMessageSheet extends StatefulWidget {
  const EnqueueMessageSheet({
    required this.baseUrlProvider,
    this.initialTarget,
    this.initialSender = 'Orchestrator',
    super.key,
  });

  final String? Function() baseUrlProvider;
  final String? initialTarget;
  final String initialSender;

  @override
  State<EnqueueMessageSheet> createState() => _EnqueueMessageSheetState();
}

class _EnqueueMessageSheetState extends State<EnqueueMessageSheet> {
  late final TextEditingController targetController;
  late final TextEditingController senderController;
  late final TextEditingController messageController;
  bool _isSubmitting = false;

  @override
  void initState() {
    super.initState();
    targetController = TextEditingController(
      text: widget.initialTarget ?? 'Orchestrator',
    );
    senderController = TextEditingController(text: widget.initialSender);
    messageController = TextEditingController();
  }

  @override
  void dispose() {
    targetController.dispose();
    senderController.dispose();
    messageController.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    final baseUrl = widget.baseUrlProvider();
    if (baseUrl == null) {
      Get.snackbar(
        'Not connected',
        'Connect to a server before enqueueing messages.',
      );
      return;
    }

    final normalizedTarget = _normalizeActor(targetController.text);
    final normalizedSender = _normalizeActor(senderController.text);
    final body = messageController.text.trim();

    if (normalizedTarget == null) {
      Get.snackbar(
        'Invalid target',
        'Use Orchestrator, Quality Assurance, or ws# handles.',
      );
      return;
    }
    if (normalizedSender == null) {
      Get.snackbar(
        'Invalid sender',
        'Use Orchestrator, Quality Assurance, or ws# handles.',
      );
      return;
    }
    if (body.isEmpty) {
      Get.snackbar('Empty message', 'Write a message before enqueueing.');
      return;
    }

    setState(() {
      _isSubmitting = true;
    });

    final apiClient = robot_farm_api.ApiClient(basePath: baseUrl);
    final payload = {
      'from': normalizedSender,
      'to': normalizedTarget,
      'message': body,
    };

    try {
      final response = await apiClient.invokeAPI(
        '/message_queue',
        'POST',
        <robot_farm_api.QueryParam>[],
        payload,
        <String, String>{},
        <String, String>{},
        'application/json',
      );

      if (response.statusCode >= 400) {
        throw robot_farm_api.ApiException(response.statusCode, response.body);
      }

      if (!mounted) return;
      Get.snackbar('Message enqueued', 'Sent to $normalizedTarget.');
      Navigator.of(context).maybePop();
    } on robot_farm_api.ApiException catch (err) {
      Get.snackbar(
        'Failed to enqueue',
        err.message ?? 'Server rejected the request (HTTP ${err.code}).',
      );
    } catch (error) {
      Get.snackbar('Failed to enqueue', '$error');
    } finally {
      if (mounted) {
        setState(() {
          _isSubmitting = false;
        });
      }
    }
  }

  String? _normalizeActor(String value) {
    final trimmed = value.trim();
    if (trimmed.isEmpty) {
      return null;
    }
    final lower = trimmed.toLowerCase();
    if (lower == 'orchestrator') {
      return 'Orchestrator';
    }
    if (lower == 'quality assurance' || lower == 'qa') {
      return 'Quality Assurance';
    }
    if (lower.startsWith('ws')) {
      final digits = lower.substring(2);
      final workerId = int.tryParse(digits);
      if (workerId != null && workerId > 0) {
        return 'ws$workerId';
      }
    }
    return null;
  }

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      child: Padding(
        padding: EdgeInsets.only(
          left: 24,
          right: 24,
          top: 24,
          bottom: MediaQuery.of(context).viewInsets.bottom + 24,
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Row(
              children: [
                Text(
                  'Enqueue Message',
                  style: Theme.of(context).textTheme.titleLarge,
                ),
                const Spacer(),
                IconButton(
                  icon: const Icon(Icons.close),
                  onPressed: () => Navigator.of(context).maybePop(),
                ),
              ],
            ),
            const SizedBox(height: 12),
            TextField(
              controller: senderController,
              decoration: const InputDecoration(
                labelText: 'Sender (Orchestrator, QA, ws#)',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: targetController,
              decoration: const InputDecoration(
                labelText:
                    'Target feed (Orchestrator, ws3, Quality Assurance, etc.)',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 12),
            Expanded(
              child: TextField(
                controller: messageController,
                expands: true,
                maxLines: null,
                textAlignVertical: TextAlignVertical.top,
                decoration: const InputDecoration(
                  labelText: 'Message',
                  border: OutlineInputBorder(),
                  alignLabelWithHint: true,
                ),
              ),
            ),
            const SizedBox(height: 12),
            FilledButton.icon(
              icon: _isSubmitting
                  ? const SizedBox(
                      width: 16,
                      height: 16,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    )
                  : const Icon(Icons.send),
              label: const Text('Enqueue'),
              onPressed: _isSubmitting ? null : _submit,
            ),
          ],
        ),
      ),
    );
  }
}

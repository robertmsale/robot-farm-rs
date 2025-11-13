import 'package:flutter/material.dart';

class EnqueueMessageSheet extends StatefulWidget {
  const EnqueueMessageSheet({super.key, this.initialTarget});

  final String? initialTarget;

  @override
  State<EnqueueMessageSheet> createState() => _EnqueueMessageSheetState();
}

class _EnqueueMessageSheetState extends State<EnqueueMessageSheet> {
  late final TextEditingController targetController;
  late final TextEditingController messageController;

  @override
  void initState() {
    super.initState();
    targetController = TextEditingController(text: widget.initialTarget);
    messageController = TextEditingController();
  }

  @override
  void dispose() {
    targetController.dispose();
    messageController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Row(
              children: [
                Text('Enqueue Message', style: Theme.of(context).textTheme.titleLarge),
                const Spacer(),
                IconButton(
                  icon: const Icon(Icons.close),
                  onPressed: () => Navigator.of(context).maybePop(),
                ),
              ],
            ),
            const SizedBox(height: 12),
            TextField(
              controller: targetController,
              decoration: const InputDecoration(
                labelText: 'Target feed (Orchestrator, ws3, Quality Assurance, etc.)',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 12),
            Expanded(
              child: TextField(
                controller: messageController,
                expands: true,
                maxLines: null,
                decoration: const InputDecoration(
                  labelText: 'Message',
                  border: OutlineInputBorder(),
                ),
              ),
            ),
            const SizedBox(height: 12),
            FilledButton.icon(
              icon: const Icon(Icons.send),
              label: const Text('Enqueue'),
              onPressed: () {},
            )
          ],
        ),
      ),
    );
  }
}

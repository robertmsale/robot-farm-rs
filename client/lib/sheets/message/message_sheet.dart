import 'package:flutter/material.dart';

class EnqueueMessageSheet extends StatelessWidget {
  const EnqueueMessageSheet({super.key, this.initialTarget});

  final String? initialTarget;

  @override
  Widget build(BuildContext context) {
    final targetController = TextEditingController(text: initialTarget);
    final messageController = TextEditingController();

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

import 'package:flutter/material.dart';

import 'models.dart';

class QueueSheet extends StatelessWidget {
  const QueueSheet({required this.messages, super.key});

  final List<QueueMessageViewModel> messages;

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
                Text('Modify Queue', style: Theme.of(context).textTheme.titleLarge),
                const Spacer(),
                IconButton(
                  icon: const Icon(Icons.close),
                  onPressed: () => Navigator.of(context).maybePop(),
                ),
              ],
            ),
            const SizedBox(height: 12),
            Expanded(
              child: messages.isEmpty
                  ? const Center(child: Text('Queue is empty.'))
                  : ReorderableListView.builder(
                      itemBuilder: (context, index) {
                        final entry = messages[index];
                        return Card(
                          key: ValueKey(entry.message.id),
                          child: ListTile(
                            title: Text(entry.title),
                            subtitle: Text(entry.subtitle),
                            trailing: IconButton(
                              icon: const Icon(Icons.delete_outline),
                              onPressed: () {},
                            ),
                          ),
                        );
                      },
                      itemCount: messages.length,
                      onReorder: (oldIndex, newIndex) {},
                    ),
            ),
            const SizedBox(height: 12),
            FilledButton.icon(
              icon: const Icon(Icons.delete_sweep),
              label: const Text('Clear queue'),
              onPressed: () {},
            )
          ],
        ),
      ),
    );
  }
}

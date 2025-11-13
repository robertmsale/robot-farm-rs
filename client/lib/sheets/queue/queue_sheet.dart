import 'package:flutter/material.dart';

import 'models.dart';

class QueueSheet extends StatefulWidget {
  const QueueSheet({required this.messages, super.key});

  final List<QueueMessageViewModel> messages;

  @override
  State<QueueSheet> createState() => _QueueSheetState();
}

class _QueueSheetState extends State<QueueSheet> {
  late List<QueueMessageViewModel> _messages;

  @override
  void initState() {
    super.initState();
    _messages = List.of(widget.messages);
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
              child: _messages.isEmpty
                  ? const Center(child: Text('Queue is empty.'))
                  : ReorderableListView.builder(
                      itemBuilder: (context, index) {
                        final entry = _messages[index];
                        return Card(
                          key: ValueKey(entry.message.id),
                          child: ListTile(
                            title: Text(entry.title),
                            subtitle: Text(entry.subtitle),
                            trailing: IconButton(
                              icon: const Icon(Icons.delete_outline),
                              onPressed: () {
                                setState(() => _messages.removeAt(index));
                              },
                            ),
                          ),
                        );
                      },
                      itemCount: _messages.length,
                      onReorder: (oldIndex, newIndex) {
                        setState(() {
                          if (newIndex > oldIndex) newIndex -= 1;
                          final item = _messages.removeAt(oldIndex);
                          _messages.insert(newIndex, item);
                        });
                      },
                    ),
            ),
            const SizedBox(height: 12),
            FilledButton.icon(
              icon: const Icon(Icons.delete_sweep),
              label: const Text('Clear queue'),
              onPressed: () {
                setState(() => _messages.clear());
              },
            )
          ],
        ),
      ),
    );
  }
}

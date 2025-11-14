import 'package:flutter/material.dart';
import 'package:get/get.dart';

import 'task_wizard_controller.dart';

class TaskWizardScreen extends GetView<TaskWizardController> {
  const TaskWizardScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final isPhone = context.isPhone;
    final feedPane = _WizardFeedPane(controller: controller);
    final inputPane = _WizardInputPane(controller: controller);

    final content = isPhone
        ? Column(
            children: [
              Expanded(child: feedPane),
              const SizedBox(height: 16),
              Expanded(child: inputPane),
            ],
          )
        : Row(
            children: [
              Expanded(child: feedPane),
              const SizedBox(width: 16),
              Expanded(child: inputPane),
            ],
          );

    return Scaffold(
      appBar: AppBar(
        title: const Row(
          children: [
            Icon(Icons.auto_fix_high),
            SizedBox(width: 8),
            Text('Task Wizard'),
          ],
        ),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Get.back<void>(),
        ),
      ),
      body: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          children: [
            Obx(() {
              final message = controller.error.value;
              if (message == null) {
                return const SizedBox.shrink();
              }
              return Padding(
                padding: const EdgeInsets.only(bottom: 16),
                child: Row(
                  children: [
                    const Icon(Icons.error_outline, color: Colors.orange),
                    const SizedBox(width: 8),
                    Expanded(child: Text(message)),
                  ],
                ),
              );
            }),
            Expanded(child: content),
          ],
        ),
      ),
    );
  }
}

class _WizardFeedPane extends StatelessWidget {
  const _WizardFeedPane({required this.controller});

  final TaskWizardController controller;

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
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text('Wizard Feed', style: theme.textTheme.titleLarge),
                Obx(() {
                  final connected = controller.isConnected.value;
                  final color = connected
                      ? theme.colorScheme.primary
                      : theme.colorScheme.error;
                  final icon = connected
                      ? Icons.radio_button_checked
                      : Icons.radio_button_unchecked;
                  return Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Icon(icon, color: color, size: 14),
                      const SizedBox(width: 4),
                      Text(
                        connected ? 'Connected' : 'Disconnected',
                        style: theme.textTheme.labelSmall?.copyWith(
                          color: color,
                        ),
                      ),
                    ],
                  );
                }),
              ],
            ),
            const SizedBox(height: 12),
            Expanded(
              child: Obx(() {
                final entries = controller.feed;
                if (entries.isEmpty) {
                  return const Center(
                    child: Text(
                      'Describe a task group to get started.',
                      textAlign: TextAlign.center,
                    ),
                  );
                }
                return ListView.separated(
                  itemCount: entries.length,
                  separatorBuilder: (_, __) => const SizedBox(height: 8),
                  itemBuilder: (context, index) {
                    final entry = entries[index];
                    return _WizardFeedEntryTile(entry: entry);
                  },
                );
              }),
            ),
          ],
        ),
      ),
    );
  }
}

class _WizardInputPane extends StatelessWidget {
  const _WizardInputPane({required this.controller});

  final TaskWizardController controller;

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
            Text('Instructions', style: theme.textTheme.titleLarge),
            const SizedBox(height: 8),
            Obx(() {
              final thread = controller.threadId;
              final threadLabel = thread == null
                  ? 'No thread yet'
                  : 'Thread $thread';
              final state = controller.isRunning.value ? 'In progress' : 'Idle';
              final session = controller.sessionId;
              return Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(threadLabel, style: theme.textTheme.labelMedium),
                      Text(
                        'Session ${session ?? '—'}',
                        style: theme.textTheme.labelSmall,
                      ),
                    ],
                  ),
                  Text(state, style: theme.textTheme.labelMedium),
                ],
              );
            }),
            const SizedBox(height: 12),
            Expanded(
              child: TextField(
                controller: controller.promptController,
                minLines: 8,
                maxLines: null,
                decoration: const InputDecoration(
                  border: OutlineInputBorder(),
                  hintText: 'Describe the task you want the wizard to run...',
                ),
                textAlignVertical: TextAlignVertical.top,
                textInputAction: TextInputAction.newline,
              ),
            ),
            const SizedBox(height: 12),
            Align(
              alignment: Alignment.centerRight,
              child: Obx(
                () => Wrap(
                  spacing: 12,
                  children: [
                    FilledButton.icon(
                      onPressed: controller.isRunning.value
                          ? controller.cancelRun
                          : null,
                      style: FilledButton.styleFrom(
                        backgroundColor: theme.colorScheme.error,
                        foregroundColor: theme.colorScheme.onError,
                        shape: const StadiumBorder(),
                      ),
                      icon: const Icon(Icons.stop_circle),
                      label: const Text('Cancel'),
                    ),
                    FilledButton.icon(
                      onPressed: controller.canSendPrompt
                          ? controller.sendPrompt
                          : null,
                      icon: const Icon(Icons.send),
                      label: const Text('Send'),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _WizardFeedEntryTile extends StatelessWidget {
  const _WizardFeedEntryTile({required this.entry});

  final TaskWizardFeedEntry entry;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    switch (entry.type) {
      case TaskWizardFeedEntryType.userPrompt:
        return ListTile(
          leading: const Icon(Icons.person_outline),
          title: const Text('You'),
          subtitle: Text(entry.message),
        );
      case TaskWizardFeedEntryType.wizardEvent:
        return ListTile(
          leading: const Icon(Icons.auto_fix_high),
          title: Text(entry.message),
        );
      case TaskWizardFeedEntryType.system:
        return Card(
          color: theme.colorScheme.surfaceContainerHighest,
          child: Padding(
            padding: const EdgeInsets.all(12),
            child: Text(
              entry.message,
              style: theme.textTheme.bodyMedium?.copyWith(
                fontStyle: FontStyle.italic,
              ),
            ),
          ),
        );
      case TaskWizardFeedEntryType.finalSummary:
        return Card(
          child: Padding(
            padding: const EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  'Wizard ${entry.status ?? 'done'}',
                  style: theme.textTheme.titleMedium,
                ),
                const SizedBox(height: 8),
                Text(entry.message),
                if (entry.feedEntry != null) ...[
                  const SizedBox(height: 8),
                  Text(
                    entry.feedEntry!.text,
                    style: theme.textTheme.labelMedium,
                  ),
                ],
              ],
            ),
          ),
        );
    }
  }
}

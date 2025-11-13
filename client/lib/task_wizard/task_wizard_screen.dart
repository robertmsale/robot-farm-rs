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
        actions: const [],
      ),
      body: Padding(
        padding: const EdgeInsets.all(24),
        child: content,
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
            Text('Wizard Feed', style: theme.textTheme.titleLarge),
            const SizedBox(height: 12),
            Expanded(
              child: Obx(
                () {
                  if (controller.feed.isEmpty) {
                    return const Center(
                      child: Text(
                        'Task wizard output will appear here (stub).',
                        textAlign: TextAlign.center,
                      ),
                    );
                  }
                  return ListView.builder(
                    itemCount: controller.feed.length,
                    itemBuilder: (context, index) {
                      final entry = controller.feed[index];
                      return Padding(
                        padding: const EdgeInsets.symmetric(
                          vertical: 6,
                        ),
                        child: Text(entry),
                      );
                    },
                  );
                },
              ),
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
                        backgroundColor: Theme.of(context).colorScheme.error,
                        foregroundColor: Theme.of(context).colorScheme.onError,
                        shape: const StadiumBorder(),
                      ),
                      icon: const Icon(Icons.stop_circle),
                      label: const Text('Cancel'),
                    ),
                    FilledButton.icon(
                      onPressed: controller.hasPrompt.value
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

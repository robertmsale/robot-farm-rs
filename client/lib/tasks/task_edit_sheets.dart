import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;

import 'task_edit_payloads.dart';

enum TaskGroupEditorAction { save, delete }

class TaskGroupEditorResult {
  const TaskGroupEditorResult._(this.action, this.payload);

  const TaskGroupEditorResult.save(TaskGroupEditPayload payload)
    : this._(TaskGroupEditorAction.save, payload);

  const TaskGroupEditorResult.delete()
    : this._(TaskGroupEditorAction.delete, null);

  final TaskGroupEditorAction action;
  final TaskGroupEditPayload? payload;
}

enum TaskEditorAction { save, delete }

class TaskEditorResult {
  const TaskEditorResult._(this.action, this.payload);

  const TaskEditorResult.save(TaskEditPayload payload)
    : this._(TaskEditorAction.save, payload);

  const TaskEditorResult.delete() : this._(TaskEditorAction.delete, null);

  final TaskEditorAction action;
  final TaskEditPayload? payload;
}

Future<TaskGroupEditorResult?> showTaskGroupEditorSheet(
  BuildContext context, {
  robot_farm_api.TaskGroup? group,
  bool isCreate = false,
}) {
  return showModalBottomSheet<TaskGroupEditorResult>(
    context: context,
    isScrollControlled: true,
    builder: (_) {
      return GetBuilder<TaskGroupEditorController>(
        init: TaskGroupEditorController(group: group),
        builder: (form) {
          final bottomInset = MediaQuery.of(context).viewInsets.bottom;

          return AnimatedPadding(
            duration: const Duration(milliseconds: 180),
            curve: Curves.easeOut,
            padding: EdgeInsets.only(bottom: bottomInset),
            child: FractionallySizedBox(
              heightFactor: 0.85,
              child: SafeArea(
                top: false,
                child: SingleChildScrollView(
                  padding: const EdgeInsets.all(24),
                  physics: const BouncingScrollPhysics(),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      Row(
                        children: [
                          Text(
                            isCreate ? 'Create Task Group' : 'Edit Task Group',
                            style: Theme.of(context).textTheme.titleLarge,
                          ),
                          const Spacer(),
                          IconButton(
                            tooltip: 'Close',
                            icon: const Icon(Icons.close),
                            onPressed: () => Navigator.of(context).maybePop(),
                          ),
                        ],
                      ),
                      const SizedBox(height: 16),
                      TextField(
                        controller: form.slugController,
                        textInputAction: TextInputAction.next,
                        decoration: const InputDecoration(
                          labelText: 'Slug',
                          border: OutlineInputBorder(),
                        ),
                      ),
                      const SizedBox(height: 12),
                      TextField(
                        controller: form.titleController,
                        textInputAction: TextInputAction.next,
                        decoration: const InputDecoration(
                          labelText: 'Title',
                          border: OutlineInputBorder(),
                        ),
                      ),
                      const SizedBox(height: 12),
                      TextField(
                        controller: form.descriptionController,
                        maxLines: null,
                        minLines: 6,
                        keyboardType: TextInputType.multiline,
                        textAlignVertical: TextAlignVertical.top,
                        decoration: const InputDecoration(
                          labelText: 'Description',
                          border: OutlineInputBorder(),
                          alignLabelWithHint: true,
                        ),
                      ),
                      const SizedBox(height: 12),
                      InputDecorator(
                        decoration: const InputDecoration(
                          labelText: 'Status',
                          border: OutlineInputBorder(),
                        ),
                        child: Text(
                          group?.status.value ??
                              robot_farm_api.TaskGroupStatus.ready.value,
                        ),
                      ),
                      const SizedBox(height: 20),
                      Row(
                        children: [
                          if (!isCreate)
                            Tooltip(
                              message: _canDeleteGroup(group)
                                  ? 'Delete this task group'
                                  : 'Built-in groups cannot be deleted',
                              child: TextButton.icon(
                                icon: const Icon(Icons.delete_outline),
                                label: const Text('Delete'),
                                onPressed: _canDeleteGroup(group)
                                    ? () async {
                                        final confirmed =
                                            await _confirmDeletion(
                                              context,
                                              group?.title ?? 'this group',
                                            ) ??
                                            false;
                                        if (!confirmed || !context.mounted) {
                                          return;
                                        }
                                        Navigator.of(context).pop(
                                          const TaskGroupEditorResult.delete(),
                                        );
                                      }
                                    : null,
                              ),
                            ),
                          const Spacer(),
                          TextButton(
                            onPressed: () => Navigator.of(context).maybePop(),
                            child: const Text('Cancel'),
                          ),
                          const SizedBox(width: 12),
                          FilledButton(
                            onPressed: form.isValid
                                ? () => Navigator.of(context).pop(
                                    TaskGroupEditorResult.save(
                                      form.buildPayload(),
                                    ),
                                  )
                                : null,
                            child: Text(isCreate ? 'Create' : 'Save changes'),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
            ),
          );
        },
      );
    },
  );
}

Future<bool?> _confirmDeletion(BuildContext context, String title) {
  return showDialog<bool>(
    context: context,
    builder: (context) => AlertDialog(
      title: const Text('Delete task group'),
      content: Text('Delete $title permanently?'),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(false),
          child: const Text('Cancel'),
        ),
        FilledButton(
          onPressed: () => Navigator.of(context).pop(true),
          child: const Text('Delete'),
        ),
      ],
    ),
  );
}

bool _canDeleteGroup(robot_farm_api.TaskGroup? group) {
  if (group == null) {
    return false;
  }
  final slug = group.slug.toLowerCase();
  return slug != 'chores' && slug != 'bugs' && slug != 'hotfix';
}

Future<TaskEditorResult?> showTaskEditorSheet(
  BuildContext context, {
  robot_farm_api.Task? task,
  bool isCreate = false,
  List<String> workerHandles = const <String>[],
  String? defaultWorkerModel,
  List<String> modelOptions = const [],
}) {
  return showModalBottomSheet<TaskEditorResult>(
    context: context,
    isScrollControlled: true,
    builder: (_) {
      return GetBuilder<TaskEditorController>(
        init: TaskEditorController(
          task: task,
          workerHandles: workerHandles,
          defaultWorkerModel: defaultWorkerModel,
          modelOptions: modelOptions,
        ),
        builder: (form) {
          final bottomInset = MediaQuery.of(context).viewInsets.bottom;

          return AnimatedPadding(
            duration: const Duration(milliseconds: 180),
            curve: Curves.easeOut,
            padding: EdgeInsets.only(bottom: bottomInset),
            child: FractionallySizedBox(
              heightFactor: 0.9,
              child: SafeArea(
                top: false,
                child: SingleChildScrollView(
                  padding: const EdgeInsets.all(24),
                  physics: const BouncingScrollPhysics(),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      Row(
                        children: [
                          Text(
                            isCreate ? 'Create Task' : 'Edit Task',
                            style: Theme.of(context).textTheme.titleLarge,
                          ),
                          const Spacer(),
                          IconButton(
                            tooltip: 'Close',
                            icon: const Icon(Icons.close),
                            onPressed: () => Navigator.of(context).maybePop(),
                          ),
                        ],
                      ),
                      const SizedBox(height: 16),
                      TextField(
                        controller: form.slugController,
                        textInputAction: TextInputAction.next,
                        decoration: const InputDecoration(
                          labelText: 'Slug',
                          border: OutlineInputBorder(),
                        ),
                      ),
                      const SizedBox(height: 12),
                      TextField(
                        controller: form.titleController,
                        textInputAction: TextInputAction.next,
                        decoration: const InputDecoration(
                          labelText: 'Title',
                          border: OutlineInputBorder(),
                        ),
                      ),
                      const SizedBox(height: 12),
                      TextField(
                        controller: form.commitController,
                        textInputAction: TextInputAction.next,
                        decoration: const InputDecoration(
                          labelText: 'Commit hash (optional)',
                          border: OutlineInputBorder(),
                        ),
                      ),
                      const SizedBox(height: 12),
                      DropdownButtonFormField<robot_farm_api.TaskStatus>(
                        initialValue: form.selectedStatus,
                        items: robot_farm_api.TaskStatus.values
                            .map(
                              (status) => DropdownMenuItem(
                                value: status,
                                child: Text(status.value),
                              ),
                            )
                            .toList(),
                        onChanged: form.updateStatus,
                        decoration: const InputDecoration(
                          labelText: 'Status',
                          border: OutlineInputBorder(),
                        ),
                      ),
                      const SizedBox(height: 12),
                      DropdownButtonFormField<String>(
                        initialValue: form.selectedOwner,
                        items: form.ownerOptions
                            .map(
                              (owner) => DropdownMenuItem(
                                value: owner,
                                child: Text(owner),
                              ),
                            )
                            .toList(),
                        onChanged: form.updateOwner,
                        decoration: const InputDecoration(
                          labelText: 'Owner',
                          border: OutlineInputBorder(),
                        ),
                      ),
                      const SizedBox(height: 12),
                      DropdownButtonFormField<String>(
                        initialValue: form.modelOverride,
                        decoration: const InputDecoration(
                          labelText: 'Worker model override',
                          border: OutlineInputBorder(),
                        ),
                        items: [
                          const DropdownMenuItem<String>(
                            value: null,
                            child: Text('Use config default'),
                          ),
                          ...form.modelOptions.map(
                            (model) => DropdownMenuItem(
                              value: model,
                              child: Text(model),
                            ),
                          ),
                        ],
                        onChanged: form.updateModelOverride,
                      ),
                      const SizedBox(height: 12),
                      DropdownButtonFormField<String>(
                        initialValue: form.reasoningOverride,
                        decoration: const InputDecoration(
                          labelText: 'Reasoning override',
                          border: OutlineInputBorder(),
                        ),
                        items: form.reasoningOptions
                            .map(
                              (level) => DropdownMenuItem(
                                value: level,
                                child: Text(_formatReasoningLabel(level)),
                              ),
                            )
                            .toList(),
                        onChanged: form.updateReasoningOverride,
                      ),
                      const SizedBox(height: 12),
                      TextField(
                        controller: form.descriptionController,
                        maxLines: null,
                        minLines: 10,
                        keyboardType: TextInputType.multiline,
                        textAlignVertical: TextAlignVertical.top,
                        decoration: const InputDecoration(
                          labelText: 'Description',
                          border: OutlineInputBorder(),
                          alignLabelWithHint: true,
                        ),
                      ),
                      const SizedBox(height: 16),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.end,
                        children: [
                          if (!isCreate)
                            TextButton.icon(
                              icon: const Icon(Icons.delete_outline),
                              label: const Text('Delete'),
                              style: TextButton.styleFrom(
                                foregroundColor: Theme.of(
                                  context,
                                ).colorScheme.error,
                              ),
                              onPressed: () async {
                                final confirmed =
                                    await showDialog<bool>(
                                      context: context,
                                      builder: (context) => AlertDialog(
                                        title: const Text('Delete task'),
                                        content: Text(
                                          'Delete ${task?.title ?? 'this task'} permanently?',
                                        ),
                                        actions: [
                                          TextButton(
                                            onPressed: () => Navigator.of(
                                              context,
                                            ).pop(false),
                                            child: const Text('Cancel'),
                                          ),
                                          FilledButton(
                                            onPressed: () =>
                                                Navigator.of(context).pop(true),
                                            child: const Text('Delete'),
                                          ),
                                        ],
                                      ),
                                    ) ??
                                    false;
                                if (!confirmed || !context.mounted) {
                                  return;
                                }
                                Navigator.of(
                                  context,
                                ).pop(const TaskEditorResult.delete());
                              },
                            ),
                          const Spacer(),
                          TextButton(
                            onPressed: () => Navigator.of(context).maybePop(),
                            child: const Text('Cancel'),
                          ),
                          const SizedBox(width: 12),
                          FilledButton(
                            onPressed: form.isValid
                                ? () => Navigator.of(context).pop(
                                    TaskEditorResult.save(form.buildPayload()),
                                  )
                                : null,
                            child: Text(isCreate ? 'Create' : 'Save changes'),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
            ),
          );
        },
      );
    },
  );
}

class TaskGroupEditorController extends GetxController {
  TaskGroupEditorController({this.group});

  final robot_farm_api.TaskGroup? group;
  late final TextEditingController slugController;
  late final TextEditingController titleController;
  late final TextEditingController descriptionController;

  @override
  void onInit() {
    super.onInit();
    slugController = TextEditingController(text: group?.slug ?? '')
      ..addListener(_invalidate);
    titleController = TextEditingController(text: group?.title ?? '')
      ..addListener(_invalidate);
    descriptionController = TextEditingController(
      text: group?.description ?? '',
    )..addListener(_invalidate);
  }

  void _invalidate() => update();

  bool get isValid =>
      slugController.text.trim().isNotEmpty &&
      titleController.text.trim().isNotEmpty &&
      descriptionController.text.trim().isNotEmpty;

  TaskGroupEditPayload buildPayload() => TaskGroupEditPayload(
    slug: slugController.text.trim(),
    title: titleController.text.trim(),
    description: descriptionController.text.trim(),
  );

  @override
  void onClose() {
    slugController.dispose();
    titleController.dispose();
    descriptionController.dispose();
    super.onClose();
  }
}

class TaskEditorController extends GetxController {
  TaskEditorController({
    this.task,
    required this.workerHandles,
    this.defaultWorkerModel,
    this.modelOptions = const <String>[],
  });

  final robot_farm_api.Task? task;
  final List<String> workerHandles;
  final String? defaultWorkerModel;
  final List<String> modelOptions;
  late final TextEditingController slugController;
  late final TextEditingController titleController;
  late final TextEditingController commitController;
  late final TextEditingController descriptionController;
  late robot_farm_api.TaskStatus selectedStatus;
  late final List<String> ownerOptions;
  late String selectedOwner;
  String? modelOverride;
  String? reasoningOverride;
  static const int _workerHandleCount = 50;

  @override
  void onInit() {
    super.onInit();
    slugController = TextEditingController(text: task?.slug ?? '')
      ..addListener(_invalidate);
    titleController = TextEditingController(text: task?.title ?? '')
      ..addListener(_invalidate);
    commitController = TextEditingController(text: task?.commitHash ?? '')
      ..addListener(_invalidate);
    descriptionController = TextEditingController(text: task?.description ?? '')
      ..addListener(_invalidate);
    selectedStatus = task?.status ?? robot_farm_api.TaskStatus.ready;
    ownerOptions = _buildOwnerOptions();
    final normalizedOwner = (task?.owner ?? '').trim();
    selectedOwner = normalizedOwner.isNotEmpty
        ? normalizedOwner
        : 'Orchestrator';
    if (!ownerOptions.contains(selectedOwner)) {
      ownerOptions.add(selectedOwner);
    }

    modelOverride = task?.modelOverride;
    reasoningOverride = task?.reasoningOverride;
  }

  void updateStatus(robot_farm_api.TaskStatus? status) {
    if (status == null) {
      return;
    }
    selectedStatus = status;
    update();
  }

  void updateOwner(String? owner) {
    if (owner == null) {
      return;
    }
    selectedOwner = owner;
    update();
  }

  void _invalidate() => update();

  bool get isValid =>
      slugController.text.trim().isNotEmpty &&
      titleController.text.trim().isNotEmpty &&
      selectedOwner.trim().isNotEmpty &&
      descriptionController.text.trim().isNotEmpty;

  TaskEditPayload buildPayload() => TaskEditPayload(
    slug: slugController.text.trim(),
    title: titleController.text.trim(),
    commitHash: commitController.text.trim().isEmpty
        ? null
        : commitController.text.trim(),
    status: selectedStatus,
    owner: selectedOwner.trim(),
    description: descriptionController.text.trim(),
    modelOverride: modelOverride,
    reasoningOverride: reasoningOverride,
  );

  @override
  void onClose() {
    slugController.dispose();
    titleController.dispose();
    commitController.dispose();
    descriptionController.dispose();
    super.onClose();
  }

  List<String> _buildOwnerOptions() {
    final normalized = workerHandles
        .map((handle) => handle.trim())
        .where((handle) => handle.isNotEmpty)
        .toSet()
        .toList();

    List<String> handles;
    if (normalized.isEmpty) {
      handles = List<String>.generate(
        _workerHandleCount,
        (index) => 'ws${index + 1}',
      );
    } else {
      final wsHandles = <String>[];
      final otherHandles = <String>[];
      for (final handle in normalized) {
        final lower = handle.toLowerCase();
        if (lower.startsWith('ws')) {
          wsHandles.add(handle);
        } else {
          otherHandles.add(handle);
        }
      }
      wsHandles.sort((a, b) {
        final numA = int.tryParse(a.toLowerCase().substring(2)) ?? 0;
        final numB = int.tryParse(b.toLowerCase().substring(2)) ?? 0;
        return numA.compareTo(numB);
      });
      otherHandles.sort((a, b) => a.toLowerCase().compareTo(b.toLowerCase()));
      handles = <String>[...wsHandles, ...otherHandles];
    }

    return <String>['Orchestrator', ...handles, 'Quality Assurance'];
  }

  List<String> get reasoningOptions {
    final model = modelOverride ?? defaultWorkerModel ?? '';
    final lower = model.toLowerCase();
    final isMini = lower.contains('codex-mini');
    final isMax = lower.contains('codex-max');
    if (isMini) {
      return const <String>['medium', 'high'];
    }
    if (isMax) {
      return const <String>['low', 'medium', 'high', 'xhigh'];
    }
    return const <String>['low', 'medium', 'high'];
  }

  void updateModelOverride(String? value) {
    modelOverride = value;
    // reset reasoning if incompatible
    if (modelOverride != null &&
        modelOverride!.toLowerCase().contains('codex-mini') &&
        reasoningOverride == 'low') {
      reasoningOverride = 'medium';
    }
    update();
  }

  void updateReasoningOverride(String? value) {
    reasoningOverride = value;
    update();
  }
}

String _formatReasoningLabel(String value) {
  if (value.isEmpty) return value;
  return '${value[0].toUpperCase()}${value.substring(1)}';
}

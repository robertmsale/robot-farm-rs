import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;

import 'task_edit_payloads.dart';

Future<TaskGroupEditPayload?> showTaskGroupEditorSheet(
  BuildContext context, {
  robot_farm_api.TaskGroup? group,
  bool isCreate = false,
}) {
  return showModalBottomSheet<TaskGroupEditPayload>(
    context: context,
    isScrollControlled: true,
    builder: (_) {
      return GetBuilder<TaskGroupEditorController>(
        init: TaskGroupEditorController(group: group),
        builder: (form) {
          final padding = EdgeInsets.only(
            bottom: MediaQuery.of(context).viewInsets.bottom,
          );

          return Padding(
            padding: padding,
            child: FractionallySizedBox(
              heightFactor: 0.85,
              child: Padding(
                padding: const EdgeInsets.all(24),
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
                      decoration: const InputDecoration(
                        labelText: 'Slug',
                        border: OutlineInputBorder(),
                      ),
                    ),
                    const SizedBox(height: 12),
                    TextField(
                      controller: form.titleController,
                      decoration: const InputDecoration(
                        labelText: 'Title',
                        border: OutlineInputBorder(),
                      ),
                    ),
                    const SizedBox(height: 12),
                    TextField(
                      controller: form.descriptionController,
                      maxLines: 4,
                      decoration: const InputDecoration(
                        labelText: 'Description',
                        border: OutlineInputBorder(),
                      ),
                    ),
                    const Spacer(),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.end,
                      children: [
                        TextButton(
                          onPressed: () => Navigator.of(context).maybePop(),
                          child: const Text('Cancel'),
                        ),
                        const SizedBox(width: 12),
                        FilledButton(
                          onPressed: form.isValid
                              ? () => Navigator.of(context).pop(
                                    form.buildPayload(),
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
          );
        },
      );
    },
  );
}

Future<TaskEditPayload?> showTaskEditorSheet(
  BuildContext context, {
  robot_farm_api.Task? task,
  bool isCreate = false,
}) {
  return showModalBottomSheet<TaskEditPayload>(
    context: context,
    isScrollControlled: true,
    builder: (_) {
      return GetBuilder<TaskEditorController>(
        init: TaskEditorController(task: task),
        builder: (form) {
          final padding = EdgeInsets.only(
            bottom: MediaQuery.of(context).viewInsets.bottom,
          );

          return Padding(
            padding: padding,
            child: FractionallySizedBox(
              heightFactor: 0.9,
              child: Padding(
                padding: const EdgeInsets.all(24),
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
                      decoration: const InputDecoration(
                        labelText: 'Slug',
                        border: OutlineInputBorder(),
                      ),
                    ),
                    const SizedBox(height: 12),
                    TextField(
                      controller: form.titleController,
                      decoration: const InputDecoration(
                        labelText: 'Title',
                        border: OutlineInputBorder(),
                      ),
                    ),
                    const SizedBox(height: 12),
                    TextField(
                      controller: form.commitController,
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
                    TextField(
                      controller: form.ownerController,
                      decoration: const InputDecoration(
                        labelText: 'Owner',
                        border: OutlineInputBorder(),
                      ),
                    ),
                    const Spacer(),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.end,
                      children: [
                        TextButton(
                          onPressed: () => Navigator.of(context).maybePop(),
                          child: const Text('Cancel'),
                        ),
                        const SizedBox(width: 12),
                        FilledButton(
                          onPressed: form.isValid
                              ? () => Navigator.of(context).pop(
                                    form.buildPayload(),
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
    descriptionController = TextEditingController(text: group?.description ?? '')
      ..addListener(_invalidate);
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
  TaskEditorController({this.task});

  final robot_farm_api.Task? task;
  late final TextEditingController slugController;
  late final TextEditingController titleController;
  late final TextEditingController commitController;
  late final TextEditingController ownerController;
  late robot_farm_api.TaskStatus selectedStatus;

  @override
  void onInit() {
    super.onInit();
    slugController = TextEditingController(text: task?.slug ?? '')
      ..addListener(_invalidate);
    titleController = TextEditingController(text: task?.title ?? '')
      ..addListener(_invalidate);
    commitController = TextEditingController(text: task?.commitHash ?? '')
      ..addListener(_invalidate);
    ownerController = TextEditingController(text: task?.owner ?? '')
      ..addListener(_invalidate);
    selectedStatus = task?.status ?? robot_farm_api.TaskStatus.ready;
  }

  void updateStatus(robot_farm_api.TaskStatus? status) {
    if (status == null) {
      return;
    }
    selectedStatus = status;
    update();
  }

  void _invalidate() => update();

  bool get isValid =>
      slugController.text.trim().isNotEmpty &&
      titleController.text.trim().isNotEmpty &&
      ownerController.text.trim().isNotEmpty;

  TaskEditPayload buildPayload() => TaskEditPayload(
        slug: slugController.text.trim(),
        title: titleController.text.trim(),
        commitHash: commitController.text.trim().isEmpty
            ? null
            : commitController.text.trim(),
        status: selectedStatus,
        owner: ownerController.text.trim(),
      );

  @override
  void onClose() {
    slugController.dispose();
    titleController.dispose();
    commitController.dispose();
    ownerController.dispose();
    super.onClose();
  }
}

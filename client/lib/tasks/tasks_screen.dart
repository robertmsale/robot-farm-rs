import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;

import 'task_edit_sheets.dart';
import 'tasks_controller.dart';

class TasksScreen extends GetView<TasksController> {
  const TasksScreen({super.key});

  Future<void> _editGroup(
    BuildContext context,
    robot_farm_api.TaskGroup group,
  ) async {
    final result = await showTaskGroupEditorSheet(
      context,
      group: group,
      isCreate: false,
    );
    if (result == null) return;

    try {
      if (result.action == TaskGroupEditorAction.delete) {
        await controller.deleteTaskGroup(group.id);
        Get.snackbar('Task group deleted', '${group.title} removed.');
      } else {
        final payload = result.payload!;
        await controller.applyGroupEdit(group.id, payload);
        Get.snackbar('Task group updated', '${group.title} saved.');
      }
    } catch (error) {
      Get.snackbar('Update failed', '$error');
    }
  }

  Future<void> _editTask(BuildContext context, robot_farm_api.Task task) async {
    final result = await showTaskEditorSheet(
      context,
      task: task,
      isCreate: false,
      workerHandles: controller.availableWorkerHandles,
    );
    if (result == null) return;

    try {
      if (result.action == TaskEditorAction.delete) {
        await controller.deleteTask(task.id);
        Get.snackbar('Task deleted', '${task.title} removed.');
      } else {
        final payload = result.payload!;
        await controller.applyTaskEdit(task.id, payload);
        Get.snackbar('Task updated', '${task.title} saved.');
      }
    } catch (error) {
      Get.snackbar('Update failed', '$error');
    }
  }

  Future<void> _createGroup(BuildContext context) async {
    final result = await showTaskGroupEditorSheet(context, isCreate: true);
    if (result == null || result.action != TaskGroupEditorAction.save) {
      return;
    }
    final payload = result.payload!;

    try {
      await controller.createTaskGroup(payload);
      await controller.refreshTaskGroups();
      Get.snackbar('Task group created', '${payload.title} added.');
    } catch (error) {
      Get.snackbar('Creation failed', '$error');
    }
  }

  Future<void> _createTask(BuildContext context) async {
    final group = controller.activeGroup;
    if (group == null) {
      Get.snackbar('No group selected', 'Select a task group first.');
      return;
    }

    final result = await showTaskEditorSheet(
      context,
      isCreate: true,
      workerHandles: controller.availableWorkerHandles,
    );
    if (result == null || result.action != TaskEditorAction.save) {
      return;
    }
    final payload = result.payload!;

    try {
      await controller.createTaskForActiveGroup(payload);
      Get.snackbar('Task created', '${payload.title} added to ${group.title}.');
    } catch (error) {
      Get.snackbar('Creation failed', '$error');
    }
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Scaffold(
      appBar: AppBar(
        title: Obx(
          () => Text(
            controller.isViewingGroups
                ? 'Task Groups'
                : controller.activeGroup?.title ?? 'Tasks',
          ),
        ),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () {
            if (controller.isViewingGroups) {
              Get.back<void>();
            } else {
              controller.goBackToGroups();
            }
          },
        ),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            tooltip: 'Refresh',
            onPressed: () {
              if (controller.isViewingGroups) {
                controller.refreshTaskGroups();
              } else {
                controller.refreshActiveGroupTasks();
              }
            },
          ),
        ],
      ),
      body: Obx(
        () => controller.isViewingGroups
            ? _buildGroupPane(context)
            : _buildTaskPane(context, theme),
      ),
      floatingActionButton: Obx(() {
        final isGroups = controller.isViewingGroups;
        final icon = isGroups ? Icons.add : Icons.add_task;
        final tooltip = isGroups
            ? 'Create task group'
            : 'Create task in this group';
        final handler = isGroups
            ? () => _createGroup(context)
            : () => _createTask(context);
        return FloatingActionButton(
          onPressed: handler,
          tooltip: tooltip,
          child: Icon(icon),
        );
      }),
    );
  }

  Widget _buildGroupPane(BuildContext context) {
    return Obx(() {
      final filteredGroups = controller.filteredTaskGroups;
      final hasAnyGroups = controller.taskGroups.isNotEmpty;
      final isLoading = controller.isLoadingGroups.value;
      final error = controller.error.value;

      Widget content;
      if (!hasAnyGroups) {
        if (isLoading) {
          content = const Center(child: CircularProgressIndicator());
        } else if (error != null) {
          content = _StateMessage(
            icon: Icons.error_outline,
            message: error,
            actionLabel: 'Try again',
            onAction: controller.refreshTaskGroups,
          );
        } else {
          content = _StateMessage(
            icon: Icons.list_alt,
            message: 'No task groups found yet.',
            actionLabel: 'Refresh',
            onAction: controller.refreshTaskGroups,
          );
        }
      } else if (filteredGroups.isEmpty) {
        content = const _EmptyFilterResultMessage(
          icon: Icons.search_off,
          message: 'No task groups match your filters.',
        );
      } else {
        content = RefreshIndicator(
          onRefresh: controller.refreshTaskGroups,
          child: _TaskGroupList(
            groups: filteredGroups,
            onEdit: (group) => _editGroup(context, group),
            onOpen: controller.selectGroup,
          ),
        );
      }

      return Column(
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(16, 16, 16, 8),
            child: Column(
              children: [
                TextField(
                  onChanged: controller.updateGroupSearch,
                  decoration: const InputDecoration(
                    labelText: 'Filter by name or slug',
                    prefixIcon: Icon(Icons.search),
                    border: OutlineInputBorder(),
                  ),
                ),
                const SizedBox(height: 8),
                DropdownButtonFormField<robot_farm_api.TaskGroupStatus?>(
                  initialValue: controller.groupStatusFilter.value,
                  items: [
                    const DropdownMenuItem<robot_farm_api.TaskGroupStatus?>(
                      value: null,
                      child: Text('All statuses'),
                    ),
                    ...robot_farm_api.TaskGroupStatus.values.map(
                      (status) => DropdownMenuItem(
                        value: status,
                        child: Text(status.value),
                      ),
                    ),
                  ],
                  onChanged: controller.updateGroupStatusFilter,
                  decoration: const InputDecoration(
                    labelText: 'Status',
                    border: OutlineInputBorder(),
                  ),
                ),
              ],
            ),
          ),
          if (error != null && hasAnyGroups)
            _ErrorBanner(message: error, onRetry: controller.refreshTaskGroups),
          Expanded(child: content),
        ],
      );
    });
  }

  Widget _buildTaskPane(BuildContext context, ThemeData theme) {
    return Obx(() {
      final group = controller.activeGroup;
      final filteredTasks = controller.activeGroupTasks;
      final hasAnyTasks = controller.tasks.isNotEmpty;
      final isLoading = controller.isLoadingTasks.value;
      final error = controller.taskError.value;

      if (group == null) {
        return const Center(child: Text('Select a task group to view tasks.'));
      }

      Widget content;
      if (!hasAnyTasks) {
        if (isLoading) {
          content = const Center(child: CircularProgressIndicator());
        } else if (error != null) {
          content = _StateMessage(
            icon: Icons.error_outline,
            message: error,
            actionLabel: 'Try again',
            onAction: controller.refreshActiveGroupTasks,
          );
        } else {
          content = _StateMessage(
            icon: Icons.task_alt,
            message: 'No tasks in ${group.title} yet.',
            actionLabel: 'Refresh',
            onAction: controller.refreshActiveGroupTasks,
          );
        }
      } else if (filteredTasks.isEmpty) {
        content = const _EmptyFilterResultMessage(
          icon: Icons.search_off,
          message: 'No tasks match your filters.',
        );
      } else {
        content = RefreshIndicator(
          onRefresh: controller.refreshActiveGroupTasks,
          child: _TaskListView(
            group: group,
            tasks: filteredTasks,
            onEdit: (task) => _editTask(context, task),
          ),
        );
      }

      final ownerFilterItems = <DropdownMenuItem<String?>>[
        const DropdownMenuItem<String?>(value: null, child: Text('All owners')),
        ...controller.ownerFilterOptions.map(
          (owner) =>
              DropdownMenuItem<String?>(value: owner, child: Text(owner)),
        ),
      ];

      return Column(
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(16, 16, 16, 8),
            child: Column(
              children: [
                TextField(
                  onChanged: controller.updateTaskSearch,
                  decoration: const InputDecoration(
                    labelText: 'Filter by title or slug',
                    prefixIcon: Icon(Icons.search),
                    border: OutlineInputBorder(),
                  ),
                ),
                const SizedBox(height: 8),
                Row(
                  children: [
                    Expanded(
                      child:
                          DropdownButtonFormField<robot_farm_api.TaskStatus?>(
                            initialValue: controller.taskStatusFilter.value,
                            items: [
                              const DropdownMenuItem<
                                robot_farm_api.TaskStatus?
                              >(value: null, child: Text('All statuses')),
                              ...robot_farm_api.TaskStatus.values.map(
                                (status) => DropdownMenuItem(
                                  value: status,
                                  child: Text(status.value),
                                ),
                              ),
                            ],
                            onChanged: controller.updateTaskStatusFilter,
                            decoration: const InputDecoration(
                              labelText: 'Status',
                              border: OutlineInputBorder(),
                            ),
                          ),
                    ),
                    const SizedBox(width: 12),
                    Expanded(
                      child: DropdownButtonFormField<String?>(
                        initialValue: controller.taskOwnerFilter.value,
                        items: ownerFilterItems,
                        onChanged: controller.updateTaskOwnerFilter,
                        decoration: const InputDecoration(
                          labelText: 'Owner',
                          border: OutlineInputBorder(),
                        ),
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ),
          if (error != null && hasAnyTasks)
            _ErrorBanner(
              message: error,
              onRetry: controller.refreshActiveGroupTasks,
            ),
          Expanded(child: content),
        ],
      );
    });
  }
}

class _TaskGroupList extends StatelessWidget {
  const _TaskGroupList({
    required this.groups,
    required this.onEdit,
    required this.onOpen,
  });

  final List<robot_farm_api.TaskGroup> groups;
  final Future<void> Function(robot_farm_api.TaskGroup group) onEdit;
  final Future<void> Function(int groupId) onOpen;

  @override
  Widget build(BuildContext context) {
    if (groups.isEmpty) {
      return const Center(child: Text('No task groups yet.'));
    }

    return ListView.separated(
      padding: const EdgeInsets.all(16),
      itemCount: groups.length,
      physics: const AlwaysScrollableScrollPhysics(
        parent: BouncingScrollPhysics(),
      ),
      separatorBuilder: (_, __) => const SizedBox(height: 12),
      itemBuilder: (context, index) {
        final group = groups[index];
        return Card(
          child: ListTile(
            onTap: () async {
              await onOpen(group.id);
            },
            title: Text(group.title),
            subtitle: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('Slug: ${group.slug}'),
                const SizedBox(height: 4),
                Text(
                  group.description,
                  maxLines: 2,
                  overflow: TextOverflow.ellipsis,
                ),
              ],
            ),
            trailing: IconButton(
              tooltip: 'Edit group',
              icon: const Icon(Icons.edit),
              onPressed: () async {
                await onEdit(group);
              },
            ),
          ),
        );
      },
    );
  }
}

class _TaskListView extends StatelessWidget {
  const _TaskListView({
    required this.group,
    required this.tasks,
    required this.onEdit,
  });

  final robot_farm_api.TaskGroup group;
  final List<robot_farm_api.Task> tasks;
  final Future<void> Function(robot_farm_api.Task task) onEdit;

  @override
  Widget build(BuildContext context) {
    if (tasks.isEmpty) {
      return Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(group.title, style: Theme.of(context).textTheme.headlineSmall),
            const SizedBox(height: 8),
            Text(
              group.description,
              style: Theme.of(context).textTheme.bodyMedium,
            ),
            const Spacer(),
            const Center(child: Text('No tasks in this group yet.')),
          ],
        ),
      );
    }

    return ListView.separated(
      padding: const EdgeInsets.all(16),
      itemCount: tasks.length,
      physics: const AlwaysScrollableScrollPhysics(
        parent: BouncingScrollPhysics(),
      ),
      separatorBuilder: (_, __) => const SizedBox(height: 12),
      itemBuilder: (context, index) {
        final task = tasks[index];
        return Card(
          child: ListTile(
            title: Text(task.title),
            subtitle: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('Slug: ${task.slug}'),
                const SizedBox(height: 4),
                Text('Status: ${task.status.value} • Owner: ${task.owner}'),
                if (task.commitHash != null && task.commitHash!.isNotEmpty)
                  Padding(
                    padding: const EdgeInsets.only(top: 4),
                    child: Text('Commit: ${task.commitHash}'),
                  ),
              ],
            ),
            trailing: IconButton(
              tooltip: 'Edit task',
              icon: const Icon(Icons.edit),
              onPressed: () async {
                await onEdit(task);
              },
            ),
          ),
        );
      },
    );
  }
}

class _EmptyFilterResultMessage extends StatelessWidget {
  const _EmptyFilterResultMessage({required this.icon, required this.message});

  final IconData icon;
  final String message;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(icon, size: 64, color: theme.colorScheme.secondary),
          const SizedBox(height: 12),
          Text(
            message,
            style: theme.textTheme.bodyLarge,
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 4),
          Text(
            'Adjust your filters to see more results.',
            style: theme.textTheme.bodySmall,
            textAlign: TextAlign.center,
          ),
        ],
      ),
    );
  }
}

class _StateMessage extends StatelessWidget {
  const _StateMessage({
    required this.icon,
    required this.message,
    required this.actionLabel,
    required this.onAction,
  });

  final IconData icon;
  final String message;
  final String actionLabel;
  final VoidCallback onAction;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(icon, size: 64, color: theme.colorScheme.primary),
          const SizedBox(height: 16),
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 24),
            child: Text(
              message,
              textAlign: TextAlign.center,
              style: theme.textTheme.titleMedium,
            ),
          ),
          const SizedBox(height: 16),
          FilledButton(onPressed: onAction, child: Text(actionLabel)),
        ],
      ),
    );
  }
}

class _ErrorBanner extends StatelessWidget {
  const _ErrorBanner({required this.message, this.onRetry});

  final String message;
  final VoidCallback? onRetry;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final color = theme.colorScheme.errorContainer;
    final onColor = theme.colorScheme.onErrorContainer;

    return Container(
      width: double.infinity,
      color: color,
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      child: Row(
        children: [
          Icon(Icons.warning_amber, color: onColor),
          const SizedBox(width: 8),
          Expanded(
            child: Text(
              message,
              style: theme.textTheme.bodyMedium?.copyWith(color: onColor),
            ),
          ),
          if (onRetry != null)
            TextButton(onPressed: onRetry, child: const Text('Retry')),
        ],
      ),
    );
  }
}

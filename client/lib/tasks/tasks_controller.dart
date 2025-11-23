import 'package:flutter/foundation.dart';
import 'package:get/get.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;

import 'task_edit_payloads.dart';

class TasksController extends GetxController {
  TasksController(this._baseUrlProvider, this._workerHandlesProvider);

  final String? Function() _baseUrlProvider;
  final List<String> Function()? _workerHandlesProvider;

  final RxList<robot_farm_api.TaskGroup> taskGroups =
      <robot_farm_api.TaskGroup>[].obs;
  final RxList<robot_farm_api.Task> tasks = <robot_farm_api.Task>[].obs;
  final RxnInt activeGroupId = RxnInt();

  final RxString groupSearchQuery = ''.obs;
  final Rx<robot_farm_api.TaskGroupStatus?> groupStatusFilter =
      Rx<robot_farm_api.TaskGroupStatus?>(null);

  final RxString taskSearchQuery = ''.obs;
  final Rx<robot_farm_api.TaskStatus?> taskStatusFilter =
      Rx<robot_farm_api.TaskStatus?>(null);
  final RxnString taskOwnerFilter = RxnString(null);

  final RxBool isLoadingGroups = false.obs;
  final RxBool isLoadingTasks = false.obs;
  final RxnString error = RxnString();
  final RxnString taskError = RxnString();

  robot_farm_api.DefaultApi? _cachedApi;
  String? _cachedBaseUrl;

  @override
  void onReady() {
    super.onReady();
    refreshTaskGroups();
  }

  bool get isViewingGroups => activeGroupId.value == null;

  robot_farm_api.TaskGroup? get activeGroup {
    final id = activeGroupId.value;
    if (id == null) {
      return null;
    }
    try {
      return taskGroups.firstWhere((group) => group.id == id);
    } catch (_) {
      return null;
    }
  }

  List<robot_farm_api.TaskGroup> get filteredTaskGroups {
    final query = groupSearchQuery.value.toLowerCase().trim();
    final status = groupStatusFilter.value;

    return taskGroups.where((group) {
      final matchesQuery =
          query.isEmpty ||
          group.title.toLowerCase().contains(query) ||
          group.slug.toLowerCase().contains(query);
      final matchesStatus = status == null || group.status == status;
      return matchesQuery && matchesStatus;
    }).toList();
  }

  List<robot_farm_api.Task> get activeGroupTasks {
    final id = activeGroupId.value;
    if (id == null) {
      return const <robot_farm_api.Task>[];
    }
    final query = taskSearchQuery.value.toLowerCase().trim();
    final statusFilter = taskStatusFilter.value;
    final ownerFilterValue = taskOwnerFilter.value?.trim();

    return tasks.where((task) {
      if (task.groupId != id) {
        return false;
      }
      final matchesQuery =
          query.isEmpty ||
          task.title.toLowerCase().contains(query) ||
          task.slug.toLowerCase().contains(query);
      final matchesStatus = statusFilter == null || task.status == statusFilter;
      final matchesOwner =
          ownerFilterValue == null ||
          ownerFilterValue.isEmpty ||
          _ownerMatches(task.owner, ownerFilterValue);
      return matchesQuery && matchesStatus && matchesOwner;
    }).toList();
  }

  List<String> get ownerFilterOptions {
    final workerHandles = <String>{};
    for (final task in tasks) {
      final owner = task.owner.trim();
      if (_isWorkerHandle(owner)) {
        workerHandles.add(owner.toLowerCase());
      }
    }

    if (workerHandles.isEmpty) {
      for (var i = 1; i <= 6; i++) {
        workerHandles.add('ws$i');
      }
    }

    final sortedHandles = workerHandles.toList()
      ..sort((a, b) {
        final numA = int.tryParse(a.substring(2)) ?? 0;
        final numB = int.tryParse(b.substring(2)) ?? 0;
        return numA.compareTo(numB);
      });

    final extras =
        tasks
            .map((task) => task.owner.trim())
            .where(
              (owner) =>
                  owner.isNotEmpty &&
                  !_isWorkerHandle(owner) &&
                  !_isOrchestrator(owner) &&
                  !_isQa(owner),
            )
            .toSet()
            .toList()
          ..sort((a, b) => a.toLowerCase().compareTo(b.toLowerCase()));

    final result = <String>[];
    result.add('Orchestrator');
    result.addAll(sortedHandles);
    result.add('Quality Assurance');
    result.addAll(extras);
    return result;
  }

  void updateGroupSearch(String value) {
    groupSearchQuery.value = value;
  }

  void updateGroupStatusFilter(robot_farm_api.TaskGroupStatus? status) {
    groupStatusFilter.value = status;
  }

  void updateTaskSearch(String value) {
    taskSearchQuery.value = value;
  }

  void updateTaskStatusFilter(robot_farm_api.TaskStatus? status) {
    taskStatusFilter.value = status;
  }

  void updateTaskOwnerFilter(String? owner) {
    if (owner == null || owner.isEmpty) {
      taskOwnerFilter.value = null;
      return;
    }
    taskOwnerFilter.value = owner;
  }

  Future<void> refreshTaskGroups() async {
    final api = _apiOrNull(setGroupError: true);
    if (api == null) {
      return;
    }

    isLoadingGroups.value = true;
    error.value = null;
    try {
      final groups =
          await api.listTaskGroups() ?? const <robot_farm_api.TaskGroup>[];
      final previousId = activeGroupId.value;
      taskGroups.assignAll(groups);

      if (previousId != null && groups.any((group) => group.id == previousId)) {
        activeGroupId.value = previousId;
        await _loadTasksForGroup(previousId);
      } else {
        activeGroupId.value = null;
        tasks.clear();
      }
    } on robot_farm_api.ApiException catch (err) {
      error.value =
          err.message ?? 'Failed to load task groups (HTTP ${err.code}).';
    } catch (err) {
      error.value = 'Failed to load task groups: $err';
    } finally {
      isLoadingGroups.value = false;
    }
  }

  Future<void> selectGroup(int groupId) async {
    activeGroupId.value = groupId;
    await _loadTasksForGroup(groupId);
  }

  void goBackToGroups() {
    activeGroupId.value = null;
    taskError.value = null;
    tasks.clear();
  }

  Future<void> refreshActiveGroupTasks() async {
    final id = activeGroupId.value;
    if (id == null) {
      return;
    }
    await _loadTasksForGroup(id, force: true);
  }

  Future<void> createTaskGroup(TaskGroupEditPayload payload) async {
    final api = _apiOrThrow();
    final input = robot_farm_api.TaskGroupCreateInput(
      slug: payload.slug,
      title: payload.title,
      description: payload.description,
    );

    try {
      final created = await api.createTaskGroup(input);
      if (created == null) {
        throw Exception('Server returned an empty response.');
      }
      taskGroups.add(created);
      taskGroups.sort((a, b) => a.id.compareTo(b.id));
      taskGroups.refresh();
      activeGroupId.value = created.id;
      tasks.clear();
    } on robot_farm_api.ApiException catch (err) {
      throw Exception(
        err.message ?? 'Failed to create task group (HTTP ${err.code}).',
      );
    } catch (err) {
      throw Exception('Failed to create task group: $err');
    }
  }

  Future<void> deleteTaskGroup(int groupId) async {
    final api = _apiOrThrow();
    try {
      await api.deleteTaskGroup(groupId);
      taskGroups.removeWhere((group) => group.id == groupId);
      if (activeGroupId.value == groupId) {
        activeGroupId.value = null;
        tasks.clear();
      }
      taskGroups.refresh();
    } on robot_farm_api.ApiException catch (err) {
      throw Exception(
        err.message ?? 'Failed to delete task group (HTTP ${err.code}).',
      );
    } catch (err) {
      throw Exception('Failed to delete task group: $err');
    }
  }

  Future<void> createTaskForActiveGroup(TaskEditPayload payload) async {
    final groupId = activeGroupId.value;
    if (groupId == null) {
      throw Exception('Select a task group before creating tasks.');
    }

    if (activeGroup == null) {
      await refreshTaskGroups();
      if (activeGroup == null) {
        throw Exception('Selected task group is no longer available.');
      }
    }

    final api = _apiOrThrow();
    final input = robot_farm_api.TaskCreateInput(
      groupId: groupId,
      slug: payload.slug,
      title: payload.title,
      commitHash: payload.commitHash,
      status: payload.status,
      owner: payload.owner,
      description: payload.description,
    );

    try {
      debugPrint(
        'Creating task in group $groupId with payload: '
        '${payload.toStringForLog()}',
      );
      final created = await api.createTask(input);
      if (created == null) {
        throw Exception('Server returned an empty response.');
      }

      if (created.groupId == groupId) {
        tasks.add(created);
        tasks.sort((a, b) => a.id.compareTo(b.id));
        tasks.refresh();
      }
    } on robot_farm_api.ApiException catch (err) {
      if (err.code == 400) {
        await refreshTaskGroups();
      }
      throw Exception(
        err.message ?? 'Failed to create task (HTTP ${err.code}).',
      );
    } catch (err) {
      throw Exception('Failed to create task: $err');
    }
  }

  Future<void> setStrategyForGroup(
    robot_farm_api.Strategy strategy,
    int groupId,
  ) async {
    final api = _apiOrThrow();
    final payload = robot_farm_api.ActiveStrategy(
      id: strategy,
      focus: [groupId],
    );
    await api.updateActiveStrategy(payload);
  }

  Future<void> enqueueOrchestratorSeed(
    String groupTitle,
    robot_farm_api.Strategy strategy,
  ) async {
    final api = _apiOrNull(setGroupError: true);
    if (api == null) return;
    final client = api.apiClient;
    final payload = {
      'from': 'System',
      'to': 'Orchestrator',
      'message':
          'Strategy set to ${strategy.value}; new task added in group "$groupTitle".',
    };
    await client.invokeAPI(
      '/message_queue',
      'POST',
      const <robot_farm_api.QueryParam>[],
      payload,
      <String, String>{},
      <String, String>{},
      'application/json',
    );
  }

  Future<void> deleteTask(int taskId) async {
    final api = _apiOrThrow();
    try {
      await api.deleteTask(taskId);
      tasks.removeWhere((task) => task.id == taskId);
      tasks.refresh();
    } on robot_farm_api.ApiException catch (err) {
      throw Exception(
        err.message ?? 'Failed to delete task (HTTP ${err.code}).',
      );
    } catch (err) {
      throw Exception('Failed to delete task: $err');
    }
  }

  Future<void> applyGroupEdit(int groupId, TaskGroupEditPayload payload) async {
    final api = _apiOrThrow();
    final input = robot_farm_api.TaskGroupUpdateInput(
      slug: payload.slug,
      title: payload.title,
      description: payload.description,
    );

    try {
      final updated = await api.updateTaskGroup(groupId, input);
      if (updated == null) {
        throw Exception('Server returned an empty response.');
      }

      final index = taskGroups.indexWhere((group) => group.id == groupId);
      if (index != -1) {
        taskGroups[index] = updated;
        taskGroups.refresh();
      }
    } on robot_farm_api.ApiException catch (err) {
      throw Exception(
        err.message ?? 'Failed to update task group (HTTP ${err.code}).',
      );
    } catch (err) {
      throw Exception('Failed to update task group: $err');
    }
  }

  Future<void> applyTaskEdit(int taskId, TaskEditPayload payload) async {
    final api = _apiOrThrow();
    final input = robot_farm_api.TaskUpdateInput(
      slug: payload.slug,
      title: payload.title,
      commitHash: payload.commitHash,
      status: payload.status,
      owner: payload.owner,
      description: payload.description,
    );

    try {
      debugPrint(
        'Updating task $taskId with payload: '
        '${payload.toStringForLog()}',
      );
      final updated = await api.updateTask(taskId, input);
      if (updated == null) {
        throw Exception('Server returned an empty response.');
      }

      final index = tasks.indexWhere((task) => task.id == taskId);
      if (index != -1) {
        if (activeGroupId.value == updated.groupId) {
          tasks[index] = updated;
          tasks.refresh();
        } else {
          tasks.removeAt(index);
        }
      } else if (activeGroupId.value == updated.groupId) {
        tasks.add(updated);
      }
    } on robot_farm_api.ApiException catch (err) {
      throw Exception(
        err.message ?? 'Failed to update task (HTTP ${err.code}).',
      );
    } catch (err) {
      throw Exception('Failed to update task: $err');
    }
  }

  robot_farm_api.DefaultApi? _apiOrNull({bool setGroupError = false}) {
    final baseUrl = _baseUrlProvider();
    if (baseUrl == null) {
      if (setGroupError) {
        error.value = 'Connect to a server to manage tasks.';
      } else {
        taskError.value = 'Connect to a server to manage tasks.';
      }
      return null;
    }

    if (_cachedApi == null || _cachedBaseUrl != baseUrl) {
      final client = robot_farm_api.ApiClient(basePath: baseUrl);
      _cachedApi = robot_farm_api.DefaultApi(client);
      _cachedBaseUrl = baseUrl;
    }
    return _cachedApi;
  }

  robot_farm_api.DefaultApi _apiOrThrow() {
    final api = _apiOrNull(setGroupError: true);
    if (api == null) {
      throw Exception('Connect to a server before performing this action.');
    }
    return api;
  }

  Future<void> _loadTasksForGroup(int groupId, {bool force = false}) async {
    final api = _apiOrNull(setGroupError: false);
    if (api == null) {
      return;
    }

    if (!force && isLoadingTasks.value) {
      return;
    }

    isLoadingTasks.value = true;
    taskError.value = null;

    try {
      final allTasks = await api.listTasks() ?? const <robot_farm_api.Task>[];
      final filtered = allTasks
          .where((task) => task.groupId == groupId)
          .toList();
      tasks.assignAll(filtered);
    } on robot_farm_api.ApiException catch (err) {
      taskError.value =
          err.message ?? 'Failed to load tasks (HTTP ${err.code}).';
      tasks.clear();
    } catch (err) {
      taskError.value = 'Failed to load tasks: $err';
      tasks.clear();
    } finally {
      isLoadingTasks.value = false;
    }
  }

  static bool _ownerMatches(String owner, String filter) {
    final normalizedOwner = owner.trim().toLowerCase();
    final normalizedFilter = filter.trim().toLowerCase();
    if (normalizedFilter == 'qa' || normalizedFilter == 'quality assurance') {
      return normalizedOwner == 'qa' || normalizedOwner == 'quality assurance';
    }
    if (normalizedFilter == 'orchestrator') {
      return normalizedOwner == 'orchestrator';
    }
    return normalizedOwner == normalizedFilter;
  }

  static bool _isWorkerHandle(String owner) {
    return RegExp(r'^ws\d+$', caseSensitive: false).hasMatch(owner.trim());
  }

  static bool _isOrchestrator(String owner) =>
      owner.trim().toLowerCase() == 'orchestrator';

  static bool _isQa(String owner) {
    final lower = owner.trim().toLowerCase();
    return lower == 'qa' || lower == 'quality assurance';
  }

  List<String> get availableWorkerHandles =>
      _workerHandlesProvider?.call() ?? const <String>[];
}

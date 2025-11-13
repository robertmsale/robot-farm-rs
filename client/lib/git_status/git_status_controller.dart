import 'package:get/get.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;

class GitStatusController extends GetxController {
  GitStatusController(this._baseUrlProvider);

  final String? Function() _baseUrlProvider;

  final RxList<robot_farm_api.GitWorktreeStatus> worktrees =
      <robot_farm_api.GitWorktreeStatus>[].obs;
  final RxBool isLoading = false.obs;
  final RxnString error = RxnString();

  robot_farm_api.DefaultApi? _cachedApi;
  String? _cachedBaseUrl;
  final Map<String, robot_farm_api.GitWorktreeStatus> _detailCache = {};

  @override
  void onReady() {
    super.onReady();
    refreshStatuses();
  }

  Future<void> refreshStatuses() async {
    final api = _apiOrNull();
    if (api == null) {
      return;
    }

    isLoading.value = true;
    error.value = null;

    try {
      final summary = await api.getGitStatusSummary();
      worktrees.assignAll(summary?.worktrees ?? const []);
      _detailCache.clear();
    } on robot_farm_api.ApiException catch (err) {
      error.value =
          err.message ?? 'Failed to load git statuses (HTTP ${err.code}).';
    } catch (err) {
      error.value = 'Failed to load git statuses: $err';
    } finally {
      isLoading.value = false;
    }
  }

  bool isFileInteractive(robot_farm_api.GitStatusFileChange change) {
    if (change.statusCode.isEmpty) {
      return false;
    }
    final code = change.statusCode[0];
    if (code == 'R' || code == 'D') {
      return false;
    }
    final hasHunks = change.hunks.isNotEmpty;
    return hasHunks || change.additions > 0 || change.deletions > 0;
  }

  Future<robot_farm_api.GitStatusFileChange?> loadFileWithHunks(
    String worktreeId,
    String path,
  ) async {
    final detail = await _loadWorktreeDetail(worktreeId);
    if (detail == null) {
      return null;
    }
    for (final file in detail.files) {
      if (file.path == path) {
        return file;
      }
    }
    return null;
  }

  Future<robot_farm_api.GitWorktreeStatus?> _loadWorktreeDetail(
    String worktreeId,
  ) async {
    if (_detailCache.containsKey(worktreeId)) {
      return _detailCache[worktreeId];
    }

    final api = _apiOrNull();
    if (api == null) {
      return null;
    }

    try {
      final status = await api.getGitStatusForWorktree(worktreeId);
      if (status != null) {
        _detailCache[worktreeId] = status;
        _replaceWorktree(status);
      }
      return status;
    } on robot_farm_api.ApiException catch (err) {
      error.value =
          err.message ??
          'Failed to load worktree $worktreeId (HTTP ${err.code}).';
    } catch (err) {
      error.value = 'Failed to load worktree $worktreeId: $err';
    }
    return null;
  }

  void _replaceWorktree(robot_farm_api.GitWorktreeStatus status) {
    final index = worktrees.indexWhere((item) => item.id == status.id);
    if (index != -1) {
      worktrees[index] = status;
      worktrees.refresh();
    }
  }

  robot_farm_api.DefaultApi? _apiOrNull() {
    final baseUrl = _baseUrlProvider();
    if (baseUrl == null || baseUrl.isEmpty) {
      error.value = 'Connect to the server first.';
      return null;
    }

    if (_cachedApi != null && _cachedBaseUrl == baseUrl) {
      return _cachedApi;
    }

    final client = robot_farm_api.ApiClient(basePath: baseUrl);
    _cachedApi = robot_farm_api.DefaultApi(client);
    _cachedBaseUrl = baseUrl;
    return _cachedApi;
  }
}

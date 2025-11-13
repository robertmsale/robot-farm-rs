import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;

import 'git_diff_sheet.dart';
import 'git_status_controller.dart';

class GitStatusScreen extends GetView<GitStatusController> {
  const GitStatusScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Git Statuses'),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Get.back(),
        ),
        actions: [
          IconButton(
            tooltip: 'Refresh',
            icon: const Icon(Icons.refresh),
            onPressed: controller.refreshStatuses,
          ),
        ],
      ),
      body: Obx(() {
        if (controller.isLoading.value) {
          return const Center(child: CircularProgressIndicator());
        }

        final message = controller.error.value;
        if (message != null) {
          return _ErrorState(
            message: message,
            onRetry: controller.refreshStatuses,
          );
        }

        final worktrees = controller.worktrees;
        if (worktrees.isEmpty) {
          return const _EmptyState();
        }

        return DefaultTabController(
          length: worktrees.length,
          child: Column(
            children: [
              TabBar(
                isScrollable: true,
                tabs: worktrees
                    .map(
                      (tree) => Tab(
                        child: Row(
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            Text(tree.id),
                            if (tree.isDirty)
                              Padding(
                                padding: const EdgeInsets.only(left: 6),
                                child: Icon(
                                  Icons.circle,
                                  size: 10,
                                  color: Theme.of(context).colorScheme.tertiary,
                                ),
                              ),
                          ],
                        ),
                      ),
                    )
                    .toList(),
              ),
              const SizedBox(height: 12),
              Expanded(
                child: TabBarView(
                  children: worktrees
                      .map(
                        (worktree) => _GitWorktreeTab(
                          worktree: worktree,
                          onOpenFile: (change) =>
                              _openFileDiff(context, worktree, change),
                          canOpenFile: controller.isFileInteractive,
                        ),
                      )
                      .toList(),
                ),
              ),
            ],
          ),
        );
      }),
    );
  }

  Future<void> _openFileDiff(
    BuildContext context,
    robot_farm_api.GitWorktreeStatus worktree,
    robot_farm_api.GitStatusFileChange change,
  ) async {
    if (!controller.isFileInteractive(change)) {
      return;
    }

    Get.dialog(
      const Center(child: CircularProgressIndicator()),
      barrierDismissible: false,
    );

    robot_farm_api.GitStatusFileChange? detailed;
    try {
      detailed = await controller.loadFileWithHunks(worktree.id, change.path);
    } finally {
      if (Get.isDialogOpen == true) {
        Get.back();
      }
    }

    final enriched = detailed;
    final hunks = enriched?.hunks;

    if (enriched == null || hunks == null || hunks.isEmpty) {
      Get.snackbar(
        'Git Status',
        'No diff hunks available for ${change.path}.',
        snackPosition: SnackPosition.BOTTOM,
        duration: const Duration(seconds: 3),
      );
      return;
    }

    if (!context.mounted) {
      return;
    }

    await showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (_) => GitDiffSheet(
        worktreeId: worktree.id,
        filePath: enriched.path,
        statusCode: enriched.statusCode,
        additions: enriched.additions,
        deletions: enriched.deletions,
        hunks: hunks,
      ),
    );
  }
}

class _GitWorktreeTab extends StatelessWidget {
  const _GitWorktreeTab({
    required this.worktree,
    required this.onOpenFile,
    required this.canOpenFile,
  });

  final robot_farm_api.GitWorktreeStatus worktree;
  final void Function(robot_farm_api.GitStatusFileChange change) onOpenFile;
  final bool Function(robot_farm_api.GitStatusFileChange change) canOpenFile;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        _WorktreeHeader(worktree: worktree),
        const SizedBox(height: 12),
        Expanded(
          child: worktree.files.isEmpty
              ? Center(
                  child: Text(
                    'No local changes detected.',
                    style: Theme.of(context).textTheme.bodyMedium,
                  ),
                )
              : ListView.separated(
                  itemCount: worktree.files.length,
                  separatorBuilder: (_, __) => const Divider(height: 1),
                  itemBuilder: (context, index) {
                    final file = worktree.files[index];
                    final interactive = canOpenFile(file);
                    return ListTile(
                      title: Text(file.path),
                      subtitle: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            '${file.statusCode} • +${file.additions} -${file.deletions}',
                          ),
                          if (file.oldPath != null)
                            Text(
                              'Renamed from ${file.oldPath}',
                              style: Theme.of(context).textTheme.labelSmall,
                            ),
                        ],
                      ),
                      trailing: interactive
                          ? const Icon(Icons.chevron_right)
                          : null,
                      enabled: interactive,
                      onTap: interactive ? () => onOpenFile(file) : null,
                    );
                  },
                ),
        ),
      ],
    );
  }
}

class _WorktreeHeader extends StatelessWidget {
  const _WorktreeHeader({required this.worktree});

  final robot_farm_api.GitWorktreeStatus worktree;

  @override
  Widget build(BuildContext context) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(worktree.path, style: Theme.of(context).textTheme.titleSmall),
            const SizedBox(height: 8),
            Wrap(
              spacing: 12,
              runSpacing: 8,
              children: [
                Chip(
                  avatar: const Icon(Icons.call_split, size: 18),
                  label: Text(worktree.branch),
                ),
                Chip(
                  label: Text(
                    worktree.upstream == null
                        ? 'No upstream'
                        : 'Upstream: ${worktree.upstream}',
                  ),
                ),
                Chip(label: Text('Ahead ${worktree.ahead}')),
                Chip(label: Text('Behind ${worktree.behind}')),
                Chip(
                  avatar: Icon(
                    worktree.isDirty ? Icons.warning_amber : Icons.check_circle,
                    color: worktree.isDirty
                        ? Colors.orange.shade700
                        : Colors.green.shade700,
                  ),
                  label: Text(worktree.isDirty ? 'Dirty' : 'Clean'),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class _ErrorState extends StatelessWidget {
  const _ErrorState({required this.message, required this.onRetry});

  final String message;
  final VoidCallback onRetry;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(
              message,
              style: Theme.of(context).textTheme.bodyLarge,
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 16),
            ElevatedButton.icon(
              onPressed: onRetry,
              icon: const Icon(Icons.refresh),
              label: const Text('Retry'),
            ),
          ],
        ),
      ),
    );
  }
}

class _EmptyState extends StatelessWidget {
  const _EmptyState();

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Text(
          'No worktrees detected yet. Kick off some work to see changes here.',
          style: Theme.of(context).textTheme.bodyLarge,
          textAlign: TextAlign.center,
        ),
      ),
    );
  }
}

import 'package:flutter/material.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;

class GitDiffSheet extends StatelessWidget {
  const GitDiffSheet({
    super.key,
    required this.worktreeId,
    required this.filePath,
    required this.statusCode,
    required this.additions,
    required this.deletions,
    required this.hunks,
  });

  final String worktreeId;
  final String filePath;
  final String statusCode;
  final int additions;
  final int deletions;
  final List<robot_farm_api.GitStatusHunk> hunks;

  @override
  Widget build(BuildContext context) {
    final height = MediaQuery.of(context).size.height * 0.8;
    final monospace = const TextStyle(fontFamily: 'RobotoMono');

    return SafeArea(
      child: Padding(
        padding: EdgeInsets.only(
          bottom: MediaQuery.of(context).viewInsets.bottom,
        ),
        child: SizedBox(
          height: height,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          filePath,
                          style: Theme.of(context).textTheme.titleMedium,
                          overflow: TextOverflow.ellipsis,
                        ),
                        const SizedBox(height: 4),
                        Text(
                          '$worktreeId • $statusCode',
                          style: Theme.of(context).textTheme.bodySmall,
                        ),
                      ],
                    ),
                  ),
                  IconButton(
                    tooltip: 'Close',
                    icon: const Icon(Icons.close),
                    onPressed: () => Navigator.of(context).maybePop(),
                  ),
                ],
              ),
              const SizedBox(height: 8),
              Wrap(
                spacing: 12,
                runSpacing: 8,
                children: [
                  _StatChip(
                    label: 'Additions',
                    value: '+$additions',
                    color: Colors.green.shade700,
                  ),
                  _StatChip(
                    label: 'Deletions',
                    value: '-$deletions',
                    color: Colors.red.shade700,
                  ),
                ],
              ),
              const SizedBox(height: 16),
              Expanded(
                child: hunks.isEmpty
                    ? Center(
                        child: Text(
                          'No diff hunks to display.',
                          style: Theme.of(context).textTheme.bodyMedium,
                        ),
                      )
                    : ListView.separated(
                        itemCount: hunks.length,
                        separatorBuilder: (_, __) => const SizedBox(height: 12),
                        itemBuilder: (context, index) {
                          final hunk = hunks[index];
                          return Card(
                            clipBehavior: Clip.antiAlias,
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.stretch,
                              children: [
                                Container(
                                  color: Theme.of(
                                    context,
                                  ).colorScheme.surfaceContainerHighest,
                                  padding: const EdgeInsets.symmetric(
                                    horizontal: 12,
                                    vertical: 8,
                                  ),
                                  child: Text(
                                    hunk.header,
                                    style: monospace.copyWith(
                                      fontWeight: FontWeight.bold,
                                    ),
                                  ),
                                ),
                                const Divider(height: 1),
                                ...hunk.lines.map(
                                  (line) => Container(
                                    color: _lineColor(line),
                                    padding: const EdgeInsets.symmetric(
                                      horizontal: 12,
                                      vertical: 2,
                                    ),
                                    child: Text(
                                      line.isEmpty ? ' ' : line,
                                      style: monospace.copyWith(
                                        color: _lineTextColor(line),
                                      ),
                                    ),
                                  ),
                                ),
                              ],
                            ),
                          );
                        },
                      ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Color? _lineColor(String line) {
    if (line.startsWith('+')) {
      return Colors.green.withValues(alpha: 0.05);
    }
    if (line.startsWith('-')) {
      return Colors.red.withValues(alpha: 0.05);
    }
    return null;
  }

  Color? _lineTextColor(String line) {
    if (line.startsWith('+')) {
      return Colors.green.shade800;
    }
    if (line.startsWith('-')) {
      return Colors.red.shade800;
    }
    return null;
  }
}

class _StatChip extends StatelessWidget {
  const _StatChip({
    required this.label,
    required this.value,
    required this.color,
  });

  final String label;
  final String value;
  final Color color;

  @override
  Widget build(BuildContext context) {
    return Chip(
      label: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(label, style: Theme.of(context).textTheme.labelSmall),
          Text(
            value,
            style: Theme.of(context).textTheme.titleMedium?.copyWith(
              color: color,
              fontWeight: FontWeight.bold,
            ),
          ),
        ],
      ),
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
    );
  }
}

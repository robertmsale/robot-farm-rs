import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;

class QuickTaskSheet extends StatefulWidget {
  const QuickTaskSheet({
    super.key,
    required this.baseUrl,
    required this.taskGroups,
    required this.initialGroupId,
    required this.onGroupPersist,
  });

  final String baseUrl;
  final List<robot_farm_api.TaskGroup> taskGroups;
  final int? initialGroupId;
  final ValueChanged<int?> onGroupPersist;

  @override
  State<QuickTaskSheet> createState() => _QuickTaskSheetState();
}

class _QuickTaskSheetState extends State<QuickTaskSheet> {
  final _slugController = TextEditingController();
  final _titleController = TextEditingController();
  final _descController = TextEditingController();
  bool _enqueueMessage = true;
  bool _isSaving = false;
  String? _slugError;
  int? _selectedGroupId;
  List<String> _existingSlugs = const [];
  List<robot_farm_api.TaskGroup> _groups = const [];

  @override
  void initState() {
    super.initState();
    _groups = widget.taskGroups;
    _selectedGroupId =
        widget.initialGroupId ?? (_groups.isNotEmpty ? _groups.first.id : null);
    _loadExistingSlugs();
    _maybeFetchGroups();
    _slugController.addListener(_validateSlug);
  }

  Future<void> _maybeFetchGroups() async {
    if (_groups.isNotEmpty) return;
    try {
      final client = robot_farm_api.ApiClient(basePath: widget.baseUrl);
      final api = robot_farm_api.DefaultApi(client);
      final fetched =
          await api.listTaskGroups() ?? const <robot_farm_api.TaskGroup>[];
      if (!mounted) return;
      setState(() {
        _groups = fetched;
        _selectedGroupId =
            widget.initialGroupId ??
            (fetched.isNotEmpty ? fetched.first.id : null);
      });
    } catch (_) {
      // ignore; keep as-is
    }
  }

  Future<void> _loadExistingSlugs() async {
    try {
      final client = robot_farm_api.ApiClient(basePath: widget.baseUrl);
      final api = robot_farm_api.DefaultApi(client);
      final tasks = await api.listTasks() ?? const <robot_farm_api.Task>[];
      setState(() {
        _existingSlugs = tasks.map((t) => t.slug.toLowerCase()).toList();
      });
      _validateSlug();
    } catch (_) {
      // best-effort; keep slugs empty
    }
  }

  void _validateSlug() {
    final slug = _slugController.text.trim().toLowerCase();
    if (slug.isEmpty) {
      setState(() => _slugError = 'Slug is required');
      return;
    }
    if (_existingSlugs.contains(slug)) {
      setState(() => _slugError = 'Slug already exists');
      return;
    }
    setState(() => _slugError = null);
  }

  bool get _canSave =>
      !_isSaving &&
      _selectedGroupId != null &&
      _slugError == null &&
      _slugController.text.trim().isNotEmpty &&
      _titleController.text.trim().isNotEmpty;

  Future<void> _save() async {
    if (!_canSave) return;
    setState(() => _isSaving = true);
    final client = robot_farm_api.ApiClient(basePath: widget.baseUrl);
    final api = robot_farm_api.DefaultApi(client);
    final groupId = _selectedGroupId!;
    final input = robot_farm_api.TaskCreateInput(
      groupId: groupId,
      slug: _slugController.text.trim(),
      title: _titleController.text.trim(),
      commitHash: null,
      status: robot_farm_api.TaskStatus.ready,
      owner: 'Orchestrator',
      description: _descController.text.trim(),
      modelOverride: null,
      reasoningOverride: null,
    );
    try {
      await api.createTask(input);
      widget.onGroupPersist(groupId);
      if (_enqueueMessage) {
        await client.invokeAPI(
          '/message_queue',
          'POST',
          const <robot_farm_api.QueryParam>[],
          {
            'from': 'System',
            'to': 'Orchestrator',
            'message':
                'New task "${input.title}" created in group ${input.groupId}.',
          },
          <String, String>{},
          <String, String>{},
          'application/json',
        );
      }
      if (mounted) Navigator.of(context).maybePop(true);
    } on robot_farm_api.ApiException catch (err) {
      Get.snackbar(
        'Failed to create task',
        err.message ?? 'HTTP ${err.code}',
        snackPosition: SnackPosition.BOTTOM,
      );
    } catch (err) {
      Get.snackbar(
        'Failed to create task',
        '$err',
        snackPosition: SnackPosition.BOTTOM,
      );
    } finally {
      if (mounted) setState(() => _isSaving = false);
    }
  }

  @override
  void dispose() {
    _slugController.dispose();
    _titleController.dispose();
    _descController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Padding(
      padding: EdgeInsets.only(
        bottom: MediaQuery.of(context).viewInsets.bottom,
      ),
      child: SingleChildScrollView(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          mainAxisSize: MainAxisSize.min,
          children: [
            Row(
              children: [
                Text('Quick Task', style: theme.textTheme.titleLarge),
                const Spacer(),
                IconButton(
                  icon: const Icon(Icons.close),
                  onPressed: () => Navigator.of(context).maybePop(),
                ),
              ],
            ),
            const SizedBox(height: 12),
            DropdownButtonFormField<int>(
              initialValue: _selectedGroupId,
              items: _groups
                  .map(
                    (g) => DropdownMenuItem(
                      value: g.id,
                      child: Text('${g.title} (${g.slug})'),
                    ),
                  )
                  .toList(),
              onChanged: (val) => setState(() => _selectedGroupId = val),
              decoration: const InputDecoration(
                labelText: 'Task group',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _slugController,
              decoration: InputDecoration(
                labelText: 'Slug',
                border: const OutlineInputBorder(),
                errorText: _slugError,
              ),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _titleController,
              decoration: const InputDecoration(
                labelText: 'Title',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _descController,
              maxLines: 3,
              decoration: const InputDecoration(
                labelText: 'Description',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 12),
            CheckboxListTile(
              value: _enqueueMessage,
              onChanged: (v) =>
                  setState(() => _enqueueMessage = v ?? _enqueueMessage),
              title: const Text('Notify orchestrator about this task'),
              dense: true,
              controlAffinity: ListTileControlAffinity.leading,
            ),
            const SizedBox(height: 12),
            FilledButton.icon(
              onPressed: _canSave ? _save : null,
              icon: _isSaving
                  ? const SizedBox(
                      height: 16,
                      width: 16,
                      child: CircularProgressIndicator(),
                    )
                  : const Icon(Icons.save),
              label: const Text('Save'),
            ),
          ],
        ),
      ),
    );
  }
}

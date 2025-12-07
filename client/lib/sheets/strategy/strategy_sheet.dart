import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;

class StrategySheet extends StatefulWidget {
  const StrategySheet({required this.baseUrlProvider, super.key});

  final String? Function() baseUrlProvider;

  @override
  State<StrategySheet> createState() => _StrategySheetState();
}

class _StrategySheetState extends State<StrategySheet> {
  late robot_farm_api.Strategy _selectedStrategy;
  Set<int> _focusedGroups = <int>{};
  List<robot_farm_api.TaskGroup> _taskGroups =
      const <robot_farm_api.TaskGroup>[];
  bool _isLoading = true;
  bool _isSaving = false;
  String? _error;
  robot_farm_api.DefaultApi? _api;

  @override
  void initState() {
    super.initState();
    _selectedStrategy = robot_farm_api.Strategy.PLANNING;
    _loadInitial();
  }

  bool get _requiresFocusSelection =>
      _selectedStrategy == robot_farm_api.Strategy.MODERATE ||
      _selectedStrategy == robot_farm_api.Strategy.ECONOMICAL;

  bool get _canSubmit {
    if (_requiresFocusSelection) {
      final hasValidGroup = _taskGroups.any(
        (group) => group.slug != 'chores' && _focusedGroups.contains(group.id),
      );
      return hasValidGroup;
    }
    return true;
  }

  Future<void> _loadInitial() async {
    final baseUrl = widget.baseUrlProvider();
    if (baseUrl == null) {
      setState(() {
        _error = 'Connect to a server to manage strategy.';
        _isLoading = false;
      });
      return;
    }

    setState(() {
      _isLoading = true;
      _error = null;
    });

    _api ??= robot_farm_api.DefaultApi(
      robot_farm_api.ApiClient(basePath: baseUrl),
    );

    try {
      final active = await _api!.getActiveStrategy();
      final groups =
          await _api!.listTaskGroups() ?? const <robot_farm_api.TaskGroup>[];
      setState(() {
        _taskGroups = groups;
        _selectedStrategy = active?.id ?? robot_farm_api.Strategy.PLANNING;
        _focusedGroups = active == null
            ? groups.map((group) => group.id).toSet()
            : active.focus.toSet();
        _isLoading = false;
        _error = null;
      });
    } catch (error) {
      setState(() {
        _error = 'Failed to load strategy: $error';
        _isLoading = false;
      });
    }
  }

  Future<void> _saveStrategy() async {
    if (!_canSubmit || _api == null) {
      return;
    }

    setState(() {
      _isSaving = true;
    });

    final focus = _requiresFocusSelection
        ? _focusedGroups.where((id) {
            final group = _groupById(id);
            return group != null && group.slug != 'chores';
          }).toList()
        : <int>[];

    final payload = robot_farm_api.ActiveStrategy(
      id: _selectedStrategy,
      focus: focus,
    );

    try {
      await _api!.updateActiveStrategy(payload);
      if (!mounted) return;
      Navigator.of(context).maybePop();
    } on robot_farm_api.ApiException catch (err) {
      Get.snackbar(
        'Update failed',
        err.message ?? 'Server rejected the request (HTTP ${err.code}).',
      );
    } catch (error) {
      Get.snackbar('Update failed', '$error');
    } finally {
      if (mounted) {
        setState(() {
          _isSaving = false;
        });
      }
    }
  }

  robot_farm_api.TaskGroup? _groupById(int id) {
    for (final group in _taskGroups) {
      if (group.id == id) {
        return group;
      }
    }
    return null;
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return SafeArea(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Row(
              children: [
                Text('Change Strategy', style: theme.textTheme.titleLarge),
                const Spacer(),
                IconButton(
                  icon: const Icon(Icons.close),
                  onPressed: () => Navigator.of(context).maybePop(),
                ),
              ],
            ),
            const SizedBox(height: 16),
            if (_isLoading)
              const Expanded(child: Center(child: CircularProgressIndicator()))
            else if (_error != null)
              Expanded(
                child: Center(
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Text(_error!, textAlign: TextAlign.center),
                      const SizedBox(height: 12),
                      FilledButton.icon(
                        onPressed: _loadInitial,
                        icon: const Icon(Icons.refresh),
                        label: const Text('Retry'),
                      ),
                    ],
                  ),
                ),
              )
            else ...[
              InputDecorator(
                decoration: const InputDecoration(
                  labelText: 'Strategy',
                  border: OutlineInputBorder(),
                ),
                child: DropdownButtonHideUnderline(
                  child: DropdownButton<robot_farm_api.Strategy>(
                    value: _selectedStrategy,
                    isExpanded: true,
                    items: robot_farm_api.Strategy.values
                        .map(
                          (strategy) => DropdownMenuItem(
                            value: strategy,
                            child: Text(strategy.value),
                          ),
                        )
                        .toList(),
                    onChanged: (value) {
                      if (value == null) return;
                      setState(() {
                        _selectedStrategy = value;
                      });
                    },
                  ),
                ),
              ),
              if (_requiresFocusSelection) ...[
                const SizedBox(height: 16),
                Text(
                  'Focus task groups (moderate/economical)',
                  style: theme.textTheme.titleMedium,
                ),
                const SizedBox(height: 8),
                Expanded(
                  child: ListView(
                    children: _taskGroups
                        .where((group) => group.status != robot_farm_api.TaskGroupStatus.done)
                        .map((group) {
                      final disabled = group.slug == 'chores';
                      return CheckboxListTile(
                        title: Text(group.title),
                        subtitle: Text(group.slug),
                        value: _focusedGroups.contains(group.id) && !disabled,
                        onChanged: disabled
                            ? null
                            : (checked) {
                                setState(() {
                                  if (checked ?? false) {
                                    _focusedGroups.add(group.id);
                                  } else {
                                    _focusedGroups.remove(group.id);
                                  }
                                });
                              },
                      );
                    }).toList(),
                  ),
                ),
              ] else
                const Spacer(),
              const SizedBox(height: 16),
              FilledButton.icon(
                icon: _isSaving
                    ? const SizedBox(
                        width: 16,
                        height: 16,
                        child: CircularProgressIndicator(strokeWidth: 2),
                      )
                    : const Icon(Icons.save),
                label: const Text('Save Strategy'),
                onPressed: _isSaving || !_canSubmit ? null : _saveStrategy,
              ),
            ],
          ],
        ),
      ),
    );
  }
}

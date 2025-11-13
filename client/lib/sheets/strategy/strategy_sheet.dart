import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;

class StrategySheet extends StatefulWidget {
  const StrategySheet({
    required this.availableStrategies,
    required this.currentStrategy,
    required this.taskGroups,
    super.key,
  });

  final List<robot_farm_api.Strategy> availableStrategies;
  final robot_farm_api.Strategy currentStrategy;
  final List<robot_farm_api.TaskGroup> taskGroups;

  @override
  State<StrategySheet> createState() => _StrategySheetState();
}

class _StrategySheetState extends State<StrategySheet> {
  late robot_farm_api.Strategy _selectedStrategy;
  late Set<int> _focusedGroups;

  @override
  void initState() {
    super.initState();
    _selectedStrategy = widget.currentStrategy;
    _focusedGroups = widget.taskGroups
        .map((group) => group.id)
        .toSet();
  }

  bool get _requiresFocusSelection =>
      _selectedStrategy == robot_farm_api.Strategy.MODERATE ||
      _selectedStrategy == robot_farm_api.Strategy.ECONOMICAL;

  bool get _canSubmit {
    if (_requiresFocusSelection) {
      final hasValidGroup = widget.taskGroups.any(
        (group) => group.slug != 'chores' && _focusedGroups.contains(group.id),
      );
      return hasValidGroup;
    }
    return true;
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
            InputDecorator(
              decoration: const InputDecoration(
                labelText: 'Strategy',
                border: OutlineInputBorder(),
              ),
              child: DropdownButtonHideUnderline(
                child: DropdownButton<robot_farm_api.Strategy>(
                  value: _selectedStrategy,
                  isExpanded: true,
                  items: widget.availableStrategies
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
                  children: widget.taskGroups.map((group) {
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
              icon: const Icon(Icons.save),
              label: const Text('Save Strategy'),
              onPressed: _canSubmit
                  ? () {
                      Get.snackbar('Not implemented', 'API wiring pending.');
                    }
                  : null,
            ),
          ],
        ),
      ),
    );
  }
}

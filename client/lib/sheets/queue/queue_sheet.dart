import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;

class QueueSheet extends StatefulWidget {
  const QueueSheet({required this.baseUrlProvider, super.key});

  final String? Function() baseUrlProvider;

  @override
  State<QueueSheet> createState() => _QueueSheetState();
}

class _QueueSheetState extends State<QueueSheet> {
  List<robot_farm_api.Message> _messages = const <robot_farm_api.Message>[];
  bool _isLoading = true;
  bool _isMutating = false;
  String? _error;
  robot_farm_api.ApiClient? _client;
  robot_farm_api.DefaultApi? _api;

  @override
  void initState() {
    super.initState();
    _loadMessages();
  }

  robot_farm_api.DefaultApi? _ensureApi({bool notify = true}) {
    final baseUrl = widget.baseUrlProvider();
    if (baseUrl == null) {
      if (notify) {
        Get.snackbar(
          'Not connected',
          'Connect to a server to manage the queue.',
        );
      }
      return null;
    }
    if (_client == null || _client!.basePath != baseUrl) {
      _client = robot_farm_api.ApiClient(basePath: baseUrl);
      _api = robot_farm_api.DefaultApi(_client!);
    }
    return _api;
  }

  Future<void> _loadMessages() async {
    final api = _ensureApi();
    if (api == null) {
      setState(() {
        _isLoading = false;
        _error = 'Connect to a server to load the queue.';
      });
      return;
    }
    setState(() {
      _isLoading = true;
      _error = null;
    });
    try {
      final queue =
          await api.listMessages() ?? const <robot_farm_api.Message>[];
      setState(() {
        _messages = queue;
      });
    } catch (error) {
      setState(() {
        _error = 'Failed to load queue: $error';
      });
    } finally {
      if (mounted) {
        setState(() {
          _isLoading = false;
        });
      }
    }
  }

  Future<void> _clearQueue() async {
    final api = _ensureApi();
    if (api == null) {
      return;
    }
    setState(() => _isMutating = true);
    try {
      await api.deleteAllMessages();
      if (!mounted) return;
      setState(() {
        _messages = const <robot_farm_api.Message>[];
      });
    } catch (error) {
      Get.snackbar('Failed to clear queue', '$error');
    } finally {
      if (mounted) {
        setState(() => _isMutating = false);
      }
    }
  }

  Future<void> _deleteMessage(robot_farm_api.Message message) async {
    final api = _ensureApi();
    final id = message.id;
    if (api == null) {
      return;
    }
    setState(() => _isMutating = true);
    try {
      await api.deleteMessageById(id);
      if (!mounted) return;
      setState(() {
        _messages = List.of(_messages)..removeWhere((m) => m.id == id);
      });
    } catch (error) {
      Get.snackbar('Deletion failed', '$error');
    } finally {
      if (mounted) {
        setState(() => _isMutating = false);
      }
    }
  }

  Future<void> _persistReorder(
    int messageId,
    _AnchorDirective directive,
  ) async {
    final api = _ensureApi();
    if (api == null) {
      throw Exception('Not connected to a server.');
    }
    final client = _client!;
    final path = '/message_queue/$messageId/insert';
    final body = directive.isBefore
        ? {'before': directive.anchorId}
        : {'after': directive.anchorId};
    final response = await client.invokeAPI(
      path,
      'PATCH',
      const <robot_farm_api.QueryParam>[],
      body,
      <String, String>{},
      <String, String>{},
      'application/json',
    );
    if (response.statusCode >= 400) {
      throw robot_farm_api.ApiException(response.statusCode, response.body);
    }
  }

  void _handleReorder(int oldIndex, int newIndex) {
    if (_isMutating || oldIndex == newIndex) {
      setState(() {});
      return;
    }
    var targetIndex = newIndex;
    if (targetIndex > oldIndex) {
      targetIndex -= 1;
    }
    final previousOrder = List.of(_messages);
    final moving = previousOrder[oldIndex];
    final directive = _buildDirective(previousOrder, oldIndex, targetIndex);
    if (directive == null) {
      return;
    }
    final updated = List.of(previousOrder)
      ..removeAt(oldIndex)
      ..insert(targetIndex, moving);
    setState(() {
      _messages = updated;
      _isMutating = true;
    });
    _persistReorder(moving.id, directive)
        .then((_) {
          if (mounted) {
            setState(() => _isMutating = false);
          }
        })
        .catchError((error) {
          if (!mounted) return;
          setState(() {
            _messages = previousOrder;
            _isMutating = false;
          });
          Get.snackbar('Reorder failed', '$error');
        });
  }

  _AnchorDirective? _buildDirective(
    List<robot_farm_api.Message> order,
    int oldIndex,
    int targetIndex,
  ) {
    final remaining = List.of(order)..removeAt(oldIndex);
    if (remaining.isEmpty) {
      return null;
    }
    if (targetIndex >= remaining.length) {
      final anchor = remaining.last.id;
      return _AnchorDirective.after(anchor);
    }
    final anchor = remaining[targetIndex].id;
    return _AnchorDirective.before(anchor);
  }

  String _formatTimestamp(int? epochSeconds) {
    if (epochSeconds == null) {
      return '';
    }
    final dt = DateTime.fromMillisecondsSinceEpoch(
      epochSeconds * 1000,
      isUtc: true,
    ).toLocal();
    return '${dt.hour.toString().padLeft(2, '0')}:${dt.minute.toString().padLeft(2, '0')}';
  }

  Widget _buildBody() {
    if (_isLoading) {
      return const Center(child: CircularProgressIndicator());
    }
    if (_error != null) {
      return Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(_error!, textAlign: TextAlign.center),
            const SizedBox(height: 12),
            FilledButton.icon(
              onPressed: _loadMessages,
              icon: const Icon(Icons.refresh),
              label: const Text('Retry'),
            ),
          ],
        ),
      );
    }
    if (_messages.isEmpty) {
      return RefreshIndicator(
        onRefresh: _loadMessages,
        child: ListView(
          children: const [
            SizedBox(height: 120),
            Center(child: Text('Queue is empty.')),
          ],
        ),
      );
    }
    return RefreshIndicator(
      onRefresh: _loadMessages,
      child: ReorderableListView.builder(
        buildDefaultDragHandles: !_isMutating,
        onReorder: _handleReorder,
        itemCount: _messages.length,
        itemBuilder: (context, index) {
          final entry = _messages[index];
          return Card(
            key: ValueKey(entry.id),
            child: ListTile(
              title: Text(entry.message),
              subtitle: Text(
                'from ${entry.from} → ${entry.to} • ${_formatTimestamp(entry.insertedAt)}',
              ),
              trailing: IconButton(
                icon: const Icon(Icons.delete_outline),
                tooltip: 'Delete message',
                onPressed: _isMutating ? null : () => _deleteMessage(entry),
              ),
            ),
          );
        },
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Row(
              children: [
                Text(
                  'Modify Queue',
                  style: Theme.of(context).textTheme.titleLarge,
                ),
                const Spacer(),
                IconButton(
                  icon: const Icon(Icons.close),
                  onPressed: () => Navigator.of(context).maybePop(),
                ),
              ],
            ),
            const SizedBox(height: 12),
            Expanded(child: _buildBody()),
            const SizedBox(height: 12),
            FilledButton.icon(
              icon: const Icon(Icons.delete_sweep),
              label: const Text('Clear queue'),
              onPressed: _isMutating || _messages.isEmpty ? null : _clearQueue,
            ),
          ],
        ),
      ),
    );
  }
}

class _AnchorDirective {
  const _AnchorDirective.before(this.anchorId) : isBefore = true;
  const _AnchorDirective.after(this.anchorId) : isBefore = false;

  final int anchorId;
  final bool isBefore;
}

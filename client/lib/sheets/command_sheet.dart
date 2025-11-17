import 'package:flutter/material.dart';
import 'package:my_api_client/api.dart' as robot_farm_api;

const _outputMonospaceStyle = TextStyle(fontFamily: 'RobotoMono');

class CommandSheet extends StatefulWidget {
  const CommandSheet({super.key, required this.baseUrl, this.workerId});

  final String baseUrl;
  final int? workerId;

  @override
  State<CommandSheet> createState() => _CommandSheetState();
}

class _CommandSheetState extends State<CommandSheet> {
  final TextEditingController _commandController = TextEditingController();
  final TextEditingController _cwdController = TextEditingController();
  bool _isRunning = false;
  String? _stdout;
  String? _stderr;
  int? _exitCode;
  String? _error;

  @override
  void dispose() {
    _commandController.dispose();
    _cwdController.dispose();
    super.dispose();
  }

  Future<void> _runCommand() async {
    final command = _commandController.text.trim();
    if (command.isEmpty) {
      setState(() {
        _error = 'Enter a command to run.';
      });
      return;
    }

    setState(() {
      _isRunning = true;
      _stdout = null;
      _stderr = null;
      _exitCode = null;
      _error = null;
    });

    try {
      final client = robot_farm_api.ApiClient(basePath: widget.baseUrl);
      final api = robot_farm_api.DefaultApi(client);
      final payload = robot_farm_api.ExecCommandInput(
        command: command,
        cwd: _cwdController.text.trim().isEmpty
            ? null
            : _cwdController.text.trim(),
      );
      robot_farm_api.ExecResult? result = widget.workerId == null
          ? await api.execOrchestratorCommand(payload)
          : await api.execWorkerCommand(widget.workerId!, payload);

      if (!mounted) return;

      setState(() {
        _stdout = result?.stdout ?? '';
        _stderr = result?.stderr ?? '';
        _exitCode = result?.exitCode;
      });
    } on robot_farm_api.ApiException catch (error) {
      if (!mounted) return;
      setState(() {
        _error = error.message ?? 'Command failed with status ${error.code}.';
      });
    } catch (error) {
      if (!mounted) return;
      setState(() {
        _error = 'Command failed: $error';
      });
    } finally {
      if (mounted) {
        setState(() {
          _isRunning = false;
        });
      }
    }
  }

  String get _outputText {
    final sections = <String>[];

    if (_exitCode != null) {
      sections.add('Exit code: $_exitCode');
    }

    final stdoutText = _stdout ?? '';
    final stderrText = _stderr ?? '';

    if (stdoutText.isNotEmpty) {
      sections.add('stdout:\n$stdoutText');
    }

    if (stderrText.isNotEmpty) {
      sections.add('stderr:\n$stderrText');
    }

    if (sections.isEmpty) {
      return 'No output yet.';
    }

    return sections.join('\n\n');
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final bottomPadding = MediaQuery.of(context).viewInsets.bottom;

    return Padding(
      padding: EdgeInsets.only(bottom: bottomPadding),
      child: FractionallySizedBox(
        heightFactor: 0.85,
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Row(
                children: [
                  Expanded(
                    child: Text(
                      widget.workerId == null
                          ? 'Workspace Command Runner (staging)'
                          : 'Worker ${widget.workerId} Command Runner',
                      style: theme.textTheme.titleLarge,
                    ),
                  ),
                  IconButton(
                    tooltip: 'Close',
                    icon: const Icon(Icons.close),
                    onPressed: () => Navigator.of(context).maybePop(),
                  ),
                ],
              ),
              const SizedBox(height: 16),
              TextField(
                controller: _commandController,
                decoration: const InputDecoration(
                  labelText: 'Command',
                  hintText: 'e.g., ls -la',
                  border: OutlineInputBorder(),
                ),
                textInputAction: TextInputAction.send,
                onSubmitted: (_) => _runCommand(),
              ),
              const SizedBox(height: 12),
              TextField(
                controller: _cwdController,
                decoration: const InputDecoration(
                  labelText: 'Working directory (optional)',
                  hintText: './relative/path or absolute path',
                  border: OutlineInputBorder(),
                ),
              ),
              const SizedBox(height: 12),
              Align(
                alignment: Alignment.centerRight,
                child: FilledButton.icon(
                  onPressed: _isRunning ? null : _runCommand,
                  icon: _isRunning
                      ? const SizedBox(
                          width: 16,
                          height: 16,
                          child: CircularProgressIndicator(strokeWidth: 2),
                        )
                      : const Icon(Icons.play_arrow),
                  label: Text(_isRunning ? 'Running...' : 'Run'),
                ),
              ),
              const SizedBox(height: 12),
              if (_error != null) ...[
                Text(
                  _error!,
                  style: theme.textTheme.bodyMedium?.copyWith(
                    color: theme.colorScheme.error,
                  ),
                ),
                const SizedBox(height: 12),
              ],
              Expanded(
                child: DecoratedBox(
                  decoration: BoxDecoration(
                    border: Border.all(color: theme.colorScheme.outlineVariant),
                    borderRadius: BorderRadius.circular(8),
                    color: theme.colorScheme.surfaceContainerHighest,
                  ),
                  child: Scrollbar(
                    child: SingleChildScrollView(
                      padding: const EdgeInsets.all(12),
                      child: SelectableText(
                        _outputText,
                        style:
                            theme.textTheme.bodyMedium?.merge(
                              _outputMonospaceStyle,
                            ) ??
                            _outputMonospaceStyle,
                      ),
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

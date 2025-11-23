//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class Config {
  /// Returns a new [Config] instance.
  Config({
    this.workspacePath,
    required this.appendAgentsFile,
    required this.models,
    required this.reasoning,
    this.commands = const [],
    this.postTurnChecks = const [],
    this.onStagingChange = const [],
    required this.dockerOverrides,
    this.dirtyStagingAction,
  });

  /// Absolute path of the workspace the server is running in.
  ///
  /// Please note: This property should have been non-nullable! Since the specification file
  /// does not include a default value (using the "default:" property), however, the generated
  /// source code must fall back to having a nullable type.
  /// Consider adding a "default:" property in the specification file to hide this note.
  ///
  String? workspacePath;

  AppendFilesConfig appendAgentsFile;

  AgentModelOverrides models;

  AgentReasoningOverrides reasoning;

  List<CommandConfig> commands;

  /// Commands executed after each turn.
  List<String> postTurnChecks;

  /// Commands to run when staging branch is updated.
  List<String> onStagingChange;

  DockerOverrides dockerOverrides;

  /// Action to take when staging worktree is dirty during task completion.
  ///
  /// Please note: This property should have been non-nullable! Since the specification file
  /// does not include a default value (using the "default:" property), however, the generated
  /// source code must fall back to having a nullable type.
  /// Consider adding a "default:" property in the specification file to hide this note.
  ///
  String? dirtyStagingAction;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Config &&
          other.workspacePath == workspacePath &&
          other.appendAgentsFile == appendAgentsFile &&
          other.models == models &&
          other.reasoning == reasoning &&
          _deepEquality.equals(other.commands, commands) &&
          _deepEquality.equals(other.postTurnChecks, postTurnChecks) &&
          _deepEquality.equals(other.onStagingChange, onStagingChange) &&
          other.dockerOverrides == dockerOverrides &&
          other.dirtyStagingAction == dirtyStagingAction;

  @override
  int get hashCode =>
      // ignore: unnecessary_parenthesis
      (workspacePath == null ? 0 : workspacePath!.hashCode) +
      (appendAgentsFile.hashCode) +
      (models.hashCode) +
      (reasoning.hashCode) +
      (commands.hashCode) +
      (postTurnChecks.hashCode) +
      (onStagingChange.hashCode) +
      (dockerOverrides.hashCode) +
      (dirtyStagingAction == null ? 0 : dirtyStagingAction!.hashCode);

  @override
  String toString() =>
      'Config[workspacePath=$workspacePath, appendAgentsFile=$appendAgentsFile, models=$models, reasoning=$reasoning, commands=$commands, postTurnChecks=$postTurnChecks, onStagingChange=$onStagingChange, dockerOverrides=$dockerOverrides, dirtyStagingAction=$dirtyStagingAction]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
    if (this.workspacePath != null) {
      json[r'workspace_path'] = this.workspacePath;
    } else {
      json[r'workspace_path'] = null;
    }
    json[r'append_agents_file'] = this.appendAgentsFile;
    json[r'models'] = this.models;
    json[r'reasoning'] = this.reasoning;
    json[r'commands'] = this.commands;
    json[r'post_turn_checks'] = this.postTurnChecks;
    json[r'on_staging_change'] = this.onStagingChange;
    json[r'docker_overrides'] = this.dockerOverrides;
    json[r'dirty_staging_action'] = this.dirtyStagingAction;
    return json;
  }

  /// Returns a new [Config] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static Config? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key),
              'Required key "Config[$key]" is missing from JSON.');
          assert(json[key] != null,
              'Required key "Config[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return Config(
        workspacePath: mapValueOfType<String>(json, r'workspace_path'),
        appendAgentsFile:
            AppendFilesConfig.fromJson(json[r'append_agents_file'])!,
        models: AgentModelOverrides.fromJson(json[r'models'])!,
        reasoning: AgentReasoningOverrides.fromJson(json[r'reasoning'])!,
        commands: CommandConfig.listFromJson(json[r'commands']),
        postTurnChecks: json[r'post_turn_checks'] is Iterable
            ? (json[r'post_turn_checks'] as Iterable)
                .cast<String>()
                .toList(growable: false)
            : const [],
        onStagingChange: json[r'on_staging_change'] is Iterable
            ? (json[r'on_staging_change'] as Iterable)
                .cast<String>()
                .toList(growable: false)
            : const [],
        dockerOverrides: DockerOverrides.fromJson(json[r'docker_overrides'])!,
        dirtyStagingAction:
            mapValueOfType<String>(json, r'dirty_staging_action'),
      );
    }
    return null;
  }

  static List<Config> listFromJson(
    dynamic json, {
    bool growable = false,
  }) {
    final result = <Config>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = Config.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, Config> mapFromJson(dynamic json) {
    final map = <String, Config>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = Config.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of Config-objects as value to a dart map
  static Map<String, List<Config>> mapListFromJson(
    dynamic json, {
    bool growable = false,
  }) {
    final map = <String, List<Config>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = Config.listFromJson(
          entry.value,
          growable: growable,
        );
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'append_agents_file',
    'models',
    'reasoning',
    'commands',
    'post_turn_checks',
    'docker_overrides',
    'on_staging_change',
  };
}

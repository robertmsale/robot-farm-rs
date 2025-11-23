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
    required this.dockerOverrides,
    this.dirtyStagingAction = const ConfigDirtyStagingActionEnum._('commit'),
    this.onStagingChange = const [],
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

  DockerOverrides dockerOverrides;

  /// Action to take when staging worktree is dirty during task completion.
  ConfigDirtyStagingActionEnum dirtyStagingAction;

  /// Command IDs to run after staging updates.
  List<String> onStagingChange;

  @override
  bool operator ==(Object other) => identical(this, other) || other is Config &&
    other.workspacePath == workspacePath &&
    other.appendAgentsFile == appendAgentsFile &&
    other.models == models &&
    other.reasoning == reasoning &&
    _deepEquality.equals(other.commands, commands) &&
    _deepEquality.equals(other.postTurnChecks, postTurnChecks) &&
    other.dockerOverrides == dockerOverrides &&
    other.dirtyStagingAction == dirtyStagingAction &&
    _deepEquality.equals(other.onStagingChange, onStagingChange);

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (workspacePath == null ? 0 : workspacePath!.hashCode) +
    (appendAgentsFile.hashCode) +
    (models.hashCode) +
    (reasoning.hashCode) +
    (commands.hashCode) +
    (postTurnChecks.hashCode) +
    (dockerOverrides.hashCode) +
    (dirtyStagingAction.hashCode) +
    (onStagingChange.hashCode);

  @override
  String toString() => 'Config[workspacePath=$workspacePath, appendAgentsFile=$appendAgentsFile, models=$models, reasoning=$reasoning, commands=$commands, postTurnChecks=$postTurnChecks, dockerOverrides=$dockerOverrides, dirtyStagingAction=$dirtyStagingAction, onStagingChange=$onStagingChange]';

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
      json[r'docker_overrides'] = this.dockerOverrides;
      json[r'dirty_staging_action'] = this.dirtyStagingAction;
      json[r'on_staging_change'] = this.onStagingChange;
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
          assert(json.containsKey(key), 'Required key "Config[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "Config[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return Config(
        workspacePath: mapValueOfType<String>(json, r'workspace_path'),
        appendAgentsFile: AppendFilesConfig.fromJson(json[r'append_agents_file'])!,
        models: AgentModelOverrides.fromJson(json[r'models'])!,
        reasoning: AgentReasoningOverrides.fromJson(json[r'reasoning'])!,
        commands: CommandConfig.listFromJson(json[r'commands']),
        postTurnChecks: json[r'post_turn_checks'] is Iterable
            ? (json[r'post_turn_checks'] as Iterable).cast<String>().toList(growable: false)
            : const [],
        dockerOverrides: DockerOverrides.fromJson(json[r'docker_overrides'])!,
        dirtyStagingAction: ConfigDirtyStagingActionEnum.fromJson(json[r'dirty_staging_action']) ?? 'commit',
        onStagingChange: json[r'on_staging_change'] is Iterable
            ? (json[r'on_staging_change'] as Iterable).cast<String>().toList(growable: false)
            : const [],
      );
    }
    return null;
  }

  static List<Config> listFromJson(dynamic json, {bool growable = false,}) {
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
  static Map<String, List<Config>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<Config>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = Config.listFromJson(entry.value, growable: growable,);
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
  };
}

/// Action to take when staging worktree is dirty during task completion.
class ConfigDirtyStagingActionEnum {
  /// Instantiate a new enum with the provided [value].
  const ConfigDirtyStagingActionEnum._(this.value);

  /// The underlying value of this enum member.
  final String value;

  @override
  String toString() => value;

  String toJson() => value;

  static const commit = ConfigDirtyStagingActionEnum._(r'commit');
  static const stash = ConfigDirtyStagingActionEnum._(r'stash');

  /// List of all possible values in this [enum][ConfigDirtyStagingActionEnum].
  static const values = <ConfigDirtyStagingActionEnum>[
    commit,
    stash,
  ];

  static ConfigDirtyStagingActionEnum? fromJson(dynamic value) => ConfigDirtyStagingActionEnumTypeTransformer().decode(value);

  static List<ConfigDirtyStagingActionEnum> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <ConfigDirtyStagingActionEnum>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = ConfigDirtyStagingActionEnum.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }
}

/// Transformation class that can [encode] an instance of [ConfigDirtyStagingActionEnum] to String,
/// and [decode] dynamic data back to [ConfigDirtyStagingActionEnum].
class ConfigDirtyStagingActionEnumTypeTransformer {
  factory ConfigDirtyStagingActionEnumTypeTransformer() => _instance ??= const ConfigDirtyStagingActionEnumTypeTransformer._();

  const ConfigDirtyStagingActionEnumTypeTransformer._();

  String encode(ConfigDirtyStagingActionEnum data) => data.value;

  /// Decodes a [dynamic value][data] to a ConfigDirtyStagingActionEnum.
  ///
  /// If [allowNull] is true and the [dynamic value][data] cannot be decoded successfully,
  /// then null is returned. However, if [allowNull] is false and the [dynamic value][data]
  /// cannot be decoded successfully, then an [UnimplementedError] is thrown.
  ///
  /// The [allowNull] is very handy when an API changes and a new enum value is added or removed,
  /// and users are still using an old app with the old code.
  ConfigDirtyStagingActionEnum? decode(dynamic data, {bool allowNull = true}) {
    if (data != null) {
      switch (data) {
        case r'commit': return ConfigDirtyStagingActionEnum.commit;
        case r'stash': return ConfigDirtyStagingActionEnum.stash;
        default:
          if (!allowNull) {
            throw ArgumentError('Unknown enum value to decode: $data');
          }
      }
    }
    return null;
  }

  /// Singleton [ConfigDirtyStagingActionEnumTypeTransformer] instance.
  static ConfigDirtyStagingActionEnumTypeTransformer? _instance;
}



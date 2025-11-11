//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class CommandConfig {
  /// Returns a new [CommandConfig] instance.
  CommandConfig({
    required this.id,
    this.exec = const [],
    this.stdoutSuccessMessage,
    this.hidden = false,
    this.timeoutSeconds,
  });

  String id;

  /// Shell command and arguments to execute.
  List<String> exec;

  ///
  /// Please note: This property should have been non-nullable! Since the specification file
  /// does not include a default value (using the "default:" property), however, the generated
  /// source code must fall back to having a nullable type.
  /// Consider adding a "default:" property in the specification file to hide this note.
  ///
  String? stdoutSuccessMessage;

  bool hidden;

  ///
  /// Please note: This property should have been non-nullable! Since the specification file
  /// does not include a default value (using the "default:" property), however, the generated
  /// source code must fall back to having a nullable type.
  /// Consider adding a "default:" property in the specification file to hide this note.
  ///
  int? timeoutSeconds;

  @override
  bool operator ==(Object other) => identical(this, other) || other is CommandConfig &&
    other.id == id &&
    _deepEquality.equals(other.exec, exec) &&
    other.stdoutSuccessMessage == stdoutSuccessMessage &&
    other.hidden == hidden &&
    other.timeoutSeconds == timeoutSeconds;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (id.hashCode) +
    (exec.hashCode) +
    (stdoutSuccessMessage == null ? 0 : stdoutSuccessMessage!.hashCode) +
    (hidden.hashCode) +
    (timeoutSeconds == null ? 0 : timeoutSeconds!.hashCode);

  @override
  String toString() => 'CommandConfig[id=$id, exec=$exec, stdoutSuccessMessage=$stdoutSuccessMessage, hidden=$hidden, timeoutSeconds=$timeoutSeconds]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'id'] = this.id;
      json[r'exec'] = this.exec;
    if (this.stdoutSuccessMessage != null) {
      json[r'stdout_success_message'] = this.stdoutSuccessMessage;
    } else {
      json[r'stdout_success_message'] = null;
    }
      json[r'hidden'] = this.hidden;
    if (this.timeoutSeconds != null) {
      json[r'timeout_seconds'] = this.timeoutSeconds;
    } else {
      json[r'timeout_seconds'] = null;
    }
    return json;
  }

  /// Returns a new [CommandConfig] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static CommandConfig? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "CommandConfig[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "CommandConfig[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return CommandConfig(
        id: mapValueOfType<String>(json, r'id')!,
        exec: json[r'exec'] is Iterable
            ? (json[r'exec'] as Iterable).cast<String>().toList(growable: false)
            : const [],
        stdoutSuccessMessage: mapValueOfType<String>(json, r'stdout_success_message'),
        hidden: mapValueOfType<bool>(json, r'hidden') ?? false,
        timeoutSeconds: mapValueOfType<int>(json, r'timeout_seconds'),
      );
    }
    return null;
  }

  static List<CommandConfig> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <CommandConfig>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = CommandConfig.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, CommandConfig> mapFromJson(dynamic json) {
    final map = <String, CommandConfig>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = CommandConfig.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of CommandConfig-objects as value to a dart map
  static Map<String, List<CommandConfig>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<CommandConfig>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = CommandConfig.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'id',
    'exec',
  };
}


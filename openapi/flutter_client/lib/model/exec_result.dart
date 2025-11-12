//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class ExecResult {
  /// Returns a new [ExecResult] instance.
  ExecResult({
    required this.command,
    required this.exitCode,
    required this.stdout,
    required this.stderr,
  });

  String command;

  int exitCode;

  String stdout;

  String stderr;

  @override
  bool operator ==(Object other) => identical(this, other) || other is ExecResult &&
    other.command == command &&
    other.exitCode == exitCode &&
    other.stdout == stdout &&
    other.stderr == stderr;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (command.hashCode) +
    (exitCode.hashCode) +
    (stdout.hashCode) +
    (stderr.hashCode);

  @override
  String toString() => 'ExecResult[command=$command, exitCode=$exitCode, stdout=$stdout, stderr=$stderr]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'command'] = this.command;
      json[r'exit_code'] = this.exitCode;
      json[r'stdout'] = this.stdout;
      json[r'stderr'] = this.stderr;
    return json;
  }

  /// Returns a new [ExecResult] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static ExecResult? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "ExecResult[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "ExecResult[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return ExecResult(
        command: mapValueOfType<String>(json, r'command')!,
        exitCode: mapValueOfType<int>(json, r'exit_code')!,
        stdout: mapValueOfType<String>(json, r'stdout')!,
        stderr: mapValueOfType<String>(json, r'stderr')!,
      );
    }
    return null;
  }

  static List<ExecResult> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <ExecResult>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = ExecResult.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, ExecResult> mapFromJson(dynamic json) {
    final map = <String, ExecResult>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = ExecResult.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of ExecResult-objects as value to a dart map
  static Map<String, List<ExecResult>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<ExecResult>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = ExecResult.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'command',
    'exit_code',
    'stdout',
    'stderr',
  };
}


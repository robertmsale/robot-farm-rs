//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class AppendFilesConfig {
  /// Returns a new [AppendFilesConfig] instance.
  AppendFilesConfig({
    this.orchestrator = const [],
    this.worker = const [],
  });

  /// List of file paths appended to orchestrator prompts.
  List<String> orchestrator;

  /// List of file paths appended to worker prompts.
  List<String> worker;

  @override
  bool operator ==(Object other) => identical(this, other) || other is AppendFilesConfig &&
    _deepEquality.equals(other.orchestrator, orchestrator) &&
    _deepEquality.equals(other.worker, worker);

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (orchestrator.hashCode) +
    (worker.hashCode);

  @override
  String toString() => 'AppendFilesConfig[orchestrator=$orchestrator, worker=$worker]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'orchestrator'] = this.orchestrator;
      json[r'worker'] = this.worker;
    return json;
  }

  /// Returns a new [AppendFilesConfig] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static AppendFilesConfig? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "AppendFilesConfig[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "AppendFilesConfig[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return AppendFilesConfig(
        orchestrator: json[r'orchestrator'] is Iterable
            ? (json[r'orchestrator'] as Iterable).cast<String>().toList(growable: false)
            : const [],
        worker: json[r'worker'] is Iterable
            ? (json[r'worker'] as Iterable).cast<String>().toList(growable: false)
            : const [],
      );
    }
    return null;
  }

  static List<AppendFilesConfig> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <AppendFilesConfig>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = AppendFilesConfig.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, AppendFilesConfig> mapFromJson(dynamic json) {
    final map = <String, AppendFilesConfig>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = AppendFilesConfig.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of AppendFilesConfig-objects as value to a dart map
  static Map<String, List<AppendFilesConfig>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<AppendFilesConfig>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = AppendFilesConfig.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'orchestrator',
    'worker',
  };
}


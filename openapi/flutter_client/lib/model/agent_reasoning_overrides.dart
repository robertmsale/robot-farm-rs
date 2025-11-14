//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class AgentReasoningOverrides {
  /// Returns a new [AgentReasoningOverrides] instance.
  AgentReasoningOverrides({
    required this.orchestrator,
    required this.worker,
    required this.wizard,
  });

  ReasoningEffort orchestrator;

  ReasoningEffort worker;

  ReasoningEffort wizard;

  @override
  bool operator ==(Object other) => identical(this, other) || other is AgentReasoningOverrides &&
    other.orchestrator == orchestrator &&
    other.worker == worker &&
    other.wizard == wizard;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (orchestrator.hashCode) +
    (worker.hashCode) +
    (wizard.hashCode);

  @override
  String toString() => 'AgentReasoningOverrides[orchestrator=$orchestrator, worker=$worker, wizard=$wizard]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'orchestrator'] = this.orchestrator;
      json[r'worker'] = this.worker;
      json[r'wizard'] = this.wizard;
    return json;
  }

  /// Returns a new [AgentReasoningOverrides] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static AgentReasoningOverrides? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "AgentReasoningOverrides[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "AgentReasoningOverrides[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return AgentReasoningOverrides(
        orchestrator: ReasoningEffort.fromJson(json[r'orchestrator'])!,
        worker: ReasoningEffort.fromJson(json[r'worker'])!,
        wizard: ReasoningEffort.fromJson(json[r'wizard'])!,
      );
    }
    return null;
  }

  static List<AgentReasoningOverrides> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <AgentReasoningOverrides>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = AgentReasoningOverrides.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, AgentReasoningOverrides> mapFromJson(dynamic json) {
    final map = <String, AgentReasoningOverrides>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = AgentReasoningOverrides.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of AgentReasoningOverrides-objects as value to a dart map
  static Map<String, List<AgentReasoningOverrides>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<AgentReasoningOverrides>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = AgentReasoningOverrides.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'orchestrator',
    'worker',
    'wizard',
  };
}


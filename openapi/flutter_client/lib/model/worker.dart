//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class Worker {
  /// Returns a new [Worker] instance.
  Worker({
    required this.id,
    required this.lastSeen,
    required this.state,
    this.threadId,
  });

  int id;

  /// Unix timestamp in seconds when the worker last reported.
  int lastSeen;

  WorkerState state;

  String? threadId;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Worker &&
          other.id == id &&
          other.lastSeen == lastSeen &&
          other.state == state &&
          other.threadId == threadId;

  @override
  int get hashCode =>
      // ignore: unnecessary_parenthesis
      (id.hashCode) +
      (lastSeen.hashCode) +
      (state.hashCode) +
      (threadId == null ? 0 : threadId!.hashCode);

  @override
  String toString() =>
      'Worker[id=$id, lastSeen=$lastSeen, state=$state, threadId=$threadId]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
    json[r'id'] = this.id;
    json[r'last_seen'] = this.lastSeen;
    json[r'state'] = this.state;
    if (this.threadId != null) {
      json[r'thread_id'] = this.threadId;
    }
    return json;
  }

  /// Returns a new [Worker] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static Worker? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key),
              'Required key "Worker[$key]" is missing from JSON.');
          assert(json[key] != null,
              'Required key "Worker[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return Worker(
        id: mapValueOfType<int>(json, r'id')!,
        lastSeen: mapValueOfType<int>(json, r'last_seen')!,
        state: WorkerState.fromJson(json[r'state'])!,
        threadId: mapValueOfType<String>(json, r'thread_id'),
      );
    }
    return null;
  }

  static List<Worker> listFromJson(
    dynamic json, {
    bool growable = false,
  }) {
    final result = <Worker>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = Worker.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, Worker> mapFromJson(dynamic json) {
    final map = <String, Worker>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = Worker.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of Worker-objects as value to a dart map
  static Map<String, List<Worker>> mapListFromJson(
    dynamic json, {
    bool growable = false,
  }) {
    final map = <String, List<Worker>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = Worker.listFromJson(
          entry.value,
          growable: growable,
        );
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'id',
    'last_seen',
    'state',
  };
}

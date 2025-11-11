//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class TaskDependency {
  /// Returns a new [TaskDependency] instance.
  TaskDependency({
    required this.taskId,
    required this.dependsOnTaskId,
  });

  int taskId;

  int dependsOnTaskId;

  @override
  bool operator ==(Object other) => identical(this, other) || other is TaskDependency &&
    other.taskId == taskId &&
    other.dependsOnTaskId == dependsOnTaskId;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (taskId.hashCode) +
    (dependsOnTaskId.hashCode);

  @override
  String toString() => 'TaskDependency[taskId=$taskId, dependsOnTaskId=$dependsOnTaskId]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'task_id'] = this.taskId;
      json[r'depends_on_task_id'] = this.dependsOnTaskId;
    return json;
  }

  /// Returns a new [TaskDependency] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static TaskDependency? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "TaskDependency[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "TaskDependency[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return TaskDependency(
        taskId: mapValueOfType<int>(json, r'task_id')!,
        dependsOnTaskId: mapValueOfType<int>(json, r'depends_on_task_id')!,
      );
    }
    return null;
  }

  static List<TaskDependency> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <TaskDependency>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = TaskDependency.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, TaskDependency> mapFromJson(dynamic json) {
    final map = <String, TaskDependency>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = TaskDependency.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of TaskDependency-objects as value to a dart map
  static Map<String, List<TaskDependency>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<TaskDependency>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = TaskDependency.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'task_id',
    'depends_on_task_id',
  };
}


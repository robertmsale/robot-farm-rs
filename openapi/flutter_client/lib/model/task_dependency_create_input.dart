//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class TaskDependencyCreateInput {
  /// Returns a new [TaskDependencyCreateInput] instance.
  TaskDependencyCreateInput({
    required this.taskId,
    required this.dependsOnTaskId,
  });

  int taskId;

  int dependsOnTaskId;

  @override
  bool operator ==(Object other) => identical(this, other) || other is TaskDependencyCreateInput &&
    other.taskId == taskId &&
    other.dependsOnTaskId == dependsOnTaskId;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (taskId.hashCode) +
    (dependsOnTaskId.hashCode);

  @override
  String toString() => 'TaskDependencyCreateInput[taskId=$taskId, dependsOnTaskId=$dependsOnTaskId]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'task_id'] = this.taskId;
      json[r'depends_on_task_id'] = this.dependsOnTaskId;
    return json;
  }

  /// Returns a new [TaskDependencyCreateInput] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static TaskDependencyCreateInput? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "TaskDependencyCreateInput[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "TaskDependencyCreateInput[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return TaskDependencyCreateInput(
        taskId: mapValueOfType<int>(json, r'task_id')!,
        dependsOnTaskId: mapValueOfType<int>(json, r'depends_on_task_id')!,
      );
    }
    return null;
  }

  static List<TaskDependencyCreateInput> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <TaskDependencyCreateInput>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = TaskDependencyCreateInput.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, TaskDependencyCreateInput> mapFromJson(dynamic json) {
    final map = <String, TaskDependencyCreateInput>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = TaskDependencyCreateInput.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of TaskDependencyCreateInput-objects as value to a dart map
  static Map<String, List<TaskDependencyCreateInput>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<TaskDependencyCreateInput>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = TaskDependencyCreateInput.listFromJson(entry.value, growable: growable,);
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


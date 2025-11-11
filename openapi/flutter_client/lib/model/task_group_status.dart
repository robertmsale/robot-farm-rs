//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

/// Derived status for a task group: Done when it has no tasks or every task is Done, otherwise Ready.
class TaskGroupStatus {
  /// Instantiate a new enum with the provided [value].
  const TaskGroupStatus._(this.value);

  /// The underlying value of this enum member.
  final String value;

  @override
  String toString() => value;

  String toJson() => value;

  static const ready = TaskGroupStatus._(r'Ready');
  static const done = TaskGroupStatus._(r'Done');

  /// List of all possible values in this [enum][TaskGroupStatus].
  static const values = <TaskGroupStatus>[
    ready,
    done,
  ];

  static TaskGroupStatus? fromJson(dynamic value) => TaskGroupStatusTypeTransformer().decode(value);

  static List<TaskGroupStatus> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <TaskGroupStatus>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = TaskGroupStatus.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }
}

/// Transformation class that can [encode] an instance of [TaskGroupStatus] to String,
/// and [decode] dynamic data back to [TaskGroupStatus].
class TaskGroupStatusTypeTransformer {
  factory TaskGroupStatusTypeTransformer() => _instance ??= const TaskGroupStatusTypeTransformer._();

  const TaskGroupStatusTypeTransformer._();

  String encode(TaskGroupStatus data) => data.value;

  /// Decodes a [dynamic value][data] to a TaskGroupStatus.
  ///
  /// If [allowNull] is true and the [dynamic value][data] cannot be decoded successfully,
  /// then null is returned. However, if [allowNull] is false and the [dynamic value][data]
  /// cannot be decoded successfully, then an [UnimplementedError] is thrown.
  ///
  /// The [allowNull] is very handy when an API changes and a new enum value is added or removed,
  /// and users are still using an old app with the old code.
  TaskGroupStatus? decode(dynamic data, {bool allowNull = true}) {
    if (data != null) {
      switch (data) {
        case r'Ready': return TaskGroupStatus.ready;
        case r'Done': return TaskGroupStatus.done;
        default:
          if (!allowNull) {
            throw ArgumentError('Unknown enum value to decode: $data');
          }
      }
    }
    return null;
  }

  /// Singleton [TaskGroupStatusTypeTransformer] instance.
  static TaskGroupStatusTypeTransformer? _instance;
}


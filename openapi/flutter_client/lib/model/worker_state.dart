//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

/// State reported by the worker.
class WorkerState {
  /// Instantiate a new enum with the provided [value].
  const WorkerState._(this.value);

  /// The underlying value of this enum member.
  final String value;

  @override
  String toString() => value;

  String toJson() => value;

  static const ready = WorkerState._(r'Ready');
  static const working = WorkerState._(r'Working');
  static const blocked = WorkerState._(r'Blocked');
  static const done = WorkerState._(r'Done');

  /// List of all possible values in this [enum][WorkerState].
  static const values = <WorkerState>[
    ready,
    working,
    blocked,
    done,
  ];

  static WorkerState? fromJson(dynamic value) => WorkerStateTypeTransformer().decode(value);

  static List<WorkerState> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <WorkerState>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = WorkerState.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }
}

/// Transformation class that can [encode] an instance of [WorkerState] to String,
/// and [decode] dynamic data back to [WorkerState].
class WorkerStateTypeTransformer {
  factory WorkerStateTypeTransformer() => _instance ??= const WorkerStateTypeTransformer._();

  const WorkerStateTypeTransformer._();

  String encode(WorkerState data) => data.value;

  /// Decodes a [dynamic value][data] to a WorkerState.
  ///
  /// If [allowNull] is true and the [dynamic value][data] cannot be decoded successfully,
  /// then null is returned. However, if [allowNull] is false and the [dynamic value][data]
  /// cannot be decoded successfully, then an [UnimplementedError] is thrown.
  ///
  /// The [allowNull] is very handy when an API changes and a new enum value is added or removed,
  /// and users are still using an old app with the old code.
  WorkerState? decode(dynamic data, {bool allowNull = true}) {
    if (data != null) {
      switch (data) {
        case r'Ready': return WorkerState.ready;
        case r'Working': return WorkerState.working;
        case r'Blocked': return WorkerState.blocked;
        case r'Done': return WorkerState.done;
        default:
          if (!allowNull) {
            throw ArgumentError('Unknown enum value to decode: $data');
          }
      }
    }
    return null;
  }

  /// Singleton [WorkerStateTypeTransformer] instance.
  static WorkerStateTypeTransformer? _instance;
}


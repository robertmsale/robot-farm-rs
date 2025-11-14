//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

/// Codex reasoning effort to request when launching a persona.
class ReasoningEffort {
  /// Instantiate a new enum with the provided [value].
  const ReasoningEffort._(this.value);

  /// The underlying value of this enum member.
  final String value;

  @override
  String toString() => value;

  String toJson() => value;

  static const low = ReasoningEffort._(r'low');
  static const medium = ReasoningEffort._(r'medium');
  static const high = ReasoningEffort._(r'high');

  /// List of all possible values in this [enum][ReasoningEffort].
  static const values = <ReasoningEffort>[
    low,
    medium,
    high,
  ];

  static ReasoningEffort? fromJson(dynamic value) => ReasoningEffortTypeTransformer().decode(value);

  static List<ReasoningEffort> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <ReasoningEffort>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = ReasoningEffort.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }
}

/// Transformation class that can [encode] an instance of [ReasoningEffort] to String,
/// and [decode] dynamic data back to [ReasoningEffort].
class ReasoningEffortTypeTransformer {
  factory ReasoningEffortTypeTransformer() => _instance ??= const ReasoningEffortTypeTransformer._();

  const ReasoningEffortTypeTransformer._();

  String encode(ReasoningEffort data) => data.value;

  /// Decodes a [dynamic value][data] to a ReasoningEffort.
  ///
  /// If [allowNull] is true and the [dynamic value][data] cannot be decoded successfully,
  /// then null is returned. However, if [allowNull] is false and the [dynamic value][data]
  /// cannot be decoded successfully, then an [UnimplementedError] is thrown.
  ///
  /// The [allowNull] is very handy when an API changes and a new enum value is added or removed,
  /// and users are still using an old app with the old code.
  ReasoningEffort? decode(dynamic data, {bool allowNull = true}) {
    if (data != null) {
      switch (data) {
        case r'low': return ReasoningEffort.low;
        case r'medium': return ReasoningEffort.medium;
        case r'high': return ReasoningEffort.high;
        default:
          if (!allowNull) {
            throw ArgumentError('Unknown enum value to decode: $data');
          }
      }
    }
    return null;
  }

  /// Singleton [ReasoningEffortTypeTransformer] instance.
  static ReasoningEffortTypeTransformer? _instance;
}


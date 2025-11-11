//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

/// Severity of the feed entry.
class FeedLevel {
  /// Instantiate a new enum with the provided [value].
  const FeedLevel._(this.value);

  /// The underlying value of this enum member.
  final String value;

  @override
  String toString() => value;

  String toJson() => value;

  static const info = FeedLevel._(r'info');
  static const warning = FeedLevel._(r'warning');
  static const error = FeedLevel._(r'error');

  /// List of all possible values in this [enum][FeedLevel].
  static const values = <FeedLevel>[
    info,
    warning,
    error,
  ];

  static FeedLevel? fromJson(dynamic value) => FeedLevelTypeTransformer().decode(value);

  static List<FeedLevel> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <FeedLevel>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = FeedLevel.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }
}

/// Transformation class that can [encode] an instance of [FeedLevel] to String,
/// and [decode] dynamic data back to [FeedLevel].
class FeedLevelTypeTransformer {
  factory FeedLevelTypeTransformer() => _instance ??= const FeedLevelTypeTransformer._();

  const FeedLevelTypeTransformer._();

  String encode(FeedLevel data) => data.value;

  /// Decodes a [dynamic value][data] to a FeedLevel.
  ///
  /// If [allowNull] is true and the [dynamic value][data] cannot be decoded successfully,
  /// then null is returned. However, if [allowNull] is false and the [dynamic value][data]
  /// cannot be decoded successfully, then an [UnimplementedError] is thrown.
  ///
  /// The [allowNull] is very handy when an API changes and a new enum value is added or removed,
  /// and users are still using an old app with the old code.
  FeedLevel? decode(dynamic data, {bool allowNull = true}) {
    if (data != null) {
      switch (data) {
        case r'info': return FeedLevel.info;
        case r'warning': return FeedLevel.warning;
        case r'error': return FeedLevel.error;
        default:
          if (!allowNull) {
            throw ArgumentError('Unknown enum value to decode: $data');
          }
      }
    }
    return null;
  }

  /// Singleton [FeedLevelTypeTransformer] instance.
  static FeedLevelTypeTransformer? _instance;
}


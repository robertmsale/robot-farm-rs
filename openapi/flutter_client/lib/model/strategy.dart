//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

/// System-wide execution strategy.
class Strategy {
  /// Instantiate a new enum with the provided [value].
  const Strategy._(this.value);

  /// The underlying value of this enum member.
  final String value;

  @override
  String toString() => value;

  String toJson() => value;

  static const AGGRESSIVE = Strategy._(r'AGGRESSIVE');
  static const MODERATE = Strategy._(r'MODERATE');
  static const ECONOMICAL = Strategy._(r'ECONOMICAL');
  static const BUG_SMASH = Strategy._(r'BUG_SMASH');
  static const HOTFIX_SWARM = Strategy._(r'HOTFIX_SWARM');
  static const PLANNING = Strategy._(r'PLANNING');
  static const WIND_DOWN = Strategy._(r'WIND_DOWN');

  /// List of all possible values in this [enum][Strategy].
  static const values = <Strategy>[
    AGGRESSIVE,
    MODERATE,
    ECONOMICAL,
    BUG_SMASH,
    HOTFIX_SWARM,
    PLANNING,
    WIND_DOWN,
  ];

  static Strategy? fromJson(dynamic value) => StrategyTypeTransformer().decode(value);

  static List<Strategy> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <Strategy>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = Strategy.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }
}

/// Transformation class that can [encode] an instance of [Strategy] to String,
/// and [decode] dynamic data back to [Strategy].
class StrategyTypeTransformer {
  factory StrategyTypeTransformer() => _instance ??= const StrategyTypeTransformer._();

  const StrategyTypeTransformer._();

  String encode(Strategy data) => data.value;

  /// Decodes a [dynamic value][data] to a Strategy.
  ///
  /// If [allowNull] is true and the [dynamic value][data] cannot be decoded successfully,
  /// then null is returned. However, if [allowNull] is false and the [dynamic value][data]
  /// cannot be decoded successfully, then an [UnimplementedError] is thrown.
  ///
  /// The [allowNull] is very handy when an API changes and a new enum value is added or removed,
  /// and users are still using an old app with the old code.
  Strategy? decode(dynamic data, {bool allowNull = true}) {
    if (data != null) {
      switch (data) {
        case r'AGGRESSIVE': return Strategy.AGGRESSIVE;
        case r'MODERATE': return Strategy.MODERATE;
        case r'ECONOMICAL': return Strategy.ECONOMICAL;
        case r'BUG_SMASH': return Strategy.BUG_SMASH;
        case r'HOTFIX_SWARM': return Strategy.HOTFIX_SWARM;
        case r'PLANNING': return Strategy.PLANNING;
        case r'WIND_DOWN': return Strategy.WIND_DOWN;
        default:
          if (!allowNull) {
            throw ArgumentError('Unknown enum value to decode: $data');
          }
      }
    }
    return null;
  }

  /// Singleton [StrategyTypeTransformer] instance.
  static StrategyTypeTransformer? _instance;
}


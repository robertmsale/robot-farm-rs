//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

/// Valid Feed fields that can be used for ordering.
class FeedOrderField {
  /// Instantiate a new enum with the provided [value].
  const FeedOrderField._(this.value);

  /// The underlying value of this enum member.
  final String value;

  @override
  String toString() => value;

  String toJson() => value;

  static const id = FeedOrderField._(r'id');
  static const source_ = FeedOrderField._(r'source');
  static const target = FeedOrderField._(r'target');
  static const ts = FeedOrderField._(r'ts');
  static const level = FeedOrderField._(r'level');
  static const text = FeedOrderField._(r'text');
  static const raw = FeedOrderField._(r'raw');
  static const category = FeedOrderField._(r'category');

  /// List of all possible values in this [enum][FeedOrderField].
  static const values = <FeedOrderField>[
    id,
    source_,
    target,
    ts,
    level,
    text,
    raw,
    category,
  ];

  static FeedOrderField? fromJson(dynamic value) => FeedOrderFieldTypeTransformer().decode(value);

  static List<FeedOrderField> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <FeedOrderField>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = FeedOrderField.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }
}

/// Transformation class that can [encode] an instance of [FeedOrderField] to String,
/// and [decode] dynamic data back to [FeedOrderField].
class FeedOrderFieldTypeTransformer {
  factory FeedOrderFieldTypeTransformer() => _instance ??= const FeedOrderFieldTypeTransformer._();

  const FeedOrderFieldTypeTransformer._();

  String encode(FeedOrderField data) => data.value;

  /// Decodes a [dynamic value][data] to a FeedOrderField.
  ///
  /// If [allowNull] is true and the [dynamic value][data] cannot be decoded successfully,
  /// then null is returned. However, if [allowNull] is false and the [dynamic value][data]
  /// cannot be decoded successfully, then an [UnimplementedError] is thrown.
  ///
  /// The [allowNull] is very handy when an API changes and a new enum value is added or removed,
  /// and users are still using an old app with the old code.
  FeedOrderField? decode(dynamic data, {bool allowNull = true}) {
    if (data != null) {
      switch (data) {
        case r'id': return FeedOrderField.id;
        case r'source': return FeedOrderField.source_;
        case r'target': return FeedOrderField.target;
        case r'ts': return FeedOrderField.ts;
        case r'level': return FeedOrderField.level;
        case r'text': return FeedOrderField.text;
        case r'raw': return FeedOrderField.raw;
        case r'category': return FeedOrderField.category;
        default:
          if (!allowNull) {
            throw ArgumentError('Unknown enum value to decode: $data');
          }
      }
    }
    return null;
  }

  /// Singleton [FeedOrderFieldTypeTransformer] instance.
  static FeedOrderFieldTypeTransformer? _instance;
}


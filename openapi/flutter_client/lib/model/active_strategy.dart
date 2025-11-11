//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class ActiveStrategy {
  /// Returns a new [ActiveStrategy] instance.
  ActiveStrategy({
    required this.id,
    this.focus = const [],
  });

  Strategy id;

  /// Optional list of task group identifiers receiving additional focus.
  List<int> focus;

  @override
  bool operator ==(Object other) => identical(this, other) || other is ActiveStrategy &&
    other.id == id &&
    _deepEquality.equals(other.focus, focus);

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (id.hashCode) +
    (focus.hashCode);

  @override
  String toString() => 'ActiveStrategy[id=$id, focus=$focus]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'id'] = this.id;
      json[r'focus'] = this.focus;
    return json;
  }

  /// Returns a new [ActiveStrategy] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static ActiveStrategy? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "ActiveStrategy[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "ActiveStrategy[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return ActiveStrategy(
        id: Strategy.fromJson(json[r'id'])!,
        focus: json[r'focus'] is Iterable
            ? (json[r'focus'] as Iterable).cast<int>().toList(growable: false)
            : const [],
      );
    }
    return null;
  }

  static List<ActiveStrategy> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <ActiveStrategy>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = ActiveStrategy.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, ActiveStrategy> mapFromJson(dynamic json) {
    final map = <String, ActiveStrategy>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = ActiveStrategy.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of ActiveStrategy-objects as value to a dart map
  static Map<String, List<ActiveStrategy>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<ActiveStrategy>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = ActiveStrategy.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'id',
  };
}


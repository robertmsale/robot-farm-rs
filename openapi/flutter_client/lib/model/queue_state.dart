//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class QueueState {
  /// Returns a new [QueueState] instance.
  QueueState({
    required this.paused,
  });

  bool paused;

  @override
  bool operator ==(Object other) => identical(this, other) || other is QueueState &&
    other.paused == paused;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (paused.hashCode);

  @override
  String toString() => 'QueueState[paused=$paused]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'paused'] = this.paused;
    return json;
  }

  /// Returns a new [QueueState] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static QueueState? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "QueueState[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "QueueState[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return QueueState(
        paused: mapValueOfType<bool>(json, r'paused')!,
      );
    }
    return null;
  }

  static List<QueueState> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <QueueState>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = QueueState.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, QueueState> mapFromJson(dynamic json) {
    final map = <String, QueueState>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = QueueState.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of QueueState-objects as value to a dart map
  static Map<String, List<QueueState>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<QueueState>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = QueueState.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'paused',
  };
}


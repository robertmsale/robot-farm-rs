//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class GitStatusHunk {
  /// Returns a new [GitStatusHunk] instance.
  GitStatusHunk({
    required this.header,
    this.lines = const [],
  });

  /// The @@ header line describing the hunk range.
  String header;

  /// All lines within the hunk including +/- context.
  List<String> lines;

  @override
  bool operator ==(Object other) => identical(this, other) || other is GitStatusHunk &&
    other.header == header &&
    _deepEquality.equals(other.lines, lines);

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (header.hashCode) +
    (lines.hashCode);

  @override
  String toString() => 'GitStatusHunk[header=$header, lines=$lines]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'header'] = this.header;
      json[r'lines'] = this.lines;
    return json;
  }

  /// Returns a new [GitStatusHunk] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static GitStatusHunk? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "GitStatusHunk[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "GitStatusHunk[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return GitStatusHunk(
        header: mapValueOfType<String>(json, r'header')!,
        lines: json[r'lines'] is Iterable
            ? (json[r'lines'] as Iterable).cast<String>().toList(growable: false)
            : const [],
      );
    }
    return null;
  }

  static List<GitStatusHunk> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <GitStatusHunk>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = GitStatusHunk.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, GitStatusHunk> mapFromJson(dynamic json) {
    final map = <String, GitStatusHunk>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = GitStatusHunk.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of GitStatusHunk-objects as value to a dart map
  static Map<String, List<GitStatusHunk>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<GitStatusHunk>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = GitStatusHunk.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'header',
    'lines',
  };
}


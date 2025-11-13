//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class GitStatusFileChange {
  /// Returns a new [GitStatusFileChange] instance.
  GitStatusFileChange({
    required this.path,
    this.oldPath,
    required this.statusCode,
    required this.additions,
    required this.deletions,
    this.hunks = const [],
  });

  /// Current file path relative to the worktree root.
  String path;

  /// Previous path when the file was renamed or moved.
  ///
  /// Please note: This property should have been non-nullable! Since the specification file
  /// does not include a default value (using the "default:" property), however, the generated
  /// source code must fall back to having a nullable type.
  /// Consider adding a "default:" property in the specification file to hide this note.
  ///
  String? oldPath;

  /// Two-character porcelain status code (e.g., M?, R?).
  String statusCode;

  /// Number of added lines reported by git.
  ///
  /// Minimum value: 0
  int additions;

  /// Number of removed lines reported by git.
  ///
  /// Minimum value: 0
  int deletions;

  /// Diff hunks for this file when requested.
  List<GitStatusHunk> hunks;

  @override
  bool operator ==(Object other) => identical(this, other) || other is GitStatusFileChange &&
    other.path == path &&
    other.oldPath == oldPath &&
    other.statusCode == statusCode &&
    other.additions == additions &&
    other.deletions == deletions &&
    _deepEquality.equals(other.hunks, hunks);

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (path.hashCode) +
    (oldPath == null ? 0 : oldPath!.hashCode) +
    (statusCode.hashCode) +
    (additions.hashCode) +
    (deletions.hashCode) +
    (hunks.hashCode);

  @override
  String toString() => 'GitStatusFileChange[path=$path, oldPath=$oldPath, statusCode=$statusCode, additions=$additions, deletions=$deletions, hunks=$hunks]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'path'] = this.path;
    if (this.oldPath != null) {
      json[r'old_path'] = this.oldPath;
    } else {
      json[r'old_path'] = null;
    }
      json[r'status_code'] = this.statusCode;
      json[r'additions'] = this.additions;
      json[r'deletions'] = this.deletions;
      json[r'hunks'] = this.hunks;
    return json;
  }

  /// Returns a new [GitStatusFileChange] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static GitStatusFileChange? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "GitStatusFileChange[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "GitStatusFileChange[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return GitStatusFileChange(
        path: mapValueOfType<String>(json, r'path')!,
        oldPath: mapValueOfType<String>(json, r'old_path'),
        statusCode: mapValueOfType<String>(json, r'status_code')!,
        additions: mapValueOfType<int>(json, r'additions')!,
        deletions: mapValueOfType<int>(json, r'deletions')!,
        hunks: GitStatusHunk.listFromJson(json[r'hunks']),
      );
    }
    return null;
  }

  static List<GitStatusFileChange> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <GitStatusFileChange>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = GitStatusFileChange.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, GitStatusFileChange> mapFromJson(dynamic json) {
    final map = <String, GitStatusFileChange>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = GitStatusFileChange.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of GitStatusFileChange-objects as value to a dart map
  static Map<String, List<GitStatusFileChange>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<GitStatusFileChange>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = GitStatusFileChange.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'path',
    'status_code',
    'additions',
    'deletions',
  };
}


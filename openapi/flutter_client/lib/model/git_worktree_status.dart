//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class GitWorktreeStatus {
  /// Returns a new [GitWorktreeStatus] instance.
  GitWorktreeStatus({
    required this.id,
    required this.path,
    required this.branch,
    this.upstream,
    required this.ahead,
    required this.behind,
    required this.isDirty,
    this.files = const [],
  });

  /// Logical identifier (staging or ws{n}).
  String id;

  /// Absolute filesystem path to the worktree.
  String path;

  /// Currently checked out branch.
  String branch;

  /// Configured upstream tracking branch, if any.
  ///
  /// Please note: This property should have been non-nullable! Since the specification file
  /// does not include a default value (using the "default:" property), however, the generated
  /// source code must fall back to having a nullable type.
  /// Consider adding a "default:" property in the specification file to hide this note.
  ///
  String? upstream;

  /// Number of commits ahead of upstream.
  ///
  /// Minimum value: 0
  int ahead;

  /// Number of commits behind upstream.
  ///
  /// Minimum value: 0
  int behind;

  /// True when there are local modifications.
  bool isDirty;

  /// Per-file change metadata.
  List<GitStatusFileChange> files;

  @override
  bool operator ==(Object other) => identical(this, other) || other is GitWorktreeStatus &&
    other.id == id &&
    other.path == path &&
    other.branch == branch &&
    other.upstream == upstream &&
    other.ahead == ahead &&
    other.behind == behind &&
    other.isDirty == isDirty &&
    _deepEquality.equals(other.files, files);

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (id.hashCode) +
    (path.hashCode) +
    (branch.hashCode) +
    (upstream == null ? 0 : upstream!.hashCode) +
    (ahead.hashCode) +
    (behind.hashCode) +
    (isDirty.hashCode) +
    (files.hashCode);

  @override
  String toString() => 'GitWorktreeStatus[id=$id, path=$path, branch=$branch, upstream=$upstream, ahead=$ahead, behind=$behind, isDirty=$isDirty, files=$files]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'id'] = this.id;
      json[r'path'] = this.path;
      json[r'branch'] = this.branch;
    if (this.upstream != null) {
      json[r'upstream'] = this.upstream;
    } else {
      json[r'upstream'] = null;
    }
      json[r'ahead'] = this.ahead;
      json[r'behind'] = this.behind;
      json[r'is_dirty'] = this.isDirty;
      json[r'files'] = this.files;
    return json;
  }

  /// Returns a new [GitWorktreeStatus] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static GitWorktreeStatus? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "GitWorktreeStatus[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "GitWorktreeStatus[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return GitWorktreeStatus(
        id: mapValueOfType<String>(json, r'id')!,
        path: mapValueOfType<String>(json, r'path')!,
        branch: mapValueOfType<String>(json, r'branch')!,
        upstream: mapValueOfType<String>(json, r'upstream'),
        ahead: mapValueOfType<int>(json, r'ahead')!,
        behind: mapValueOfType<int>(json, r'behind')!,
        isDirty: mapValueOfType<bool>(json, r'is_dirty')!,
        files: GitStatusFileChange.listFromJson(json[r'files']),
      );
    }
    return null;
  }

  static List<GitWorktreeStatus> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <GitWorktreeStatus>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = GitWorktreeStatus.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, GitWorktreeStatus> mapFromJson(dynamic json) {
    final map = <String, GitWorktreeStatus>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = GitWorktreeStatus.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of GitWorktreeStatus-objects as value to a dart map
  static Map<String, List<GitWorktreeStatus>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<GitWorktreeStatus>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = GitWorktreeStatus.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'id',
    'path',
    'branch',
    'ahead',
    'behind',
    'is_dirty',
    'files',
  };
}


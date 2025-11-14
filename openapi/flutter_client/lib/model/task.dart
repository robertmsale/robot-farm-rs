//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class Task {
  /// Returns a new [Task] instance.
  Task({
    required this.id,
    required this.groupId,
    required this.slug,
    required this.title,
    this.commitHash,
    required this.status,
    required this.owner,
    required this.description,
  });

  int id;

  int groupId;

  String slug;

  String title;

  /// Commit hash associated with the task.
  ///
  /// Please note: This property should have been non-nullable! Since the specification file
  /// does not include a default value (using the "default:" property), however, the generated
  /// source code must fall back to having a nullable type.
  /// Consider adding a "default:" property in the specification file to hide this note.
  ///
  String? commitHash;

  TaskStatus status;

  /// Owner information encoded as display text (\"Orchestrator\", \"Quality Assurance\", or worker handles like \"ws42\").
  String owner;

  /// Detailed description of the task.
  String description;

  @override
  bool operator ==(Object other) => identical(this, other) || other is Task &&
    other.id == id &&
    other.groupId == groupId &&
    other.slug == slug &&
    other.title == title &&
    other.commitHash == commitHash &&
    other.status == status &&
    other.owner == owner &&
    other.description == description;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (id.hashCode) +
    (groupId.hashCode) +
    (slug.hashCode) +
    (title.hashCode) +
    (commitHash == null ? 0 : commitHash!.hashCode) +
    (status.hashCode) +
    (owner.hashCode) +
    (description.hashCode);

  @override
  String toString() => 'Task[id=$id, groupId=$groupId, slug=$slug, title=$title, commitHash=$commitHash, status=$status, owner=$owner, description=$description]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'id'] = this.id;
      json[r'group_id'] = this.groupId;
      json[r'slug'] = this.slug;
      json[r'title'] = this.title;
    if (this.commitHash != null) {
      json[r'commit_hash'] = this.commitHash;
    } else {
      json[r'commit_hash'] = null;
    }
      json[r'status'] = this.status;
      json[r'owner'] = this.owner;
      json[r'description'] = this.description;
    return json;
  }

  /// Returns a new [Task] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static Task? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "Task[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "Task[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return Task(
        id: mapValueOfType<int>(json, r'id')!,
        groupId: mapValueOfType<int>(json, r'group_id')!,
        slug: mapValueOfType<String>(json, r'slug')!,
        title: mapValueOfType<String>(json, r'title')!,
        commitHash: mapValueOfType<String>(json, r'commit_hash'),
        status: TaskStatus.fromJson(json[r'status'])!,
        owner: mapValueOfType<String>(json, r'owner')!,
        description: mapValueOfType<String>(json, r'description')!,
      );
    }
    return null;
  }

  static List<Task> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <Task>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = Task.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, Task> mapFromJson(dynamic json) {
    final map = <String, Task>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = Task.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of Task-objects as value to a dart map
  static Map<String, List<Task>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<Task>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = Task.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'id',
    'group_id',
    'slug',
    'title',
    'status',
    'owner',
    'description',
  };
}


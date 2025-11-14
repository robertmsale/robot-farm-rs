//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class TaskUpdateInput {
  /// Returns a new [TaskUpdateInput] instance.
  TaskUpdateInput({
    this.groupId,
    this.slug,
    this.title,
    this.commitHash,
    this.status,
    this.owner,
    this.description,
  });

  ///
  /// Please note: This property should have been non-nullable! Since the specification file
  /// does not include a default value (using the "default:" property), however, the generated
  /// source code must fall back to having a nullable type.
  /// Consider adding a "default:" property in the specification file to hide this note.
  ///
  int? groupId;

  ///
  /// Please note: This property should have been non-nullable! Since the specification file
  /// does not include a default value (using the "default:" property), however, the generated
  /// source code must fall back to having a nullable type.
  /// Consider adding a "default:" property in the specification file to hide this note.
  ///
  String? slug;

  ///
  /// Please note: This property should have been non-nullable! Since the specification file
  /// does not include a default value (using the "default:" property), however, the generated
  /// source code must fall back to having a nullable type.
  /// Consider adding a "default:" property in the specification file to hide this note.
  ///
  String? title;

  /// Commit hash associated with the task.
  ///
  /// Please note: This property should have been non-nullable! Since the specification file
  /// does not include a default value (using the "default:" property), however, the generated
  /// source code must fall back to having a nullable type.
  /// Consider adding a "default:" property in the specification file to hide this note.
  ///
  String? commitHash;

  ///
  /// Please note: This property should have been non-nullable! Since the specification file
  /// does not include a default value (using the "default:" property), however, the generated
  /// source code must fall back to having a nullable type.
  /// Consider adding a "default:" property in the specification file to hide this note.
  ///
  TaskStatus? status;

  /// Owner information encoded as display text (\"Orchestrator\", \"Quality Assurance\", or worker handles like \"ws42\").
  ///
  /// Please note: This property should have been non-nullable! Since the specification file
  /// does not include a default value (using the "default:" property), however, the generated
  /// source code must fall back to having a nullable type.
  /// Consider adding a "default:" property in the specification file to hide this note.
  ///
  String? owner;

  /// Detailed description of the task.
  ///
  /// Please note: This property should have been non-nullable! Since the specification file
  /// does not include a default value (using the "default:" property), however, the generated
  /// source code must fall back to having a nullable type.
  /// Consider adding a "default:" property in the specification file to hide this note.
  ///
  String? description;

  @override
  bool operator ==(Object other) => identical(this, other) || other is TaskUpdateInput &&
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
    (groupId == null ? 0 : groupId!.hashCode) +
    (slug == null ? 0 : slug!.hashCode) +
    (title == null ? 0 : title!.hashCode) +
    (commitHash == null ? 0 : commitHash!.hashCode) +
    (status == null ? 0 : status!.hashCode) +
    (owner == null ? 0 : owner!.hashCode) +
    (description == null ? 0 : description!.hashCode);

  @override
  String toString() => 'TaskUpdateInput[groupId=$groupId, slug=$slug, title=$title, commitHash=$commitHash, status=$status, owner=$owner, description=$description]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
    if (this.groupId != null) {
      json[r'group_id'] = this.groupId;
    } else {
      json[r'group_id'] = null;
    }
    if (this.slug != null) {
      json[r'slug'] = this.slug;
    } else {
      json[r'slug'] = null;
    }
    if (this.title != null) {
      json[r'title'] = this.title;
    } else {
      json[r'title'] = null;
    }
    if (this.commitHash != null) {
      json[r'commit_hash'] = this.commitHash;
    } else {
      json[r'commit_hash'] = null;
    }
    if (this.status != null) {
      json[r'status'] = this.status;
    } else {
      json[r'status'] = null;
    }
    if (this.owner != null) {
      json[r'owner'] = this.owner;
    } else {
      json[r'owner'] = null;
    }
    if (this.description != null) {
      json[r'description'] = this.description;
    } else {
      json[r'description'] = null;
    }
    return json;
  }

  /// Returns a new [TaskUpdateInput] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static TaskUpdateInput? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "TaskUpdateInput[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "TaskUpdateInput[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return TaskUpdateInput(
        groupId: mapValueOfType<int>(json, r'group_id'),
        slug: mapValueOfType<String>(json, r'slug'),
        title: mapValueOfType<String>(json, r'title'),
        commitHash: mapValueOfType<String>(json, r'commit_hash'),
        status: TaskStatus.fromJson(json[r'status']),
        owner: mapValueOfType<String>(json, r'owner'),
        description: mapValueOfType<String>(json, r'description'),
      );
    }
    return null;
  }

  static List<TaskUpdateInput> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <TaskUpdateInput>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = TaskUpdateInput.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, TaskUpdateInput> mapFromJson(dynamic json) {
    final map = <String, TaskUpdateInput>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = TaskUpdateInput.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of TaskUpdateInput-objects as value to a dart map
  static Map<String, List<TaskUpdateInput>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<TaskUpdateInput>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = TaskUpdateInput.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
  };
}


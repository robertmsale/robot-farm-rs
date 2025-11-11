//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class Feed {
  /// Returns a new [Feed] instance.
  Feed({
    required this.id,
    required this.source_,
    required this.target,
    required this.ts,
    required this.level,
    required this.text,
    required this.raw,
    required this.category,
  });

  int id;

  /// Emitter of the feed entry (Orchestrator, Quality Assurance, System, ws#).
  String source_;

  /// Recipient focus (Orchestrator, Quality Assurance, System, ws#).
  String target;

  /// Unix timestamp in seconds.
  int ts;

  FeedLevel level;

  /// Human-friendly summary.
  String text;

  /// Raw log payload.
  String raw;

  /// Category tag for the entry.
  String category;

  @override
  bool operator ==(Object other) => identical(this, other) || other is Feed &&
    other.id == id &&
    other.source_ == source_ &&
    other.target == target &&
    other.ts == ts &&
    other.level == level &&
    other.text == text &&
    other.raw == raw &&
    other.category == category;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (id.hashCode) +
    (source_.hashCode) +
    (target.hashCode) +
    (ts.hashCode) +
    (level.hashCode) +
    (text.hashCode) +
    (raw.hashCode) +
    (category.hashCode);

  @override
  String toString() => 'Feed[id=$id, source_=$source_, target=$target, ts=$ts, level=$level, text=$text, raw=$raw, category=$category]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'id'] = this.id;
      json[r'source'] = this.source_;
      json[r'target'] = this.target;
      json[r'ts'] = this.ts;
      json[r'level'] = this.level;
      json[r'text'] = this.text;
      json[r'raw'] = this.raw;
      json[r'category'] = this.category;
    return json;
  }

  /// Returns a new [Feed] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static Feed? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "Feed[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "Feed[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return Feed(
        id: mapValueOfType<int>(json, r'id')!,
        source_: mapValueOfType<String>(json, r'source')!,
        target: mapValueOfType<String>(json, r'target')!,
        ts: mapValueOfType<int>(json, r'ts')!,
        level: FeedLevel.fromJson(json[r'level'])!,
        text: mapValueOfType<String>(json, r'text')!,
        raw: mapValueOfType<String>(json, r'raw')!,
        category: mapValueOfType<String>(json, r'category')!,
      );
    }
    return null;
  }

  static List<Feed> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <Feed>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = Feed.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, Feed> mapFromJson(dynamic json) {
    final map = <String, Feed>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = Feed.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of Feed-objects as value to a dart map
  static Map<String, List<Feed>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<Feed>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = Feed.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'id',
    'source',
    'target',
    'ts',
    'level',
    'text',
    'raw',
    'category',
  };
}


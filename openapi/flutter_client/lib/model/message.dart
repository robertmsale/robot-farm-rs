//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class Message {
  /// Returns a new [Message] instance.
  Message({
    required this.id,
    required this.from,
    required this.to,
    required this.message,
    required this.insertedAt,
  });

  int id;

  /// Message sender or recipient display value (\"Orchestrator\", \"Quality Assurance\", or worker handles like \"ws42\").
  String from;

  /// Message sender or recipient display value (\"Orchestrator\", \"Quality Assurance\", or worker handles like \"ws42\").
  String to;

  String message;

  /// Unix timestamp (seconds) when the message was queued.
  int insertedAt;

  @override
  bool operator ==(Object other) => identical(this, other) || other is Message &&
    other.id == id &&
    other.from == from &&
    other.to == to &&
    other.message == message &&
    other.insertedAt == insertedAt;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (id.hashCode) +
    (from.hashCode) +
    (to.hashCode) +
    (message.hashCode) +
    (insertedAt.hashCode);

  @override
  String toString() => 'Message[id=$id, from=$from, to=$to, message=$message, insertedAt=$insertedAt]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'id'] = this.id;
      json[r'from'] = this.from;
      json[r'to'] = this.to;
      json[r'message'] = this.message;
      json[r'inserted_at'] = this.insertedAt;
    return json;
  }

  /// Returns a new [Message] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static Message? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "Message[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "Message[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return Message(
        id: mapValueOfType<int>(json, r'id')!,
        from: mapValueOfType<String>(json, r'from')!,
        to: mapValueOfType<String>(json, r'to')!,
        message: mapValueOfType<String>(json, r'message')!,
        insertedAt: mapValueOfType<int>(json, r'inserted_at')!,
      );
    }
    return null;
  }

  static List<Message> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <Message>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = Message.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, Message> mapFromJson(dynamic json) {
    final map = <String, Message>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = Message.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of Message-objects as value to a dart map
  static Map<String, List<Message>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<Message>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = Message.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'id',
    'from',
    'to',
    'message',
    'inserted_at',
  };
}


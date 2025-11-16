//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class MessageEnqueueInput {
  /// Returns a new [MessageEnqueueInput] instance.
  MessageEnqueueInput({
    required this.from,
    required this.to,
    required this.message,
  });

  /// Message sender or recipient display value (\"Orchestrator\", \"Quality Assurance\", or worker handles like \"ws42\").
  String from;

  /// Message sender or recipient display value (\"Orchestrator\", \"Quality Assurance\", or worker handles like \"ws42\").
  String to;

  String message;

  @override
  bool operator ==(Object other) => identical(this, other) || other is MessageEnqueueInput &&
    other.from == from &&
    other.to == to &&
    other.message == message;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (from.hashCode) +
    (to.hashCode) +
    (message.hashCode);

  @override
  String toString() => 'MessageEnqueueInput[from=$from, to=$to, message=$message]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'from'] = this.from;
      json[r'to'] = this.to;
      json[r'message'] = this.message;
    return json;
  }

  /// Returns a new [MessageEnqueueInput] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static MessageEnqueueInput? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "MessageEnqueueInput[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "MessageEnqueueInput[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return MessageEnqueueInput(
        from: mapValueOfType<String>(json, r'from')!,
        to: mapValueOfType<String>(json, r'to')!,
        message: mapValueOfType<String>(json, r'message')!,
      );
    }
    return null;
  }

  static List<MessageEnqueueInput> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <MessageEnqueueInput>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = MessageEnqueueInput.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, MessageEnqueueInput> mapFromJson(dynamic json) {
    final map = <String, MessageEnqueueInput>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = MessageEnqueueInput.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of MessageEnqueueInput-objects as value to a dart map
  static Map<String, List<MessageEnqueueInput>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<MessageEnqueueInput>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = MessageEnqueueInput.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'from',
    'to',
    'message',
  };
}


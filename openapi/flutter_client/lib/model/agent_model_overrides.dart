//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class AgentModelOverrides {
  /// Returns a new [AgentModelOverrides] instance.
  AgentModelOverrides({
    required this.orchestrator,
    required this.worker,
    required this.wizard,
  });

  /// Codex model used for the orchestrator persona.
  AgentModelOverridesOrchestratorEnum orchestrator;

  /// Codex model used for worker personas.
  AgentModelOverridesWorkerEnum worker;

  /// Codex model used by the task wizard helper.
  AgentModelOverridesWizardEnum wizard;

  @override
  bool operator ==(Object other) => identical(this, other) || other is AgentModelOverrides &&
    other.orchestrator == orchestrator &&
    other.worker == worker &&
    other.wizard == wizard;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (orchestrator.hashCode) +
    (worker.hashCode) +
    (wizard.hashCode);

  @override
  String toString() => 'AgentModelOverrides[orchestrator=$orchestrator, worker=$worker, wizard=$wizard]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'orchestrator'] = this.orchestrator;
      json[r'worker'] = this.worker;
      json[r'wizard'] = this.wizard;
    return json;
  }

  /// Returns a new [AgentModelOverrides] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static AgentModelOverrides? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "AgentModelOverrides[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "AgentModelOverrides[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return AgentModelOverrides(
        orchestrator: AgentModelOverridesOrchestratorEnum.fromJson(json[r'orchestrator'])!,
        worker: AgentModelOverridesWorkerEnum.fromJson(json[r'worker'])!,
        wizard: AgentModelOverridesWizardEnum.fromJson(json[r'wizard'])!,
      );
    }
    return null;
  }

  static List<AgentModelOverrides> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <AgentModelOverrides>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = AgentModelOverrides.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, AgentModelOverrides> mapFromJson(dynamic json) {
    final map = <String, AgentModelOverrides>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = AgentModelOverrides.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of AgentModelOverrides-objects as value to a dart map
  static Map<String, List<AgentModelOverrides>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<AgentModelOverrides>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = AgentModelOverrides.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'orchestrator',
    'worker',
    'wizard',
  };
}

/// Codex model used for the orchestrator persona.
class AgentModelOverridesOrchestratorEnum {
  /// Instantiate a new enum with the provided [value].
  const AgentModelOverridesOrchestratorEnum._(this.value);

  /// The underlying value of this enum member.
  final String value;

  @override
  String toString() => value;

  String toJson() => value;

  static const gpt5Period1CodexMax = AgentModelOverridesOrchestratorEnum._(r'gpt-5.1-codex-max');
  static const gpt5Period1Codex = AgentModelOverridesOrchestratorEnum._(r'gpt-5.1-codex');
  static const gpt5Period1CodexMini = AgentModelOverridesOrchestratorEnum._(r'gpt-5.1-codex-mini');
  static const gpt5Period1 = AgentModelOverridesOrchestratorEnum._(r'gpt-5.1');

  /// List of all possible values in this [enum][AgentModelOverridesOrchestratorEnum].
  static const values = <AgentModelOverridesOrchestratorEnum>[
    gpt5Period1CodexMax,
    gpt5Period1Codex,
    gpt5Period1CodexMini,
    gpt5Period1,
  ];

  static AgentModelOverridesOrchestratorEnum? fromJson(dynamic value) => AgentModelOverridesOrchestratorEnumTypeTransformer().decode(value);

  static List<AgentModelOverridesOrchestratorEnum> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <AgentModelOverridesOrchestratorEnum>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = AgentModelOverridesOrchestratorEnum.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }
}

/// Transformation class that can [encode] an instance of [AgentModelOverridesOrchestratorEnum] to String,
/// and [decode] dynamic data back to [AgentModelOverridesOrchestratorEnum].
class AgentModelOverridesOrchestratorEnumTypeTransformer {
  factory AgentModelOverridesOrchestratorEnumTypeTransformer() => _instance ??= const AgentModelOverridesOrchestratorEnumTypeTransformer._();

  const AgentModelOverridesOrchestratorEnumTypeTransformer._();

  String encode(AgentModelOverridesOrchestratorEnum data) => data.value;

  /// Decodes a [dynamic value][data] to a AgentModelOverridesOrchestratorEnum.
  ///
  /// If [allowNull] is true and the [dynamic value][data] cannot be decoded successfully,
  /// then null is returned. However, if [allowNull] is false and the [dynamic value][data]
  /// cannot be decoded successfully, then an [UnimplementedError] is thrown.
  ///
  /// The [allowNull] is very handy when an API changes and a new enum value is added or removed,
  /// and users are still using an old app with the old code.
  AgentModelOverridesOrchestratorEnum? decode(dynamic data, {bool allowNull = true}) {
    if (data != null) {
      switch (data) {
        case r'gpt-5.1-codex-max': return AgentModelOverridesOrchestratorEnum.gpt5Period1CodexMax;
        case r'gpt-5.1-codex': return AgentModelOverridesOrchestratorEnum.gpt5Period1Codex;
        case r'gpt-5.1-codex-mini': return AgentModelOverridesOrchestratorEnum.gpt5Period1CodexMini;
        case r'gpt-5.1': return AgentModelOverridesOrchestratorEnum.gpt5Period1;
        default:
          if (!allowNull) {
            throw ArgumentError('Unknown enum value to decode: $data');
          }
      }
    }
    return null;
  }

  /// Singleton [AgentModelOverridesOrchestratorEnumTypeTransformer] instance.
  static AgentModelOverridesOrchestratorEnumTypeTransformer? _instance;
}


/// Codex model used for worker personas.
class AgentModelOverridesWorkerEnum {
  /// Instantiate a new enum with the provided [value].
  const AgentModelOverridesWorkerEnum._(this.value);

  /// The underlying value of this enum member.
  final String value;

  @override
  String toString() => value;

  String toJson() => value;

  static const gpt5Period1CodexMax = AgentModelOverridesWorkerEnum._(r'gpt-5.1-codex-max');
  static const gpt5Period1Codex = AgentModelOverridesWorkerEnum._(r'gpt-5.1-codex');
  static const gpt5Period1CodexMini = AgentModelOverridesWorkerEnum._(r'gpt-5.1-codex-mini');
  static const gpt5Period1 = AgentModelOverridesWorkerEnum._(r'gpt-5.1');

  /// List of all possible values in this [enum][AgentModelOverridesWorkerEnum].
  static const values = <AgentModelOverridesWorkerEnum>[
    gpt5Period1CodexMax,
    gpt5Period1Codex,
    gpt5Period1CodexMini,
    gpt5Period1,
  ];

  static AgentModelOverridesWorkerEnum? fromJson(dynamic value) => AgentModelOverridesWorkerEnumTypeTransformer().decode(value);

  static List<AgentModelOverridesWorkerEnum> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <AgentModelOverridesWorkerEnum>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = AgentModelOverridesWorkerEnum.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }
}

/// Transformation class that can [encode] an instance of [AgentModelOverridesWorkerEnum] to String,
/// and [decode] dynamic data back to [AgentModelOverridesWorkerEnum].
class AgentModelOverridesWorkerEnumTypeTransformer {
  factory AgentModelOverridesWorkerEnumTypeTransformer() => _instance ??= const AgentModelOverridesWorkerEnumTypeTransformer._();

  const AgentModelOverridesWorkerEnumTypeTransformer._();

  String encode(AgentModelOverridesWorkerEnum data) => data.value;

  /// Decodes a [dynamic value][data] to a AgentModelOverridesWorkerEnum.
  ///
  /// If [allowNull] is true and the [dynamic value][data] cannot be decoded successfully,
  /// then null is returned. However, if [allowNull] is false and the [dynamic value][data]
  /// cannot be decoded successfully, then an [UnimplementedError] is thrown.
  ///
  /// The [allowNull] is very handy when an API changes and a new enum value is added or removed,
  /// and users are still using an old app with the old code.
  AgentModelOverridesWorkerEnum? decode(dynamic data, {bool allowNull = true}) {
    if (data != null) {
      switch (data) {
        case r'gpt-5.1-codex-max': return AgentModelOverridesWorkerEnum.gpt5Period1CodexMax;
        case r'gpt-5.1-codex': return AgentModelOverridesWorkerEnum.gpt5Period1Codex;
        case r'gpt-5.1-codex-mini': return AgentModelOverridesWorkerEnum.gpt5Period1CodexMini;
        case r'gpt-5.1': return AgentModelOverridesWorkerEnum.gpt5Period1;
        default:
          if (!allowNull) {
            throw ArgumentError('Unknown enum value to decode: $data');
          }
      }
    }
    return null;
  }

  /// Singleton [AgentModelOverridesWorkerEnumTypeTransformer] instance.
  static AgentModelOverridesWorkerEnumTypeTransformer? _instance;
}


/// Codex model used by the task wizard helper.
class AgentModelOverridesWizardEnum {
  /// Instantiate a new enum with the provided [value].
  const AgentModelOverridesWizardEnum._(this.value);

  /// The underlying value of this enum member.
  final String value;

  @override
  String toString() => value;

  String toJson() => value;

  static const gpt5Period1CodexMax = AgentModelOverridesWizardEnum._(r'gpt-5.1-codex-max');
  static const gpt5Period1Codex = AgentModelOverridesWizardEnum._(r'gpt-5.1-codex');
  static const gpt5Period1CodexMini = AgentModelOverridesWizardEnum._(r'gpt-5.1-codex-mini');
  static const gpt5Period1 = AgentModelOverridesWizardEnum._(r'gpt-5.1');

  /// List of all possible values in this [enum][AgentModelOverridesWizardEnum].
  static const values = <AgentModelOverridesWizardEnum>[
    gpt5Period1CodexMax,
    gpt5Period1Codex,
    gpt5Period1CodexMini,
    gpt5Period1,
  ];

  static AgentModelOverridesWizardEnum? fromJson(dynamic value) => AgentModelOverridesWizardEnumTypeTransformer().decode(value);

  static List<AgentModelOverridesWizardEnum> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <AgentModelOverridesWizardEnum>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = AgentModelOverridesWizardEnum.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }
}

/// Transformation class that can [encode] an instance of [AgentModelOverridesWizardEnum] to String,
/// and [decode] dynamic data back to [AgentModelOverridesWizardEnum].
class AgentModelOverridesWizardEnumTypeTransformer {
  factory AgentModelOverridesWizardEnumTypeTransformer() => _instance ??= const AgentModelOverridesWizardEnumTypeTransformer._();

  const AgentModelOverridesWizardEnumTypeTransformer._();

  String encode(AgentModelOverridesWizardEnum data) => data.value;

  /// Decodes a [dynamic value][data] to a AgentModelOverridesWizardEnum.
  ///
  /// If [allowNull] is true and the [dynamic value][data] cannot be decoded successfully,
  /// then null is returned. However, if [allowNull] is false and the [dynamic value][data]
  /// cannot be decoded successfully, then an [UnimplementedError] is thrown.
  ///
  /// The [allowNull] is very handy when an API changes and a new enum value is added or removed,
  /// and users are still using an old app with the old code.
  AgentModelOverridesWizardEnum? decode(dynamic data, {bool allowNull = true}) {
    if (data != null) {
      switch (data) {
        case r'gpt-5.1-codex-max': return AgentModelOverridesWizardEnum.gpt5Period1CodexMax;
        case r'gpt-5.1-codex': return AgentModelOverridesWizardEnum.gpt5Period1Codex;
        case r'gpt-5.1-codex-mini': return AgentModelOverridesWizardEnum.gpt5Period1CodexMini;
        case r'gpt-5.1': return AgentModelOverridesWizardEnum.gpt5Period1;
        default:
          if (!allowNull) {
            throw ArgumentError('Unknown enum value to decode: $data');
          }
      }
    }
    return null;
  }

  /// Singleton [AgentModelOverridesWizardEnumTypeTransformer] instance.
  static AgentModelOverridesWizardEnumTypeTransformer? _instance;
}



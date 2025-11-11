use crate::errors::{FindingKind, Report};
use crate::walk::walk_schema;
use schemars::Schema;
use serde_json::{Map, Value};

/// Configuration for what we consider "OpenAI safe" in `strict: true` mode.
/// This is a conservative profile that aligns with OpenAI guidance and real-world API behavior:
///  - Force `additionalProperties: false` on all objects (typed maps are flagged).
///  - Root must be an object.
///  - Every `required` key must exist in `properties`.
///  - Arrays must define `items`.
///  - Disallow empty `{}` schemas in `additionalProperties`.
///  - Warn on use of oneOf/anyOf/allOf/not (supported in some cases, but fragile).
///  - Warn if `enum` types don't match declared `type`.
#[derive(Debug, Clone)]
pub struct Rules {
    pub require_root_object: bool,
    pub require_additional_props_false_everywhere: bool,
    pub forbid_typed_additional_props_in_strict: bool,
    pub forbid_empty_additional_props_schema: bool,
    pub require_array_items: bool,
    pub warn_on_oneof_anyof_allof_not: bool,
    pub warn_on_enum_type_mismatch: bool,
    pub max_nesting: Option<usize>,
    pub max_total_properties: Option<usize>,
    pub forbid_keywords_alongside_refs: bool,
}

impl Default for Rules {
    fn default() -> Self {
        StrictProfile::OpenAI2025.default_rules()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StrictProfile {
    /// Conservative, errs on the safe side for OpenAI strict structured outputs.
    OpenAI2025,
}

impl StrictProfile {
    pub fn default_rules(self) -> Rules {
        match self {
            StrictProfile::OpenAI2025 => Rules {
                require_root_object: true,
                require_additional_props_false_everywhere: true,
                forbid_typed_additional_props_in_strict: true,
                forbid_empty_additional_props_schema: true,
                require_array_items: true,
                warn_on_oneof_anyof_allof_not: true,
                warn_on_enum_type_mismatch: true,
                forbid_keywords_alongside_refs: true,
                max_nesting: Some(5),              // conservative default
                max_total_properties: Some(100),   // conservative default
            }
        }
    }
}

pub fn validate_schema(root: &Schema, rules: &Rules) -> Report {
    let mut rep = Report::default();

    // Root must be an object
    if rules.require_root_object {
        let is_obj = match root.as_value() {
            Value::Object(map) => has_type(map, "object") || map.contains_key("properties"),
            Value::Bool(_) => false,
            _ => false,
        };
        if !is_obj {
            rep.push("/".into(), FindingKind::Error, "Root schema must have type: object");
        }
    }

    // Count properties / nesting while we walk
    let mut total_props: usize = 0;

    walk_schema(root, |ctx, sobj| {
        // Detect object subschema
        if is_object_candidate(sobj) {
            let properties = sobj.get("properties").and_then(Value::as_object);
            // Count properties
            if let Some(props) = properties {
                total_props += props.len();
            }

            // additionalProperties checks
            match sobj.get("additionalProperties") {
                Some(Value::Bool(true)) if rules.require_additional_props_false_everywhere => {
                    rep.push(
                        ctx.pointer.clone(),
                        FindingKind::Error,
                        "additionalProperties must be false under strict mode",
                    );
                }
                None if rules.require_additional_props_false_everywhere => {
                    rep.push(
                        ctx.pointer.clone(),
                        FindingKind::Error,
                        "missing additionalProperties (must be false in strict mode)",
                    );
                }
                Some(Value::Object(obj)) => {
                    if rules.forbid_typed_additional_props_in_strict {
                        rep.push(
                            ctx.pointer.clone(),
                            FindingKind::Error,
                            "typed additionalProperties is not allowed in OpenAI strict mode (declare explicit keys instead)",
                        );
                    } else if rules.forbid_empty_additional_props_schema && is_empty_schema(obj) {
                        rep.push(
                            ctx.pointer.clone(),
                            FindingKind::Error,
                            "empty schema in additionalProperties is not allowed; use false or a concrete type",
                        );
                    }
                }
                _ => {}
            }

            // required must exist in properties
            if let Some(Value::Array(required)) = sobj.get("required") {
                for entry in required {
                    if let Some(name) = entry.as_str() {
                        let defined = properties.map_or(false, |props| props.contains_key(name));
                        if !defined {
                            rep.push(
                                format!("{}/required", ctx.pointer),
                                FindingKind::Error,
                                format!(
                                    "required property '{}' is not defined in properties",
                                    name
                                ),
                            );
                        }
                    }
                }
            }
        }

        // Arrays must define items under strict
        if rules.require_array_items && is_array_candidate(sobj) && !sobj.contains_key("items") {
            rep.push(
                ctx.pointer.clone(),
                FindingKind::Error,
                "array must specify 'items' schema",
            );
        }

        // $ref must stand alone per OpenAI requirements
        if rules.forbid_keywords_alongside_refs {
            if let Some(Value::String(_)) = sobj.get("$ref") {
                if sobj
                    .keys()
                    .any(|k| k != "$ref")
                {
                    rep.push(
                        ctx.pointer.clone(),
                        FindingKind::Error,
                        "$ref schemas cannot include other keywords in OpenAI strict mode",
                    );
                }
            }
        }

        // Combinators are fragile with strict decoding
        if rules.warn_on_oneof_anyof_allof_not {
            if has_nonempty_array(sobj, "oneOf") {
                rep.push(
                    ctx.pointer.clone(),
                    FindingKind::Warning,
                    "oneOf used — supported in some cases but fragile with strict decoding",
                );
            }
            if has_nonempty_array(sobj, "anyOf") {
                rep.push(
                    ctx.pointer.clone(),
                    FindingKind::Warning,
                    "anyOf used — supported in some cases but fragile with strict decoding",
                );
            }
            if has_nonempty_array(sobj, "allOf") {
                rep.push(
                    ctx.pointer.clone(),
                    FindingKind::Warning,
                    "allOf used — supported in some cases but fragile with strict decoding",
                );
            }
            if sobj.contains_key("not") {
                rep.push(
                    ctx.pointer.clone(),
                    FindingKind::Warning,
                    "not used — may complicate strict decoding",
                );
            }
        }

        // enum vs type matching (warn)
        if rules.warn_on_enum_type_mismatch {
            if let Some(Value::Array(enums)) = sobj.get("enum") {
                let enum_all_strings = enums.iter().all(|v| v.as_str().is_some());
                if enum_all_strings && !has_type(sobj, "string") {
                    rep.push(
                        ctx.pointer.clone(),
                        FindingKind::Warning,
                        "enum has only strings but 'type' does not include 'string'",
                    );
                }
            }
        }
    });

    // Global quotas
    if let Some(max) = rules.max_total_properties {
        if total_props > max {
            rep.push("/".into(), FindingKind::Warning,
                     format!("schema has {total_props} total object properties; consider keeping ≤ {max} for reliability"));
        }
    }

    rep
}

fn has_type(map: &Map<String, Value>, needle: &str) -> bool {
    match map.get("type") {
        Some(Value::String(s)) => s == needle,
        Some(Value::Array(arr)) => arr.iter().any(|v| v.as_str() == Some(needle)),
        _ => false,
    }
}

fn is_object_candidate(map: &Map<String, Value>) -> bool {
    has_type(map, "object")
        || map.contains_key("properties")
        || map.contains_key("required")
        || map.contains_key("additionalProperties")
}

fn is_array_candidate(map: &Map<String, Value>) -> bool {
    has_type(map, "array") || map.contains_key("items") || map.contains_key("prefixItems")
}

fn has_nonempty_array(map: &Map<String, Value>, keyword: &str) -> bool {
    map.get(keyword)
        .and_then(Value::as_array)
        .map(|arr| !arr.is_empty())
        .unwrap_or(false)
}

fn is_empty_schema(value: &Map<String, Value>) -> bool {
    const SIGNAL_KEYS: &[&str] = &[
        "type",
        "properties",
        "patternProperties",
        "required",
        "items",
        "anyOf",
        "allOf",
        "oneOf",
        "not",
        "$ref",
        "enum",
        "const",
        "format",
        "minimum",
        "maximum",
    ];
    !value.keys().any(|k| SIGNAL_KEYS.contains(&k.as_str()))
}

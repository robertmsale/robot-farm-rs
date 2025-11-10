use crate::errors::{FindingKind, Report};
use crate::walk::{walk_schema, VisitContext};
use indexmap::IndexMap;
use schemars::schema::{InstanceType, RootSchema, Schema, SchemaObject};

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
                max_nesting: Some(5),              // conservative default
                max_total_properties: Some(100),   // conservative default
            }
        }
    }
}

pub fn validate_schema(root: &RootSchema, rules: &Rules) -> Report {
    let mut rep = Report::default();

    // Root must be an object
    if rules.require_root_object {
        let root_obj = &root.schema;
        let is_obj = match root_obj {
            Schema::Bool(_) => false,
            Schema::Object(sobj) => sobj.instance_type.as_ref()
                .map(|t| t.iter().any(|x| *x == InstanceType::Object)).unwrap_or(false),
        };
        if !is_obj {
            rep.push("/".into(), FindingKind::Error, "Root schema must have type: object");
        }
    }

    // Count properties / nesting while we walk
    let mut total_props: usize = 0;

    walk_schema(root, |ctx, sobj| {
        // Detect object subschema
        if let Some(ov) = &sobj.object {
            // Count properties
            total_props += ov.properties.len();

            // additionalProperties checks
            if let Some(ap) = &ov.additional_properties {
                match ap {
                    schemars::schema::AdditionalProperties::Boolean(b) => {
                        if *b && rules.require_additional_props_false_everywhere {
                            rep.push(ctx.pointer.clone(), FindingKind::Error,
                                     "additionalProperties must be false under strict mode");
                        }
                    }
                    schemars::schema::AdditionalProperties::Schema(s) => {
                        if rules.forbid_typed_additional_props_in_strict {
                            rep.push(ctx.pointer.clone(), FindingKind::Error,
                                     "typed additionalProperties is not allowed in OpenAI strict mode (declare explicit keys instead)");
                        } else if rules.forbid_empty_additional_props_schema {
                            if let Schema::Object(inner) = s {
                                if inner.object.is_none()
                                    && inner.array.is_none()
                                    && inner.instance_type.is_none()
                                {
                                    rep.push(ctx.pointer.clone(), FindingKind::Error,
                                             "empty schema in additionalProperties is not allowed; use false or a concrete type");
                                }
                            }
                        }
                    }
                }
            } else if rules.require_additional_props_false_everywhere {
                rep.push(ctx.pointer.clone(), FindingKind::Error,
                         "missing additionalProperties (must be false in strict mode)");
            }

            // required must exist in properties
            for k in &ov.required {
                if !ov.properties.contains_key(k) {
                    rep.push(format!("{}/required", ctx.pointer), FindingKind::Error,
                             format!("required property '{}' is not defined in properties", k));
                }
            }
        }

        // Arrays must define items under strict
        if let Some(av) = &sobj.array {
            if rules.require_array_items && av.items.is_none() {
                rep.push(ctx.pointer.clone(), FindingKind::Error,
                         "array must specify 'items' schema");
            }
        }

        // Combinators are fragile with strict decoding
        if rules.warn_on_oneof_anyof_allof_not {
            if sobj.one_of.as_ref().map(|v| !v.is_empty()).unwrap_or(false) {
                rep.push(ctx.pointer.clone(), FindingKind::Warning,
                         "oneOf used — supported in some cases but fragile with strict decoding");
            }
            if sobj.any_of.as_ref().map(|v| !v.is_empty()).unwrap_or(false) {
                rep.push(ctx.pointer.clone(), FindingKind::Warning,
                         "anyOf used — supported in some cases but fragile with strict decoding");
            }
            if sobj.all_of.as_ref().map(|v| !v.is_empty()).unwrap_or(false) {
                rep.push(ctx.pointer.clone(), FindingKind::Warning,
                         "allOf used — supported in some cases but fragile with strict decoding");
            }
            if sobj.not.is_some() {
                rep.push(ctx.pointer.clone(), FindingKind::Warning,
                         "not used — may complicate strict decoding");
            }
        }

        // enum vs type matching (warn)
        if rules.warn_on_enum_type_mismatch {
            if let Some(enums) = &sobj.enum_values {
                if let Some(ty) = &sobj.instance_type {
                    // best-effort: if declared type doesn't include String but enums are strings, warn
                    let has_string = ty.iter().any(|t| *t == InstanceType::String);
                    let enum_all_strings = enums.iter().all(|v| v.as_str().is_some());
                    if enum_all_strings && !has_string {
                        rep.push(ctx.pointer.clone(), FindingKind::Warning,
                                 "enum has only strings but 'type' does not include 'string'");
                    }
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

use schemars::Schema;
use serde_json::Value;

/// Recursively force per-OpenAI-strict expectations into the schema.
/// Currently:
///   - Ensure every object has `additionalProperties: false`
///   - Recurse through all nested subschemas so defaults propagate everywhere.
pub fn harden_root_schema(root: &mut Schema) {
    harden_schema(root);
}

fn harden_schema(schema: &mut Schema) {
    if schema.as_bool().is_some() {
        return;
    }

    let map = schema.ensure_object();
    enforce_additional_properties(map);

    visit_child_maps(map, |child| {
        if let Some(sub) = schema_from_value_mut(child) {
            harden_schema(sub);
        }
    });
}

fn enforce_additional_properties(map: &mut serde_json::Map<String, Value>) {
    match map.get_mut("additionalProperties") {
        None => {
            map.insert("additionalProperties".into(), Value::Bool(false));
        }
        Some(Value::Bool(true)) => {
            map.insert("additionalProperties".into(), Value::Bool(false));
        }
        Some(Value::Bool(false)) => {}
        Some(value @ Value::Object(_)) => {
            if let Some(sub) = schema_from_value_mut(value) {
                harden_schema(sub);
            }
        }
        Some(_) => {
            // Non-standard, but recurse defensively if it represents a schema.
            if let Some(sub) = schema_from_value_mut(map.get_mut("additionalProperties").unwrap()) {
                harden_schema(sub);
            }
        }
    }
}

fn visit_child_maps<F>(map: &mut serde_json::Map<String, Value>, mut f: F)
where
    F: FnMut(&mut Value),
{
    if let Some(Value::Object(props)) = map.get_mut("properties") {
        for value in props.values_mut() {
            f(value);
        }
    }

    if let Some(Value::Object(props)) = map.get_mut("patternProperties") {
        for value in props.values_mut() {
            f(value);
        }
    }

    if let Some(value) = map.get_mut("additionalProperties") {
        if !value.is_boolean() {
            f(value);
        }
    }

    if let Some(defs) = map.get_mut("$defs") {
        visit_def_container(defs, &mut f);
    }
    if let Some(defs) = map.get_mut("definitions") {
        visit_def_container(defs, &mut f);
    }

    if let Some(Value::Object(dependents)) = map.get_mut("dependentSchemas") {
        for value in dependents.values_mut() {
            f(value);
        }
    }

    if let Some(items) = map.get_mut("items") {
        match items {
            Value::Array(arr) => {
                for value in arr {
                    f(value);
                }
            }
            _ => f(items),
        }
    }

    if let Some(Value::Array(arr)) = map.get_mut("prefixItems") {
        for value in arr {
            f(value);
        }
    }

    for keyword in ["anyOf", "allOf", "oneOf"] {
        if let Some(Value::Array(arr)) = map.get_mut(keyword) {
            for value in arr {
                f(value);
            }
        }
    }

    if let Some(value) = map.get_mut("not") {
        f(value);
    }
    if let Some(value) = map.get_mut("if") {
        f(value);
    }
    if let Some(value) = map.get_mut("then") {
        f(value);
    }
    if let Some(value) = map.get_mut("else") {
        f(value);
    }
    if let Some(value) = map.get_mut("contains") {
        f(value);
    }
    if let Some(value) = map.get_mut("propertyNames") {
        f(value);
    }
}

fn schema_from_value_mut(value: &mut Value) -> Option<&mut Schema> {
    match value {
        Value::Object(_) | Value::Bool(_) => <&mut Schema>::try_from(value).ok(),
        _ => None,
    }
}

fn visit_def_container<F>(value: &mut Value, f: &mut F)
where
    F: FnMut(&mut Value),
{
    match value {
        Value::Object(obj) => {
            for value in obj.values_mut() {
                f(value);
            }
        }
        Value::Array(values) => {
            for value in values {
                f(value);
            }
        }
        _ => {}
    }
}

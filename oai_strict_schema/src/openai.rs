use crate::{
    errors::Report,
    harden_root_schema,
    rules::{validate_schema, Rules},
};
use schemars::{schema_for, JsonSchema, Schema};
use serde_json::Value;

/// Build the `"text.format"` payload expected by the OpenAI Responses API.
/// Returns the payload alongside the validation report so callers can decide
/// whether to fail fast on warnings.
pub fn responses_text_format_for<T: JsonSchema>(name: &str, rules: &Rules) -> (Value, Report) {
    let mut schema = schema_for!(T);
    strip_keywords_alongside_refs(&mut schema);
    harden_root_schema(&mut schema);
    let report = validate_schema(&schema, rules);

    let format_val = serde_json::json!({
        "type": "json_schema",
        "json_schema": {
            "name": name,
            "strict": true,
            "schema": schema_value(schema), // embed just the schema document
        }
    });

    (format_val, report)
}

/// Chat Completions legacy helper mirrors the Responses API format.
pub fn chat_response_format_for<T: JsonSchema>(name: &str, rules: &Rules) -> (Value, Report) {
    responses_text_format_for::<T>(name, rules)
}

fn schema_value(schema: Schema) -> Value {
    // `Schema` implements `Serialize`, so conversion is infallible.
    serde_json::to_value(schema).expect("schema serialization never fails")
}

fn strip_keywords_alongside_refs(schema: &mut Schema) {
    let mut value = serde_json::to_value(&*schema).expect("schema serialization succeeds");
    scrub_ref_objects(&mut value);
    *schema = Schema::try_from(value).expect("sanitized schema remains valid");
}

fn scrub_ref_objects(value: &mut Value) {
    match value {
        Value::Object(map) => {
            if let Some(reference) = map.get("$ref").cloned() {
                map.clear();
                map.insert("$ref".to_string(), reference);
                return;
            }
            for child in map.values_mut() {
                scrub_ref_objects(child);
            }
        }
        Value::Array(items) => {
            for child in items {
                scrub_ref_objects(child);
            }
        }
        _ => {}
    }
}

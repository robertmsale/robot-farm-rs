use crate::{harden_root_schema, rules::{validate_schema, Rules}, errors::Report};
use schemars::{schema_for, JsonSchema};
use serde_json::Value;

/// Build the value you can drop into the **Responses API** request under:
/// ```json
/// "text": {
///   "format": { "type": "json_schema", "json_schema": { "name": "...", "strict": true, "schema": { ... } } }
/// }
/// ```
/// NOTE: we return `(format_value, validation_report)` so you can decide whether to fail fast.
pub fn responses_text_format_for<T: JsonSchema>(
    name: &str,
    rules: &Rules,
) -> (Value, Report) {
    let mut root = schema_for!(T);
    // Harden then validate
    harden_root_schema(&mut root);
    let report = validate_schema(&root, rules);

    let format_val = serde_json::json!({
        "type": "json_schema",
        "json_schema": {
            "name": name,
            "strict": true,
            "schema": root.schema, // only the inner schema, not $defs metadata
        }
    });
    (format_val, report)
}

/// If you still use Chat Completions (legacy), this builds the old container:
/// { "type": "json_schema", "json_schema": { "name": "...", "strict": true, "schema": { ... } } }
pub fn chat_response_format_for<T: JsonSchema>(
    name: &str,
    rules: &Rules,
) -> (Value, Report) {
    responses_text_format_for::<T>(name, rules)
}

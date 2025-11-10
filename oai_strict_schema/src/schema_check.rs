use crate::{responses_text_format_for, Rules, StrictProfile};
use crate::StrictProfile;
use crate::types::responses_text_format_for;

/// Build the exact value you pass in `text.format` for `/v1/responses`.
pub fn build_text_format() -> serde_json::Value {
    let rules = StrictProfile::OpenAI2025.default_rules();
    let (format_val, report) = responses_text_format_for::<crate::types::MathReasoning>(
        "math_reasoning",
        &rules,
    );

    // Fail fast in dev: aggregate all problems at once
    if report.has_errors() {
        panic!("\nStrict schema validation failed:\n{}", report);
    } else if !report.is_ok() {
        eprintln!("\nStrict schema validation warnings:\n{}", report);
    }

    format_val
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn schema_is_openai_strict_valid() {
        let _ = build_text_format(); // panics with all issues if invalid
    }
}

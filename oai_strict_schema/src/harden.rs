use schemars::schema::{
    AdditionalProperties, ArrayValidation, InstanceType, ObjectValidation, RootSchema, Schema,
    SchemaObject, SingleOrVec,
};

/// Recursively force per-OpenAI-strict expectations into the schema.
/// Currently:
///   - Ensure every object has `additionalProperties: false`
///   - (Optionally) normalize empty "items" to an empty schema if missing (we leave missing as-is)
pub fn harden_root_schema(root: &mut RootSchema) {
    harden_schema(&mut root.schema);
}

fn harden_schema(schema: &mut Schema) {
    match schema {
        Schema::Bool(_) => {}
        Schema::Object(obj) => {
            if let Some(ov) = obj.object.as_mut() {
                // Force additionalProperties: false if absent or `true`.
                match &ov.additional_properties {
                    None => ov.additional_properties = Some(AdditionalProperties::Boolean(false)),
                    Some(AdditionalProperties::Boolean(true)) => {
                        ov.additional_properties = Some(AdditionalProperties::Boolean(false))
                    }
                    Some(AdditionalProperties::Boolean(false)) => {}
                    // If the user intentionally used typed additionalProperties, we keep it:
                    Some(AdditionalProperties::Schema(_)) => { /* leave; validator may flag */ }
                }

                // Recurse into properties
                for (_, subschema) in ov.properties.iter_mut() {
                    harden_schema(subschema);
                }
                for (_, subschema) in ov.pattern_properties.iter_mut() {
                    harden_schema(subschema);
                }
                if let Some(AdditionalProperties::Schema(sub)) = ov.additional_properties.as_mut() {
                    harden_schema(sub);
                }
            }

            if let Some(av) = obj.array.as_mut() {
                match &mut av.items {
                    Some(SingleOrVec::Single(s)) => harden_schema(s),
                    Some(SingleOrVec::Vec(v)) => v.iter_mut().for_each(harden_schema),
                    None => { /* leave */ }
                }
            }

            // Logical combinators
            if let Some(v) = obj.any_of.as_mut() {
                for s in v.iter_mut() { harden_schema(s); }
            }
            if let Some(v) = obj.all_of.as_mut() {
                for s in v.iter_mut() { harden_schema(s); }
            }
            if let Some(v) = obj.one_of.as_mut() {
                for s in v.iter_mut() { harden_schema(s); }
            }
            if let Some(s) = obj.not.as_mut() { harden_schema(s); }
        }
    }
}

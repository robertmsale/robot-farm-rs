use indexmap::IndexMap;
use schemars::schema::{
    ArrayValidation, InstanceType, ObjectValidation, RootSchema, Schema, SchemaObject, SingleOrVec,
};

pub struct VisitContext<'a> {
    /// JSON pointer we're currently at
    pub pointer: String,
    pub root: &'a RootSchema,
}

impl<'a> VisitContext<'a> {
    fn child(&self, seg: &str) -> Self {
        let p = if self.pointer == "/" { format!("/{}", seg) }
        else { format!("{}/{}", self.pointer, seg) };
        Self { pointer: p, root: self.root }
    }
}

pub fn walk_schema<F>(root: &RootSchema, mut f: F)
where
    F: FnMut(&VisitContext, &SchemaObject),
{
    let ctx = VisitContext { pointer: "/".to_string(), root };
    walk_schema_object(&ctx, &root.schema, &mut f);
}

fn walk_schema_object<F>(ctx: &VisitContext, schema: &Schema, f: &mut F)
where
    F: FnMut(&VisitContext, &SchemaObject),
{
    match schema {
        Schema::Bool(_) => { /* rare in schemars output */ }
        Schema::Object(obj) => {
            f(ctx, obj);

            // Dive into sub-parts
            if let Some(objv) = &obj.object {
                walk_object(ctx, obj, objv, f);
            }
            if let Some(arrv) = &obj.array {
                walk_array(ctx, arrv, f);
            }

            // anyOf / allOf / oneOf
            for (i, sub) in obj.any_of.iter().flatten().enumerate() {
                walk_schema_object(&ctx.child(&format!("anyOf/{}", i)), sub, f);
            }
            for (i, sub) in obj.all_of.iter().flatten().enumerate() {
                walk_schema_object(&ctx.child(&format!("allOf/{}", i)), sub, f);
            }
            for (i, sub) in obj.one_of.iter().flatten().enumerate() {
                walk_schema_object(&ctx.child(&format!("oneOf/{}", i)), sub, f);
            }
            if let Some(sub) = &obj.not {
                walk_schema_object(&ctx.child("not"), sub, f);
            }
            // If there are references, schemars inlines most, but handle just in case:
            if let Some(_ref_) = &obj.reference {
                // reference pointers are resolved by API; here we just note it.
            }
        }
    }
}

fn walk_object<F>(ctx: &VisitContext, obj: &SchemaObject, ov: &ObjectValidation, f: &mut F)
where
    F: FnMut(&VisitContext, &SchemaObject),
{
    for (k, v) in &ov.properties {
        walk_schema_object(&ctx.child(&format!("properties/{}", k)), v, f);
    }
    for (k, v) in &ov.pattern_properties {
        walk_schema_object(&ctx.child(&format!("patternProperties/{}", k)), v, f);
    }
    if let Some(ap) = &ov.additional_properties {
        match ap {
            schemars::schema::AdditionalProperties::Schema(s) => {
                walk_schema_object(&ctx.child("additionalProperties"), s, f);
            }
            schemars::schema::AdditionalProperties::Boolean(_) => {}
        }
    }

    if let Some(ty) = &obj.instance_type {
        // nothing special here; handled by validators
        let _ = ty;
    }
}

fn walk_array<F>(ctx: &VisitContext, av: &ArrayValidation, f: &mut F)
where
    F: FnMut(&VisitContext, &SchemaObject),
{
    match &av.items {
        Some(SingleOrVec::Single(schema)) => {
            walk_schema_object(&ctx.child("items"), schema, f);
        }
        Some(SingleOrVec::Vec(schemas)) => {
            for (i, s) in schemas.iter().enumerate() {
                walk_schema_object(&ctx.child(&format!("prefixItems/{}", i)), s, f);
            }
        }
        None => {}
    }
}

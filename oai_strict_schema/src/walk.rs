use schemars::Schema;
use serde_json::{Map, Value};

pub struct VisitContext<'a> {
    /// JSON pointer we're currently at
    pub pointer: String,
    pub root: &'a Schema,
}

impl<'a> VisitContext<'a> {
    fn child(&self, seg: &str) -> Self {
        let p = if self.pointer == "/" {
            format!("/{}", seg)
        } else {
            format!("{}/{}", self.pointer, seg)
        };
        Self {
            pointer: p,
            root: self.root,
        }
    }
}

pub fn walk_schema<F>(root: &Schema, mut f: F)
where
    F: FnMut(&VisitContext, &Map<String, Value>),
{
    let ctx = VisitContext {
        pointer: "/".to_string(),
        root,
    };
    walk_value(&ctx, root.as_value(), &mut f);
}

fn walk_value<F>(ctx: &VisitContext, schema: &Value, f: &mut F)
where
    F: FnMut(&VisitContext, &Map<String, Value>),
{
    match schema {
        Value::Bool(_) => {}
        Value::Object(map) => {
            f(ctx, map);
            walk_object_children(ctx, map, f);
        }
        _ => {}
    }
}

fn walk_object_children<F>(ctx: &VisitContext, obj: &Map<String, Value>, f: &mut F)
where
    F: FnMut(&VisitContext, &Map<String, Value>),
{
    if let Some(Value::Object(props)) = obj.get("properties") {
        for (k, v) in props {
            walk_value(&ctx.child(&format!("properties/{}", k)), v, f);
        }
    }

    if let Some(Value::Object(props)) = obj.get("patternProperties") {
        for (k, v) in props {
            walk_value(&ctx.child(&format!("patternProperties/{}", k)), v, f);
        }
    }

    if let Some(ap) = obj.get("additionalProperties") {
        if !ap.is_boolean() {
            walk_value(&ctx.child("additionalProperties"), ap, f);
        }
    }

    if let Some(items) = obj.get("items") {
        match items {
            Value::Array(arr) => {
                for (i, v) in arr.iter().enumerate() {
                    walk_value(&ctx.child(&format!("items/{}", i)), v, f);
                }
            }
            _ => walk_value(&ctx.child("items"), items, f),
        }
    }

    if let Some(Value::Array(arr)) = obj.get("prefixItems") {
        for (i, v) in arr.iter().enumerate() {
            walk_value(&ctx.child(&format!("prefixItems/{}", i)), v, f);
        }
    }

    for keyword in ["anyOf", "allOf", "oneOf"] {
        if let Some(Value::Array(arr)) = obj.get(keyword) {
            for (i, v) in arr.iter().enumerate() {
                walk_value(&ctx.child(&format!("{}/{}", keyword, i)), v, f);
            }
        }
    }

    for keyword in ["not", "if", "then", "else", "contains", "propertyNames"] {
        if let Some(value) = obj.get(keyword) {
            walk_value(&ctx.child(keyword), value, f);
        }
    }

    if let Some(Value::Object(deps)) = obj.get("dependentSchemas") {
        for (k, v) in deps {
            walk_value(&ctx.child(&format!("dependentSchemas/{}", k)), v, f);
        }
    }

    walk_defs(ctx, obj, "$defs", f);
    walk_defs(ctx, obj, "definitions", f);
}

fn walk_defs<F>(ctx: &VisitContext, obj: &Map<String, Value>, keyword: &str, f: &mut F)
where
    F: FnMut(&VisitContext, &Map<String, Value>),
{
    if let Some(value) = obj.get(keyword) {
        match value {
            Value::Object(map) => {
                for (k, v) in map {
                    walk_value(&ctx.child(&format!("{}/{}", keyword, k)), v, f);
                }
            }
            Value::Array(arr) => {
                for (i, v) in arr.iter().enumerate() {
                    walk_value(&ctx.child(&format!("{}/{}", keyword, i)), v, f);
                }
            }
            _ => {}
        }
    }
}

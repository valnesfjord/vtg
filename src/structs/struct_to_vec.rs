use std::borrow::Cow;

use serde::Serialize;
use serde_json::Value;

pub fn struct_to_vec<'a, T>(s: T) -> Vec<(Cow<'a, str>, Cow<'a, str>)>
where
    T: Serialize,
{
    let value = serde_json::to_value(&s).expect("Error serializing structure to JSON");

    match value {
        Value::Object(map) => {
            let mut vec = Vec::with_capacity(map.len());

            for (k, v) in map {
                let value = match v {
                    Value::String(s) => Cow::Owned(s),
                    Value::Number(n) => Cow::Owned(n.to_string()),
                    Value::Bool(b) => Cow::Owned(b.to_string()),
                    Value::Null => Cow::Borrowed(""),
                    Value::Array(a) => Cow::Owned(serde_json::to_string(&a).unwrap()),
                    Value::Object(o) => Cow::Owned(serde_json::to_string(&o).unwrap()),
                };
                vec.push((Cow::Owned(k), value));
            }
            vec
        }
        _ => panic!("Expected JSON object to convert to Vec"),
    }
}

pub fn param<'a, S1, S2>(s1: S1, s2: S2) -> (Cow<'a, str>, Cow<'a, str>)
where
    S1: Into<Cow<'a, str>>,
    S2: Into<Cow<'a, str>>,
{
    (s1.into(), s2.into())
}

use std::{borrow::Cow, collections::HashMap};

use serde::Serialize;
use serde_json::Value;

pub fn struct_to_vec<'a, T>(s: T) -> Vec<(Cow<'a, str>, Cow<'a, str>)>
where
    T: Serialize,
{
    let value = serde_json::to_value(&s).expect("Error serializing structure to JSON");

    let map: HashMap<String, Value> = match value {
        Value::Object(map) => map.into_iter().collect(),
        _ => panic!("Expected JSON object to convert to HashMap"),
    };

    let string_map: HashMap<Cow<'a, str>, Cow<'a, str>> = map
        .into_iter()
        .map(|(k, v)| (Cow::Owned(k), Cow::Owned(v.to_string())))
        .collect();

    string_map.into_iter().collect()
}

pub fn param<'a, S1, S2>(s1: S1, s2: S2) -> (Cow<'a, str>, Cow<'a, str>)
where
    S1: Into<Cow<'a, str>>,
    S2: Into<Cow<'a, str>>,
{
    (s1.into(), s2.into())
}

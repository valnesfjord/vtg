use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn struct_to_vec<T: Serialize + for<'a> Deserialize<'a>>(
    s: T,
) -> Vec<(&'static str, &'static str)> {
    let map: HashMap<String, Value> =
        serde_json::from_value(serde_json::to_value(s).unwrap()).unwrap();
    map.into_iter()
        .map(|(k, v)| {
            let k = Box::leak(k.into_boxed_str());
            let v = Box::leak(v.to_string().into_boxed_str());
            (k as &str, v as &str)
        })
        .collect()
}

use serde_json::Value;
use std::collections::HashSet;

pub fn flatten_json_to_set(json_obj: &Value) -> HashSet<String> {
    let mut result_set = HashSet::new();
    flatten(json_obj, String::new(), &mut result_set);
    result_set
}
pub fn flatten(json: &Value, prefix: String, result_set: &mut HashSet<String>) {
    match json {
        Value::Object(map) => {
            for (key, value) in map {
                flatten(value, format!("{}{}_", prefix, key), result_set);
            }
        }
        Value::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                flatten(item, format!("{}{}_", prefix, i), result_set);
            }
        }
        _ => {
            result_set.insert(format!("{}{}", prefix, json));
        }
    }
}

pub fn json_to_sets(json_strs: &Vec<&str>) -> Vec<HashSet<String>> {
    json_strs
        .iter()
        .map(|s| serde_json::from_str(s).unwrap())
        .map(|json: Value| flatten_json_to_set(&json))
        .collect()
}
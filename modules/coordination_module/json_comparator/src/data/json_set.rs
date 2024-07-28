use serde_json::Value;
use std::collections::HashSet;

/// Flattens a JSON object into a set of strings.
///
/// # Parameters
/// - `json_obj`: The JSON object to be flattened.
///
/// # Returns
/// - `HashSet<String>`: A set of strings representing the flattened JSON object.
pub fn flatten_json_to_set(json_obj: &Value) -> HashSet<String> {
    let mut result_set = HashSet::new();
    flatten(json_obj, String::new(), &mut result_set);
    result_set
}

/// Helper function to recursively flatten a JSON object.
///
/// # Parameters
/// - `json`: The current JSON value being flattened.
/// - `prefix`: The prefix used for the keys in the flattened representation.
/// - `result_set`: The set to store the flattened strings.
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

/// Converts a vector of JSON strings into a vector of sets of flattened elements.
///
/// # Parameters
/// - `json_strs`: A vector of JSON strings.
///
/// # Returns
/// - `Vec<HashSet<String>>`: A vector of sets, where each set represents a flattened JSON object.
pub fn json_to_sets(json_strs: &Vec<&str>) -> Vec<HashSet<String>> {
    json_strs
        .iter()
        .map(|s| serde_json::from_str(s).unwrap())
        .map(|json: Value| flatten_json_to_set(&json))
        .collect()
}
use colored::*;

#[macro_use]
pub mod data;
pub mod hash;

use hash::minhash_comparison::{calculate_similarities, find_best_similarity};

/// Runs the JSON comparator on a given set of JSON objects.
///
/// # Parameters
/// - `json_objects`: A slice of JSON strings.
///
/// # Returns
/// - `Option<String>`: The JSON string with the highest similarity, if any.
pub fn run_json_comparator(json_objects: &[String]) -> Option<String> {
    let similarity_threshold = 0.72;
    println!("\nSimilarity Threshold: {}%", format!("{:.1}", similarity_threshold * 100.0).blue());

    let num_hash_functions = 100;
    println!("Number of Hash Functions: {}\n", num_hash_functions.to_string().blue());

    let json_strs: Vec<&str> = json_objects.iter().map(|s| s.as_str()).collect();

    calculate_similarities(json_strs.clone(), similarity_threshold, num_hash_functions);
    if let Some(best_json) = find_best_similarity(json_strs, similarity_threshold, num_hash_functions) {
        println!("\nThe JSON with the best similarity is: {}", best_json.green().bold());
        return Some(best_json);
    } else {
        println!("\nNo JSON objects met the similarity threshold.");
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_run_json_comparator_with_similar_jsons() {
        let json1 = json!({"key": "value1"}).to_string();
        let json2 = json!({"key": "value1"}).to_string();
        let json_objects = vec![json1.clone(), json2.clone()];

        let result = run_json_comparator(&json_objects);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), json1);
    }

    #[test]
    fn test_run_json_comparator_with_dissimilar_jsons() {
        let json1 = json!({"key": "value1"}).to_string();
        let json2 = json!({"key": "value2"}).to_string();
        let json_objects = vec![json1, json2];

        let result = run_json_comparator(&json_objects);

        assert!(result.is_none());
    }

    #[test]
    fn test_run_json_comparator_with_empty_list() {
        let json_objects: Vec<String> = vec![];

        let result = run_json_comparator(&json_objects);

        assert!(result.is_none());
    }
}
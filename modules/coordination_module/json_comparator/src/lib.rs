use colored::*;

#[macro_use]
mod data;
mod hash;

use hash::minhash_comparison::{calculate_similarities, find_best_similarity};

pub fn run_json_comparator() -> Option<String> {
    let similarity_threshold = 0.72;
    println!("\nSimilarity Threshold: {}%", format!("{:.1}", similarity_threshold * 100.0).blue());

    let num_hash_functions = 100;
    println!("Number of Hash Functions: {}\n", num_hash_functions.to_string().blue());

    // Example JSON objects
    let json_strs = vec![
        r#"{"game": "World of Warcraft", "character": "Thrall", "ability": "Earthquake"}"#,
        r#"{"game": "World of Warcraft", "character": "Thrall", "ability": "Earthquake", "level": "99"}"#,
        r#"{"game": "World of Warcraft", "character": "Thrall", "ability": "Earthquake", "guild": "Horde"}"#,
        r#"{"game": "World of Warcraft", "character": "Thrall", "ability": "Earthquake", "race": "Orc"}"#,
        r#"{"game": "World of Warcraft", "character": "Jaina", "ability": "Frostbolt"}"#,
        r#"{"game": "World of Warcraft", "character": "Thrall", "ability": "Earthquake"}"#,  
    ];

    calculate_similarities(json_strs.clone(), similarity_threshold, num_hash_functions);
    if let Some(best_json) = find_best_similarity(json_strs, similarity_threshold, num_hash_functions) {
        println!("\nThe JSON with the best similarity is: {}", best_json.green().bold());
        return Some(best_json);
    } else {
        println!("\nNo JSON objects met the similarity threshold.");
        return None;
    }
}

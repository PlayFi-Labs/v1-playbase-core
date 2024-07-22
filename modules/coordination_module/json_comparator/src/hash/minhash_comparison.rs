use probabilistic_collections::similarity::{MinHash, ShingleIterator};
use colored::*;

use crate::data::json_set::json_to_sets;

pub fn calculate_similarities(json_strs: Vec<&str>, similarity_threshold: f64, num_hash_functions: usize) {
    let json_sets = json_to_sets(&json_strs);
    
    let min_hash = MinHash::new(num_hash_functions);

    let min_hashes: Vec<_> = json_sets
        .iter()
        .map(|set| {
            let shingles = ShingleIterator::new(1, set.iter().map(|s| s.as_str()).collect());
            min_hash.get_min_hashes(shingles)
        })
        .collect();

    for i in 0..min_hashes.len() {
        for j in i + 1..min_hashes.len() {
            let similarity = min_hash.get_similarity_from_hashes(&min_hashes[i], &min_hashes[j]);
            if similarity >= similarity_threshold {
                println!(
                    "Similarity between JSON {} and JSON {}: {:.1}% - {}",
                    i + 1,
                    j + 1,
                    similarity * 100.0,
                    "PASS".green()
                );
            } else {
                println!(
                    "Similarity between JSON {} and JSON {}: {:.1}% - {}",
                    i + 1,
                    j + 1,
                    similarity * 100.0,
                    "NOT PASS".red()
                );
            }
        }
    }
}

pub fn find_best_similarity(json_strs: Vec<&str>, similarity_threshold: f64, num_hash_functions: usize) -> Option<String> {
    let json_sets = json_to_sets(&json_strs);

    let min_hash = MinHash::new(num_hash_functions);

    let min_hashes: Vec<_> = json_sets
        .iter()
        .map(|set| {
            let shingles = ShingleIterator::new(1, set.iter().map(|s| s.as_str()).collect());
            min_hash.get_min_hashes(shingles)
        })
        .collect();

    let mut best_similarity = 0.0;
    let mut best_json = None;

    for i in 0..min_hashes.len() {
        for j in (i + 1)..min_hashes.len() {
            let similarity = min_hash.get_similarity_from_hashes(&min_hashes[i], &min_hashes[j]);
            if similarity >= similarity_threshold && similarity > best_similarity {
                best_similarity = similarity;
                best_json = Some(json_strs[j].to_string());
            }
        }
    }

    best_json
}
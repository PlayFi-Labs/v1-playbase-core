
use json::load_json_objects;
use json_comparator::run_json_comparator;
use fingerprint::{run_fingerprint, Fingerprint};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Load JSON objects from the specified directory
    let json_objects = load_json_objects("modules/coordination_module/json/src/json_objects").await;

    // Run the JSON comparator and get the JSON object with the highest similarity
    let best_json_str = match run_json_comparator(&json_objects) {
        Some(json) => json,
        None => {
            println!("No JSON objects met the similarity threshold.");
            return Ok(());
        }
    };

    // Parse the resulting JSON object to create a Fingerprint
    let best_json: Value = serde_json::from_str(&best_json_str)?;
    let fingerprint = Fingerprint {
        gamer: best_json["character"].as_str().unwrap_or_default().to_string(),
        strikes: 0,
        place: best_json["place"].as_str().unwrap_or_default().to_string(),
        weapon: best_json["ability"].as_str().unwrap_or_default().to_string(),
        place2: best_json["place2"].as_str().unwrap_or_default().to_string(),
    };

    // Run the fingerprint process with the resulting Fingerprint object
    run_fingerprint(fingerprint).await?;

    Ok(())
}
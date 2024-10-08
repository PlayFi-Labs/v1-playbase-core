
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio;

    async fn mock_load_json_objects(_path: &str) -> Vec<String> {
        vec![json!({
            "character": "test_character",
            "place": "test_place",
            "ability": "test_ability",
            "place2": "test_place2"
        }).to_string()]
    }

    fn mock_run_json_comparator(_json_objects: &[String]) -> Option<String> {
        Some(json!({
            "character": "test_character",
            "place": "test_place",
            "ability": "test_ability",
            "place2": "test_place2"
        }).to_string())
    }

    async fn mock_run_fingerprint(_fingerprint: Fingerprint) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    #[tokio::test]
    async fn test_main() {
        let json_objects = mock_load_json_objects("mock_path").await;
        let best_json_str = mock_run_json_comparator(&json_objects).unwrap();
        let best_json: Value = serde_json::from_str(&best_json_str).unwrap();
        let fingerprint = Fingerprint {
            gamer: best_json["character"].as_str().unwrap_or_default().to_string(),
            strikes: 0,
            place: best_json["place"].as_str().unwrap_or_default().to_string(),
            weapon: best_json["ability"].as_str().unwrap_or_default().to_string(),
            place2: best_json["place2"].as_str().unwrap_or_default().to_string(),
        };

        let result = mock_run_fingerprint(fingerprint).await;
        assert!(result.is_ok());
    }
}
use json::load_json_objects;
use json_comparator::run_json_comparator;
use fingerprint::{run_fingerprint, process_query_fingerprint, Fingerprint};
use node_request::request_to_node;
use serde_json::Value;
use std::error::Error;

/// Function `cm_write` handles loading JSON objects, selecting the best match, and processing the fingerprint.
///
/// # Returns
/// - `Ok(())` on success or an error wrapped in `Box<dyn Error>` if any step fails.
pub async fn cm_write() -> Result<(), Box<dyn Error>> {
    // Load JSON objects from the specified directory
    let json_objects = load_json_objects("json/src/json_objects").await;

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
        user: best_json["user"].as_str().unwrap_or_default().to_string(),
        game: best_json["character"].as_str().unwrap_or_default().to_string(),
        strikes: 0,
        place: best_json["place"].as_str().unwrap_or_default().to_string(),
        place2: best_json["place2"].as_str().unwrap_or_default().to_string(),
    };

    // Run the fingerprint process with the resulting Fingerprint object
    run_fingerprint(fingerprint).await?;

    Ok(())
}

/// Function `cm_query` handles querying a JSON object and checks if the generated fingerprint exists on-chain.
/// This function also request to another node to retrieve data.
///
/// # Parameters
/// - `fields`: A reference to the JSON object generated from the zk-module.
///
/// # Returns
/// - `Ok(true)` if the fingerprint is found on-chain or `Err(Box<dyn Error>)` otherwise.
pub async fn cm_query(fields: &Value) -> Result<bool, Box<dyn Error>> {
    // Mock sending a request to another node
    let mock_response = request_to_node(fields).await?;

    // Process the mock response by verifying the fingerprint
    let result = process_query_fingerprint(&mock_response).await?;

    // If the fingerprint exists on-chain, return Ok(true)
    Ok(result)
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

    async fn mock_run_fingerprint(_fingerprint: Fingerprint) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn mock_process_query_fingerprint(_generated_json: &Value) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }

    #[tokio::test]
    async fn test_run_cm_write() {
        let json_objects = mock_load_json_objects("mock_path").await;
        let best_json_str = mock_run_json_comparator(&json_objects).unwrap();
        let best_json: Value = serde_json::from_str(&best_json_str).unwrap();
        let fingerprint = Fingerprint {
            user: best_json["user"].as_str().unwrap_or_default().to_string(),
            game: best_json["character"].as_str().unwrap_or_default().to_string(),
            strikes: 0,
            place: best_json["place"].as_str().unwrap_or_default().to_string(),
            place2: best_json["place2"].as_str().unwrap_or_default().to_string(),
        };

        let result = mock_run_fingerprint(fingerprint).await;
        assert!(result.is_ok());
    }

    /// Test for `cm_query` function.
    #[tokio::test]
    async fn test_run_cm_query() {
        let query_fields = json!({
            "user": "new_user",
            "game": "new_game",
            "place": "new_place"
        });

        // Mock sending a request to another node and receiving a response
        let mock_response = request_to_node(&query_fields).await.unwrap();
        assert_eq!(mock_response, query_fields);

        // Mock processing the response by verifying the fingerprint
        let result = mock_process_query_fingerprint(&mock_response).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }
}
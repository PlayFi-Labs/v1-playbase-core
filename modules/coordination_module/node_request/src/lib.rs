use serde_json::Value;
use std::error::Error;

#[macro_use]
pub mod json_request;

use json_request::node_request::mock_request_to_node;

/// Request to another node, returning the fields passed as response.
///
/// # Parameters
/// - `fields`: A `serde_json::Value` containing the query fields.
///
/// # Returns
/// - A `serde_json::Value` containing the mock response from the node.
pub async fn request_to_node(fields: &Value) -> Result<Value, Box<dyn Error>> {

    //TODO: Implement real request to node
    // Call the mock function and return the result
    let result = mock_request_to_node(fields).await;
    println!("Sending request to node with fields: {:?}", fields);

    Ok(result?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_mock_request_to_node() {
        // Example JSON data to send in the request
        let fields = json!({
            "gamer": "test_user",
            "place": "test_place"
        });

        // Call the mock function and check the response
        let result = mock_request_to_node(&fields).await;

        // Ensure that the result is okay and matches the input fields
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response, fields);
    }
}

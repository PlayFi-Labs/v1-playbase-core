use serde_json::Value;
use std::error::Error;

//TODO: Implement real request to node
/// Mocks a request to another node, returning the fields passed as the mock response.
///
/// # Parameters
/// - `fields`: A `serde_json::Value` containing the query fields.
///
/// # Returns
/// - A `serde_json::Value` containing the mock response from the node.
pub async fn mock_request_to_node(fields: &Value) -> Result<Value, Box<dyn Error>> {
    // Simulate sending the fields to another node
    println!("Sending request to node with fields: {:?}", fields);

    // For simplicity, return the same fields as the mock response
    Ok(fields.clone())
}
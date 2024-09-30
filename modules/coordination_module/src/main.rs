
use json::load_json_objects;
use json_comparator::run_json_comparator;
use fingerprint::{run_fingerprint, Fingerprint};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    coordination_module::cm_write().await
}
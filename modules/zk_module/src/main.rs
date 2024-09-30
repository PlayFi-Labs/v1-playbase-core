use dotenv::dotenv;
use std::env;
use std::fs;
use anyhow::{Context, Result};
use serde_json::Value;
use zk_module::{JsonRecords, MerkleTreeParameters};
use circom::call_js_proof;
use verifier::check::check_proof::run_proof_verification;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // Fetch environment variables
    let json_db_name = env::var("JSON_MONGO_DB").context("JSON_MONGO_DB must be set")?;
    let json_collection_name = env::var("JSON_MONGO_COL").context("JSON_MONGO_COL must be set")?;
    let mongo_uri = env::var("MONGO_URL").context("MONGO_URL must be set")?;

    // Read the JSON data from json1.json
    let json_data_path = "./utils/src/json_object/json1.json";
    let json_data = JsonRecords::from_file(json_data_path).context("Failed to read JSON data")?;

    // Call JSON write operations
    json_data.write_to_db(&mongo_uri, &json_db_name, &json_collection_name).await.context("Failed to write JSON data to DB")?;

    // Query the same data based on a field, for example, "user"
    JsonRecords::query_db(&mongo_uri, &json_db_name, &json_collection_name, "user", &json_data.user).await.context("Failed to query DB")?;

    // Merkle Tree Parameters
    let merkle_params = MerkleTreeParameters {
        level: 5,
        g: "generator_g".to_string(),
        g_tilde: "generator_g_tilde".to_string(),
    };

    let merkle_db_name = env::var("MERKLE_MONGO_DB").context("MERKLE_MONGO_DB must be set")?;
    let merkle_collection_name = json_collection_name.clone() + "-merkle";

    // Call Merkle operations
    merkle_params.write_to_db(&mongo_uri, &merkle_db_name, &merkle_collection_name).await.context("Failed to write Merkle tree parameters to DB")?;

    // Call the JavaScript proof generation script and capture the result
    let proof_data: Value = call_js_proof("./circom/utils/preprocess_proof.js").context("Failed to generate proof")?;
    
    // Print proof data
    println!("Generated proof: {:?}", proof_data);

    let json_path = "./circom/utils/proof.json";
    let json_data = fs::read_to_string(json_path).context("Failed to read JSON data")?;
    let json_mock: Value = serde_json::from_str(&json_data).context("Failed to parse JSON data")?;

    // Print proof data
    println!("Generated proof: {:?}", json_mock);


    // Verify the proof
    run_proof_verification(json_mock).await.context("Proof verification failed")?;

    Ok(())
}
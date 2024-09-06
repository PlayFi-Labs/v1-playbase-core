use dotenv::dotenv;
use std::env;
use anyhow::Result;
use zk_module::{JsonRecords, MerkleTreeParameters, verify_proof};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // Fetch environment variables
    let json_db_name = env::var("JSON_MONGO_DB").expect("DB_NAME must be set");
    let json_collection_name = env::var("JSON_MONGO_COL").expect("COLLECTION_NAME must be set");
    let mongo_uri = env::var("MONGO_URL").expect("MONGO_URI must be set");

    // Read the JSON data from json1.json
    let json_data_path = "./utils/src/json_object/json1.json";
    let json_data = JsonRecords::from_file(json_data_path)?;

    // Call JSON write operations
    json_data.write_to_db(&mongo_uri, &json_db_name, &json_collection_name).await?;

    // Query the same data based on a field, for example, "user"
    JsonRecords::query_db(&mongo_uri, &json_db_name, &json_collection_name, "user", &json_data.user).await?;

    // Merkle Tree Parameters
    let merkle_params = MerkleTreeParameters {
        level: 5,
        g: "generator_g".to_string(),
        g_tilde: "generator_g_tilde".to_string(),
    };

    let merkle_db_name = env::var("MERKLE_MONGO_DB").expect("DB_NAME must be set");
    let merkle_collection_name = json_collection_name.to_string() + "-merkle";

    // Call Merkle operations
    merkle_params.write_to_db(&mongo_uri, &merkle_db_name, &merkle_collection_name).await?;

    // Verify the proof
    verify_proof("./circom/utils/preprocess_input.js")?;

    Ok(())
}

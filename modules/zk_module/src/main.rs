use anyhow::{Result, anyhow};
use circom::call_js_proof_and_verify;
use mongodb::bson::doc;
use dotenv::dotenv;
use std::env;
use utils::{store, restore};
use utils::json_format::JsonRecords;
use utils::mongo_connection::mongo_connection::MongoDB;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct MerkleTreeParameters {
    level: u32,
    g: String,
    g_tilde: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let json_db_name = env::var("JSON_MONGO_DB").expect("DB_NAME must be set");
    let json_collection_name = env::var("JSON_MONGO_COL").expect("COLLECTION_NAME must be set");
    let mongo_uri = env::var("MONGO_URL").expect("MONGO_URI must be set");

    let mongo_connection = MongoDB::new(&mongo_uri, &json_db_name, &json_collection_name).await?;
    let json_database = mongo_connection.get_database();
    let json_collection = json_database.collection::<JsonRecords>(&json_collection_name);

    let json_data = JsonRecords {
        user: "user1".to_string(),
        game: "Game4".to_string(),
        character: "Character2".to_string(),
        ability: "Ability1".to_string(),
        place: "Place123".to_string(),
        place2: "Place2".to_string(),
        aimodel: 1234567,
        aiversion: 1,
        ainode: 10,
        uploader: Default::default(),
        timestamp: "2024-08-14T12:35:56Z".to_string(),
        source: 1,
        sourcetype: 2,
        hash_inputdata: [0u8; 32],
    };

    println!("Attempting to insert JSON Parameters: {:?}", json_data);
    match json_collection.insert_one(&json_data).await {
        Ok(insert_result) => {
            println!("Successfully inserted document with id: {:?}", insert_result.inserted_id);
        },
        Err(e) => {
            eprintln!("Failed to insert document: {:?}", e);
        }
    }

    let json_path = "data/json_record.json";
    store(&json_data, json_path)?;

    let restored_json_data: JsonRecords = restore(json_path)?;
    println!("Restored JSON Data: {:?}", restored_json_data);

    let json_proof_verified = call_js_proof_and_verify("./circom/utils/preprocess_input.js")?;
    if !json_proof_verified {
        return Err(anyhow!("JSON proof verification failed"));
    }else {
        println!("JSON proof verified successfully");
    }

    let merkle_params = MerkleTreeParameters {
        level: 5,
        g: "generator_g".to_string(),
        g_tilde: "generator_g_tilde".to_string(),
    };

    let merkle_db_name = env::var("MERKLE_MONGO_DB").expect("DB_NAME must be set");
    let merkle_collection_name = json_collection_name.to_string() + "-merkle";
    let merkle_mongo_connection = MongoDB::new(&mongo_uri, &merkle_db_name, &merkle_collection_name).await?;
    let merkle_database = merkle_mongo_connection.get_database();
    let merkle_collection = merkle_database.collection::<MerkleTreeParameters>(&merkle_collection_name);

    println!("Attempting to insert Merkle Tree Parameters: {:?}", merkle_params);
    match merkle_collection.insert_one(&merkle_params).await {
        Ok(insert_result) => {
            println!("Successfully inserted document with id: {:?}", insert_result.inserted_id);
        },
        Err(e) => {
            eprintln!("Failed to insert document: {:?}", e);
        }
    }

    let merkle_path = format!("data/{}.json", merkle_collection_name);
    store(&merkle_params, &merkle_path)?;

    let merkle_proof_verified = call_js_proof_and_verify("./circom/utils/preprocess_input.js")?;
    if !merkle_proof_verified {
        return Err(anyhow!("Merkle tree proof verification failed"));
    }else {
        println!("Merkle tree proof verified successfully");
    }

    println!("Both JSON and Merkle Tree proofs verified successfully");

    Ok(())
}

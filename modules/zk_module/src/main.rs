use serde::{Serialize, Deserialize};
use anyhow::Result;
use mongodb::bson::doc;
use dotenv::dotenv;
use std::env;
use lasso_jolt::{store, restore};
use lasso_jolt::json_format::JsonRecords;
use lasso_jolt::mongo_connection::mongo_connection::MongoDB;

#[derive(Serialize, Deserialize, Debug)]
struct MerkleTreeParameters {
    level: u32,
    g: String,
    g_tilde: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let db_name = env::var("MONGO_DB").expect("DB_NAME must be set");
    let collection_name: String = env::var("MONGO_COL").expect("COLLECTION_NAME must be set");
    let mongo_uri = env::var("MONGO_URL").expect("MONGO_URI must be set");

    let mongo_connection = MongoDB::new(&mongo_uri, &db_name, &collection_name).await?;
    let database = mongo_connection.get_database();
    let collection = database.collection::<JsonRecords>(&collection_name);

    // Example 1: Using the `store` and `restore` functions with `JsonRecords`
    let json_data = JsonRecords {
        game: "Game1".to_string(),
        character: "Character2".to_string(),
        ability: "Ability1".to_string(),
        place: "Place1".to_string(),
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

    let json_path = "data/json_record.json";
    store(&json_data, json_path)?;

    let restored_json_data: JsonRecords = restore(json_path)?;
    println!("Restored JSON Data: {:?}", restored_json_data);

    // Insert JSON data into MongoDB
    collection.insert_one(json_data).await?;

    // Query the database
    let query_result = collection.find_one(doc! {"character": "Character1"}).await?;
    if let Some(record) = query_result {
        println!("Queried Record: {:?}", record);

        // Example 2: Using the `store` and `restore` functions with `MerkleTreeParameters`
        let _merkle_params = MerkleTreeParameters {
            level: 3,
            g: "generator_g".to_string(),
            g_tilde: "generator_g_tilde".to_string(),
        };
    }

    Ok(())
}
use serde::{Serialize, Deserialize};
use anyhow::Result;
use lasso_jolt::{store, restore};
use lasso_jolt::json_format::JsonRecords;

#[derive(Serialize, Deserialize, Debug)]
struct MerkleTreeParameters {
    level: u32,
    g: String,
    g_tilde: String,
}

fn main() -> Result<()> {
    // Example 1: Using the `store` and `restore` functions with `JsonRecords`
    let json_data = JsonRecords {
        game: "Game1".to_string(),
        character: "Character1".to_string(),
        ability: "Ability1".to_string(),
        place: "Place1".to_string(),
        place2: "Place2".to_string(),
        aimodel: 123456,
        aiversion: 1,
        ainode: 10,
        uploader: Default::default(),
        timestamp: "2024-08-14T12:34:56Z".to_string(),
        source: 1,
        sourcetype: 2,
        hash_inputdata: [0u8; 32],
    };

    let json_path = "data/json_record.json";
    store(&json_data, json_path)?;

    let restored_json_data: JsonRecords = restore(json_path)?;
    println!("Restored JSON Data: {:?}", restored_json_data);

    // Example 2: Using the `store` and `restore` functions with `MerkleTreeParameters`
    let merkle_params = MerkleTreeParameters {
        level: 3,
        g: "generator_g".to_string(),
        g_tilde: "generator_g_tilde".to_string(),
    };

    let merkle_path = "data/merkle_params.json";
    store(&merkle_params, merkle_path)?;

    let restored_merkle_params: MerkleTreeParameters = restore(merkle_path)?;
    println!("Restored Merkle Tree Parameters: {:?}", restored_merkle_params);

    Ok(())
}
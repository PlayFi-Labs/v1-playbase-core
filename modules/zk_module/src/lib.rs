use anyhow::{Result, anyhow};
use mongodb::bson::doc;
use utils::{store, restore};
use utils::mongo_connection::mongo_connection::MongoDB;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufReader;
use serde_json::from_reader;
use circom::call_js_proof_and_verify;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRecords {
    pub user: String,
    pub game: String,
    pub character: String,
    pub ability: String,
    pub place: String,
    pub place2: String,
    pub aimodel: u64,
    pub aiversion: u64,
    pub ainode: u64,
    pub uploader: String,
    pub timestamp: String,
    pub source: u8,
    pub sourcetype: u8,
    pub hash_inputdata: [u8; 32],
}

impl JsonRecords {
    // Método para leer JSON desde un archivo
    pub fn from_file(file_path: &str) -> Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let json_data: JsonRecords = from_reader(reader)?;
        Ok(json_data)
    }

    // Método para escribir los datos JSON en MongoDB
    pub async fn write_to_db(&self, mongo_uri: &str, db_name: &str, collection_name: &str) -> Result<()> {
        let mongo_connection = MongoDB::new(mongo_uri, db_name, collection_name).await?;
        let collection = mongo_connection.get_database().collection::<JsonRecords>(collection_name);

        match collection.insert_one(self).await {
            Ok(insert_result) => {
                println!("Successfully inserted document with id: {:?}", insert_result.inserted_id);
            },
            Err(e) => {
                eprintln!("Failed to insert document: {:?}", e);
            }
        }

        // Guardar y restaurar para verificar la integridad
        let json_path = "data/json_record.json";
        store(self, json_path)?;
        let restored_data: JsonRecords = restore(json_path)?;
        println!("Restored JSON Data: {:?}", restored_data);

        Ok(())
    }

    // Método para consultar datos en MongoDB
    pub async fn query_db(mongo_uri: &str, db_name: &str, collection_name: &str, field: &str, value: &str) -> Result<()> {
        let mongo_connection = MongoDB::new(mongo_uri, db_name, collection_name).await?;
        let collection = mongo_connection.get_database().collection::<JsonRecords>(collection_name);

        let query = doc! { field: value };
        let result = collection.find_one(query).await?;

        match result {
            Some(document) => {
                println!("Document found: {:?}", document);
            },
            None => {
                println!("No document matches the query.");
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MerkleTreeParameters {
    pub level: u32,
    pub g: String,
    pub g_tilde: String,
}

impl MerkleTreeParameters {
    // Método para escribir los parámetros del árbol de Merkle en MongoDB
    pub async fn write_to_db(&self, mongo_uri: &str, db_name: &str, collection_name: &str) -> Result<()> {
        let mongo_connection = MongoDB::new(mongo_uri, db_name, collection_name).await?;
        let collection = mongo_connection.get_database().collection::<MerkleTreeParameters>(collection_name);

        match collection.insert_one(self).await {
            Ok(insert_result) => {
                println!("Successfully inserted Merkle Tree document with id: {:?}", insert_result.inserted_id);
            },
            Err(e) => {
                eprintln!("Failed to insert Merkle Tree document: {:?}", e);
            }
        }

        let merkle_path = format!("data/{}.json", collection_name);
        store(self, &merkle_path)?;

        Ok(())
    }
}

// Función para verificar la prueba (proof)
pub fn verify_proof(js_script_path: &str) -> Result<()> {
    let proof_verified = call_js_proof_and_verify(js_script_path)?;
    if !proof_verified {
        return Err(anyhow!("Proof verification failed"));
    }
    println!("Proof verified successfully");
    Ok(())
}

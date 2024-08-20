use futures_util::stream::TryStreamExt;
use mongodb::{Client, Collection, Database, bson::{self, Document}};
use std::error::Error;
use std::borrow::Borrow;
use crate::merkle::MerkleTree;
use crate::json_format::{JsonRecords, u32_to_array, array_to_u32};

pub struct MongoDB {
    #[allow(dead_code)]
    client: Client,
    database: Database,
    collection: Collection<Document>,
}

use anyhow::Result;

impl MongoDB {
    pub async fn new(uri: &str, db_name: &str, coll_name: &str) -> Result<Self> {
        let client = Client::with_uri_str(uri).await?;
        let database = client.database(db_name);
        let collection = database.collection(coll_name);
        Ok(Self { client, database, collection })
    }

    pub async fn insert(&self, doc: Document) -> Result<()> {
        self.collection.insert_one(doc).await?;
        Ok(())
    }

    pub async fn query_json(&self, query: Document) -> Result<Vec<Document>> {
        let mut cursor = self.collection.find(query).await?;
        let mut result = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            result.push(doc);
        }
        Ok(result)
    }

    pub fn get_database(&self) -> &Database {
        &self.database
    }

    pub async fn drop_database(&self) -> Result<()> {
        self.database.drop().await?;
        Ok(())
    }
}

impl Borrow<Document> for JsonRecords {
    fn borrow(&self) -> &Document {
        // Implement the conversion from JsonRecords to Document
        // This is a placeholder implementation
        unimplemented!()
    }
}

pub async fn insert_to_database(database: &MongoDB, hash: u32, user_record: &JsonRecords, merkle: &mut MerkleTree) -> Result<(), Box<dyn Error + Send + Sync>> {
    let doc = bson::to_document(&JsonRecords { hash_inputdata: u32_to_array(hash), ..user_record.clone() })?;
    database.insert(doc).await?;
    merkle.insert(array_to_u32(user_record.hash_inputdata.to_vec()).into());
    Ok(())
}
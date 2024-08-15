use futures_util::stream::TryStreamExt;
use mongodb::{Client, Collection, Database, bson::Document};
use std::error::Error;
use std::borrow::Borrow;
use crate::merkle::MerkleTree;
use crate::json_format::{JsonRecords, u32_to_array, array_to_u32};

pub struct MongoDB {
    pub collection_doc: Collection<Document>,
    pub database: Database,
}

impl MongoDB {
    pub async fn new(uri: &str, db_name: &str, coll_name: &str) -> Self {
        let database = Client::with_uri_str(uri).await.unwrap().database(db_name);
        let collection_doc = database.collection(coll_name);
        Self {
            collection_doc,
            database,
        }
    }

    pub fn drop(&self) -> Result<(), Box<dyn Error>> {
        let _ = self.database.drop();  // Ignore the result of drop
        Ok(())
    }

    pub async fn query_json(&self, query: Document) -> Result<Vec<Document>, Box<dyn Error>> {
        let mut cursor = self.collection_doc.find(query).await?;
        let mut result = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            result.push(doc);
        }
        Ok(result)
    }
}

impl Borrow<Document> for JsonRecords {
    fn borrow(&self) -> &Document {
        // Implement the conversion from JsonRecords to Document
        // This is a placeholder implementation
        unimplemented!()
    }
}

pub fn insert_to_database(database: &MongoDB, merkle: &mut MerkleTree, hash: u32, user_record: &JsonRecords) {
    let _ = database.collection_doc.insert_one(JsonRecords { hash_inputdata: u32_to_array(hash), ..user_record.clone() });
    merkle.insert(array_to_u32(user_record.hash_inputdata.to_vec()).into());
}
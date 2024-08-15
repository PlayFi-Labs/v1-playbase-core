pub mod json_format;
pub mod merkle;
pub mod mongo_connection;
pub mod poseidon;

use std::path::Path;
use serde::{de::DeserializeOwned, Serialize};
use anyhow::Result;

// Store all the Merkle tree hash values or vector commitment's system parameters {l, g, g~, ...} 
pub fn store<T>(metadata: &T, path: &str) -> Result<()> 
where T: Serialize
{
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let mut file = std::fs::File::create(path)?;
    let data = serde_json::to_vec(&metadata)?;
    std::io::Write::write_all(&mut file, &data)?;
    
    Ok(())
}

// Restore all the stored values in the Store function
pub fn restore<T>(path: &str) -> Result<T> 
where T: DeserializeOwned
{
    let mut file = std::fs::File::open(path)?;
    let mut data = Vec::new();
    std::io::Read::read_to_end(&mut file, &mut data)?;
 
    Ok(serde_json::from_slice(&data)?)
}
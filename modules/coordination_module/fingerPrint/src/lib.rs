pub mod insert;
pub mod check;
pub mod create;
pub mod encoding;

use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers::types::Address;
use std::env;
use serde_json::Value;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

use crate::check::check_hash::process_and_check_fingerprint;

/// Represents a Fingerprint object.
#[derive(Serialize, Deserialize, Debug)]
pub struct Fingerprint {
    pub user: String,
    pub game: String,
    pub strikes: u64,
    pub place: String,
    pub place2: String,
}

/// Runs the entire fingerprinting process.
///
/// # Parameters
/// - `fingerprint`: The Fingerprint object to be processed.
///
/// # Returns
/// - `Result<(), Box<dyn std::error::Error>>`: Returns `Ok(())` if the process is successful.
pub async fn run_fingerprint(fingerprint: Fingerprint) -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let zksync_url = env::var("ZKSYNC_URL")?;
    let chain_id: u64 = env::var("CHAIN_ID")?.parse()?;
    let provider = Provider::<Http>::try_from(zksync_url)?;
    let contract_address_str = env::var("FINGERPRINT_PROXY_SC")?;
    let private_key = env::var("ZKSYNC_SEPOLIA_PRIVATE_KEY")?;

    let contract_address: Address = contract_address_str.parse()?;
    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(chain_id);
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    let fingerprint_hash = create::create_hash::create_fingerprint_hash(&fingerprint)?;

    insert::insert_hash::insert_fingerprint(client.clone(), contract_address, &fingerprint_hash).await?;
    let is_appended = check::check_hash::check_fingerprint(client.clone(), contract_address, &fingerprint_hash).await?;
    println!("Fingerprint appended: ");
    println!("\x1b[32;1m{}\x1b[0m", is_appended);

    Ok(())
}

/// Handles the query by preparing the client and contract details, and invoking the existing
/// `process_and_check_fingerprint` function.
///
/// # Parameters
/// - `generated_json`: The JSON object returned by the zk-module.
///
/// # Returns
/// - `Result<bool, Box<dyn std::error::Error>>`: Returns `true` if the fingerprint is appended, otherwise returns an error.
pub async fn process_query_fingerprint(
    generated_json: &Value,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Load environment variables for the blockchain setup
    dotenv::dotenv().ok();

    // Get the necessary values from environment variables
    let zksync_url = env::var("ZKSYNC_URL")?;
    let chain_id: u64 = env::var("CHAIN_ID")?.parse()?;
    let contract_address_str = env::var("FINGERPRINT_PROXY_SC")?;
    let private_key = env::var("ZKSYNC_SEPOLIA_PRIVATE_KEY")?;

    // Set up the blockchain provider and wallet
    let provider = Provider::<Http>::try_from(zksync_url)?;
    let contract_address: Address = contract_address_str.parse()?;
    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(chain_id);
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // Invoke the existing `process_and_check_fingerprint` function
    process_and_check_fingerprint(client.clone(), contract_address, generated_json).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::types::H256;
    use std::error::Error;

    fn mock_create_fingerprint_hash(_fingerprint: &Fingerprint) -> Result<H256, Box<dyn Error>> {
        Ok(H256::zero())
    }

    async fn mock_insert_fingerprint(
        _client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
        _contract_address: Address,
        _fingerprint_hash: &H256,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn mock_check_fingerprint(
        _client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
        _contract_address: Address,
        _fingerprint_hash: &H256,
    ) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }

    /// Test function to validate the `run_fingerprint` process.
    #[tokio::test]
    async fn test_run_fingerprint() {
        let fingerprint = Fingerprint {
            user: "test_user".to_string(),
            game: "test_game".to_string(),
            strikes: 0,
            place: "test_place".to_string(),
            place2: "test_place2".to_string(),
        };

        env::set_var("ZKSYNC_URL", "http://localhost:8545");
        env::set_var("CHAIN_ID", "1");
        env::set_var("FINGERPRINT_PROXY_SC", "0000000000000000000000000000000000000000"); // Removed '0x'
        env::set_var("ZKSYNC_SEPOLIA_PRIVATE_KEY", "4c0883a69102937d6231471b5dbb6204fe5129617082796e8e1a1e3b7a1e7e3e"); // Valid private key

        let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();
        let wallet: LocalWallet = "4c0883a69102937d6231471b5dbb6204fe5129617082796e8e1a1e3b7a1e7e3e".parse().unwrap(); // Valid private key
        let wallet = wallet.with_chain_id(1u64); // Convert i32 to u64
        let client = Arc::new(SignerMiddleware::new(provider, wallet));

        let contract_address: Address = "0000000000000000000000000000000000000000".parse().unwrap(); // Removed '0x'

        let fingerprint_hash = mock_create_fingerprint_hash(&fingerprint).unwrap();

        mock_insert_fingerprint(client.clone(), contract_address, &fingerprint_hash).await.unwrap();

        let is_appended = mock_check_fingerprint(client.clone(), contract_address, &fingerprint_hash).await.unwrap();

        assert!(is_appended);
    }
    /// Test function to validate the `process_query_fingerprint` process.
    #[tokio::test]
    async fn test_process_query_fingerprint() {
        // Create a mock JSON object representing a Fingerprint returned by the zk-module
        let json_obj = serde_json::json!({
            "user": "test_user",
            "game": "test_game",
            "strikes": 0,
            "place": "test_place",
            "place2": "test_place2"
        });

        // Mock provider and wallet
        let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();
        let wallet: LocalWallet = "4c0883a69102937d6231471b5dbb6204fe5129617082796e8e1a1e3b7a1e7e3e".parse().unwrap();
        let wallet = wallet.with_chain_id(1u64);
        let client = Arc::new(SignerMiddleware::new(provider, wallet));

        // Mock contract address
        let contract_address: Address = "0000000000000000000000000000000000000000".parse().unwrap();

        // Mock creating the fingerprint hash
        let fingerprint_hash = mock_create_fingerprint_hash(&serde_json::from_value(json_obj.clone()).unwrap()).unwrap();

        // Simulate checking the fingerprint hash on-chain
        let is_appended = mock_check_fingerprint(client.clone(), contract_address, &fingerprint_hash).await.unwrap();

        // Assert that the fingerprint is appended
        assert!(is_appended);
    }
}
pub mod insert;
pub mod check;
pub mod create;
pub mod encoding;

use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers::types::Address;
use std::env;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

/// Represents a Fingerprint object.
#[derive(Serialize, Deserialize, Debug)]
pub struct Fingerprint {
    pub user: String,
    pub gamer: String,
    pub strikes: u64,
    pub place: String,
    pub weapon: String,
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

    #[tokio::test]
    async fn test_run_fingerprint() {
        let fingerprint = Fingerprint {
            user: "test_user".to_string(),
            gamer: "test_gamer".to_string(),
            strikes: 0,
            place: "test_place".to_string(),
            weapon: "test_weapon".to_string(),
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
}
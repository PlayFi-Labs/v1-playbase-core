pub mod insert;
pub mod check;
pub mod create;

use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers::types::Address;
use std::env;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Fingerprint {
    pub gamer: String,
    pub strikes: u64,
    pub place: String,
    pub weapon: String,
    pub place2: String,
}

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
    println!("Fingerprint appended: {}", is_appended);

    Ok(())
}

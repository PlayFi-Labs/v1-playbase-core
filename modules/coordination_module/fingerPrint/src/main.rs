use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers::types::{transaction::eip2718::TypedTransaction, Address};
use ethers::utils::keccak256;
use hex;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tokio;

#[derive(Serialize, Deserialize, Debug)]
struct Fingerprint {
    gamer: String,
    strikes: u64,
    place: String,
    weapon: String,
    place2: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // Debugging output
    println!("FINGERPRINT_PROXY_SC: {:?}", env::var("FINGERPRINT_PROXY_SC"));
    println!("ZKSYNC_SEPOLIA_PRIVATE_KEY: {:?}", env::var("ZKSYNC_SEPOLIA_PRIVATE_KEY"));

    let zk_sync_url = "https://sepolia.era.zksync.dev";
    let provider = Provider::<Http>::try_from(zk_sync_url)?;
    let contract_address_str = env::var("FINGERPRINT_PROXY_SC")?;
    let private_key = env::var("ZKSYNC_SEPOLIA_PRIVATE_KEY")?;

    let contract_address: Address = contract_address_str.parse()?;
    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(300u64); // Sepolia chain ID as u64
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    let fingerprint = Fingerprint {
        gamer: "Merkle".to_string(),
        strikes: 793287,
        place: "SP".to_string(),
        weapon: "AK-4724".to_string(),
        place2: "Y".to_string(),
    };

    let fingerprint_hash = create_fingerprint_hash(&fingerprint)?;

    insert_fingerprint(client.clone(), contract_address, &fingerprint_hash).await?;
    let is_appended = check_fingerprint(client.clone(), contract_address, &fingerprint_hash).await?;
    println!("Fingerprint appended: {}", is_appended);

    Ok(())
}

fn create_fingerprint_hash(fingerprint: &Fingerprint) -> Result<String, Box<dyn std::error::Error>> {
    let json = serde_json::to_string(fingerprint)?;
    let hash = keccak256(json.as_bytes());
    Ok(format!("0x{}", hex::encode(hash)))
}

async fn insert_fingerprint(
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract_address: Address,
    fingerprint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let function_signature_check = "isHashAppended(bytes32)";
    let function_hash_check = &keccak256(function_signature_check.as_bytes())[0..4];
    let data_hash_padded = hex::decode(&fingerprint[2..])?;
    let mut data_check = Vec::new();
    data_check.extend_from_slice(function_hash_check);
    data_check.extend_from_slice(&data_hash_padded);

    let call = client.call(&TypedTransaction::Legacy(TransactionRequest::new().to(contract_address).data(data_check).from(client.address())), None).await?;
    let is_appended: bool = ethers::abi::decode(&[ethers::abi::ParamType::Bool], &call)?.pop().unwrap().into_bool().unwrap();
    if is_appended {
        println!("Fingerprint Hash already inserted. Operation stopped");
        return Ok(());
    }

    let function_signature = "appendData(bytes32)";
    let function_hash = &keccak256(function_signature.as_bytes())[0..4];
    let mut data = Vec::new();
    data.extend_from_slice(function_hash);
    data.extend_from_slice(&data_hash_padded);

    let tx = client.send_transaction(TypedTransaction::Legacy(TransactionRequest::new().to(contract_address).data(data).from(client.address())), None).await?;
    println!("Transaction sent: {:?}", tx);
    let receipt = tx.await?;
    println!("Transaction confirmed: {:?}", receipt);

    Ok(())
}

async fn check_fingerprint(
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract_address: Address,
    fingerprint: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let function_signature_check = "isHashAppended(bytes32)";
    let function_hash_check = &keccak256(function_signature_check.as_bytes())[0..4];
    let data_hash_padded = hex::decode(&fingerprint[2..])?;
    let mut data_check = Vec::new();
    data_check.extend_from_slice(function_hash_check);
    data_check.extend_from_slice(&data_hash_padded);

    let call = client.call(&TypedTransaction::Legacy(TransactionRequest::new().to(contract_address).data(data_check).from(client.address())), None).await?;
    let is_appended: bool = ethers::abi::decode(&[ethers::abi::ParamType::Bool], &call)?.pop().unwrap().into_bool().unwrap();
    Ok(is_appended)
}
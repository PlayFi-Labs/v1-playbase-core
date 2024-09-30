use ethers::prelude::*;
use ethers::types::{transaction::eip2718::TypedTransaction, Address};
use ethers::contract::{Contract, EthEvent};  // Import EthEvent for events
use ethers::abi::Abi;
use std::sync::Arc;
use serde_json::Value;
use crate::encoding::encode::encode_proof;
use anyhow::{Context, Result};
use std::env;
use ethers::types::{U256, Bytes};
use lazy_static::lazy_static;
use std::fs;

lazy_static! {
    static ref FIELD_SIZE: U256 = U256::from_dec_str("21888242871839275222246405745257275088548364400416034343698204186575808495617").unwrap();
}

/// Struct to represent the Debug event in Solidity
#[derive(Debug, Clone, EthEvent)]
pub struct DebugEvent {
    #[ethevent(name = "message", indexed = false)]
    pub message: String,

    #[ethevent(name = "value", indexed = false)]
    pub value: U256,
}

/// Verifies if a zk-SNARK proof is valid on the blockchain using the Groth16Verifier contract.
///
/// # Parameters
/// - `client`: The ether client connected to the blockchain.
/// - `contract_address`: The address of the verifier contract.
/// - `proof`: The JSON object containing zk-SNARK proof data.
///
/// # Returns
/// - `Result<bool>`: Returns `true` if the proof is valid, otherwise `false`.
pub async fn check_proof(
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract_address: Address,
    proof: &Value,
) -> Result<bool> {
    // Validate proof values
    validate_proof(proof).context("Proof contains invalid values")?;

    // Encode the proof
    let data_check = encode_proof(proof).context("Failed to encode proof")?;

    // Create a TypedTransaction for gas estimation
    let mut tx_request = TypedTransaction::Legacy(TransactionRequest::new()
        .to(contract_address)
        .data(data_check)
        .from(client.address()));

    // Dynamically estimate gas
    let estimated_gas = client
        .estimate_gas(&tx_request, None)
        .await
        .context("Failed to estimate gas")?;

    // Log the estimated gas for debugging
    println!("Estimated gas: {:?}", estimated_gas);

    // Set the estimated gas
    tx_request.set_gas(U256::from(5000000));

    // Send the transaction to the contract to verify the proof
    let pending_tx = client
        .send_transaction(tx_request, None)
        .await
        .context("Failed to send transaction")?;

    // Await the transaction result and check the status
    let receipt = pending_tx
        .confirmations(1)
        .await
        .context("Transaction failed to confirm")?;

    if let Some(receipt) = receipt {
        if receipt.status == Some(U64::from(1u64)) {
            println!("Proof is valid!");

            // Extract the provider from the middleware and pass it as `Arc<Provider<Http>>`
            let provider: Arc<Provider<Http>> = Arc::new(client.provider().clone());
            capture_debug_event(provider, contract_address).await?;
            return Ok(true);
        } else {
            println!("Proof verification failed.");
            return Ok(false);
        }
    }

    Err(anyhow::anyhow!("Transaction receipt not found"))
}

/// Validates that all values in the proof are less than the field size.
fn validate_proof(proof: &Value) -> Result<()> {
    let elements_to_check = vec![
        "_pA0", "_pA1", "_pB00", "_pB01", "_pB10", "_pB11", "_pC0", "_pC1", "_pubSignals0"
    ];

    for element in elements_to_check {
        let value_str = proof[element].as_str().context(format!("Missing proof element: {}", element))?;
        let value = U256::from_dec_str(value_str).context(format!("Invalid decimal value in proof: {}", element))?;
        if value >= *FIELD_SIZE {
            anyhow::bail!("Proof element {} is greater than the field size", element);
        }
    }
    Ok(())
}

/// Runs the zk-SNARK proof verification process.
///
/// # Parameters
/// - `proof`: The JSON object containing zk-SNARK proof data.
///
/// # Returns
/// - `Result<()>`: Returns an error if the process fails.
pub async fn run_proof_verification(proof: Value) -> Result<()> {
    dotenv::dotenv().ok();

    // Fetch necessary environment variables
    let zksync_url = env::var("ZKSYNC_URL").context("ZKSYNC_URL not set")?;
    let chain_id: u64 = env::var("CHAIN_ID")?.parse().context("Invalid CHAIN_ID")?;
    let contract_address_str: String = env::var("VERIFIER_CONTRACT_ADDRESS").context("VERIFIER_CONTRACT_ADDRESS not set")?;
    let private_key = env::var("ZKSYNC_SEPOLIA_PRIVATE_KEY").context("ZKSYNC_SEPOLIA_PRIVATE_KEY not set")?;

    // Set up the Ethereum client
    let provider = Provider::<Http>::try_from(zksync_url).context("Invalid ZKSYNC_URL")?;
    let wallet: LocalWallet = private_key.parse().context("Invalid private key")?;
    let wallet: Wallet<k256::ecdsa::SigningKey> = wallet.with_chain_id(chain_id);
    let client: Arc<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>> = Arc::new(SignerMiddleware::new(provider, wallet));

    // Parse contract address
    let contract_address: Address = contract_address_str.parse().context("Invalid contract address")?;

    // Verify the proof by calling `check_proof`
    let is_valid = check_proof(client.clone(), contract_address, &proof)
        .await
        .context("Failed to verify proof")?;

    // Output the result
    if is_valid {
        println!("Proof is valid!");
    } else {
        println!("Proof verification failed.");
    }

    Ok(())
}

/// Function to capture Debug events from the contract
///
/// # Parameters
/// - `client`: The client (Arc<Provider<Http>>)
/// - `contract_address`: The address of the contract
///
/// # Returns
/// - `Result<()>`: Returns an error if event capture fails
pub async fn capture_debug_event(client: Arc<Provider<Http>>, contract_address: Address) -> Result<()> {
    // Load the ABI from a file
    let abi: Abi = serde_json::from_str(&fs::read_to_string("./Verifier.json")?)?; // Adjust the path to your ABI file

    // Create a contract instance
    let contract: Contract<Provider<Http>> = Contract::new(contract_address, abi, client.clone());

    // Create a filter for the Debug event using the DebugEvent struct
    let filter = contract.event::<DebugEvent>();

    // Query for past events
    let logs = filter.query().await?;

    // Process the logs
    for log in logs {
        println!("Event Debug: {:?}, {:?}", log.message, log.value);
    }

    Ok(())
}

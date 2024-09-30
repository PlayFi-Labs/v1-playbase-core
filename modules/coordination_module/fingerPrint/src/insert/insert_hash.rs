use ethers::prelude::*;
use ethers::types::{transaction::eip2718::TypedTransaction, Address};
use std::sync::Arc;
use std::process;
use crate::encoding::encode::encode_function;

/// Inserts a given fingerprint hash into the blockchain.
///
/// # Parameters
/// - `client`: The ether client connected to the blockchain.
/// - `contract_address`: The address of the smart contract.
/// - `fingerprint`: The fingerprint hash to be inserted.
///
/// # Returns
/// - `Result<(), Box<dyn std::error::Error>>`: Returns `Ok(())` if the insertion is successful.
pub async fn insert_fingerprint(
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract_address: Address,
    fingerprint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
   // Define the function signature for checking if a hash is appended
   let function_signature_check = "isHashAppended(bytes32)";
   let data_check = encode_function(fingerprint, function_signature_check)?;

    // Call the smart contract to check if the hash is appended
    let call = client.call(&TypedTransaction::Legacy(TransactionRequest::new().to(contract_address).data(data_check).from(client.address())), None).await?;
    let is_appended: bool = ethers::abi::decode(&[ethers::abi::ParamType::Bool], &call)?.pop().unwrap().into_bool().unwrap();
    if is_appended {
        println!("\x1b[31;1mFingerprint Hash already inserted. Operation stopped\x1b[0m");
        process::exit(0);    
    }

    // Define the function signature for appending a hash
    let function_signature = "appendData(bytes32)";
    let data = encode_function(fingerprint, function_signature)?;

    // Send the transaction to append the hash
    let tx = client.send_transaction(TypedTransaction::Legacy(TransactionRequest::new().to(contract_address).data(data).from(client.address())), None).await?;
    println!("Transaction sent:");
    println!("\x1b[32;1m{:?}\x1b[0m", tx.tx_hash());
    tx.await?.ok_or("Failed to fetch transaction receipt")?;
    println!("\x1b[32;1mTransaction confirmed\x1b[0m");

    Ok(())
}

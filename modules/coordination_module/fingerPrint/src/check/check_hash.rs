use ethers::prelude::*;
use ethers::types::{transaction::eip2718::TypedTransaction, Address};
use serde_json::Value;
use std::sync::Arc;
use std::error::Error;
use crate::encoding::encode::encode_function;
use crate::create::create_hash::create_fingerprint_hash;




/// Checks if a given fingerprint hash has already been appended to the blockchain.
///
/// # Parameters
/// - `client`: The ether client connected to the blockchain.
/// - `contract_address`: The address of the smart contract.
/// - `fingerprint`: The fingerprint hash to be checked.
///
/// # Returns
/// - `Result<bool, Box<dyn std::error::Error>>`: Returns `true` if the hash is already appended, otherwise `false`.
pub async fn check_fingerprint(
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract_address: Address,
    fingerprint: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Define the function signature for checking if a hash is appended
    let function_signature_check = "isHashAppended(bytes32)";
    let data_check = encode_function(fingerprint, function_signature_check)?;

    // Call the smart contract to check if the hash is appended
    let call = client.call(&TypedTransaction::Legacy(TransactionRequest::new().to(contract_address).data(data_check).from(client.address())), None).await?;
    let is_appended: bool = ethers::abi::decode(&[ethers::abi::ParamType::Bool], &call)?.pop().unwrap().into_bool().unwrap();
    Ok(is_appended)
}

/// Processes the JSON object from the zk-module by generating a fingerprint hash
/// and checking if the hash is already appended to the blockchain.
///
/// # Parameters
/// - `client`: The ethers client connected to the blockchain.
/// - `contract_address`: The address of the smart contract.
/// - `generated_json`: The JSON object returned by the zk-module after processing.
///
/// # Returns
/// - `Result<bool, Box<dyn std::error::Error>>`: Returns `true` if the hash is already appended, otherwise `false`.
pub async fn process_and_check_fingerprint(
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract_address: Address,
    generated_json: &Value,
) -> Result<bool, Box<dyn Error>> {
    // Use the `create_fingerprint_hash` function to generate the fingerprint hash from the JSON response.
    let fingerprint_hash = create_fingerprint_hash(&serde_json::from_value(generated_json.clone())?)?;

    // Check if the fingerprint hash is already appended to the blockchain
    let is_appended = check_fingerprint(client, contract_address, &fingerprint_hash).await?;

    // Return the result of the check: `true` if the hash exists, otherwise `false`.
    if is_appended {
        Ok(true)
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Fingerprint not found on the blockchain")))
    }
}

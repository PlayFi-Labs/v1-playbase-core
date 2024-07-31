use ethers::prelude::*;
use ethers::types::{transaction::eip2718::TypedTransaction, Address};
use std::sync::Arc;
use crate::encoding::encode::encode_function;

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

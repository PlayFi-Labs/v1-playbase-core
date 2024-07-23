use ethers::prelude::*;
use ethers::types::{transaction::eip2718::TypedTransaction, Address};
use ethers::utils::keccak256;
use std::sync::Arc;

pub async fn check_fingerprint(
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

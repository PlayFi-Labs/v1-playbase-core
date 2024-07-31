use ethers::utils::keccak256;

pub fn encode_function(fingerprint: &str, function_signature: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let function_hash = &keccak256(function_signature.as_bytes())[0..4];
    let data_hash_padded = hex::decode(&fingerprint[2..])?;
    let mut data_check = Vec::new();
    data_check.extend_from_slice(function_hash);
    data_check.extend_from_slice(&data_hash_padded);
    Ok(data_check)
}
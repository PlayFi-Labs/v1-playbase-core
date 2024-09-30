use ethers::abi::Token;
use ethers::types::U256;
use serde_json::Value;
use anyhow::{Result, anyhow};

/// Encodes the zk-SNARK proof data from JSON into the expected ABI format.
///
/// # Parameters
/// - `proof`: The JSON object containing zk-SNARK proof data.
///
/// # Returns
/// - `Result<Vec<u8>>`: Returns the encoded proof in ABI format.
pub fn encode_proof(proof: &Value) -> Result<Vec<u8>> {
    // Try to extract and convert the proof values from the JSON object
    let p_a0 = U256::from_dec_str(proof["_pA0"].as_str().ok_or_else(|| anyhow!("Invalid _pA0 value"))?)?;
    let p_a1 = U256::from_dec_str(proof["_pA1"].as_str().ok_or_else(|| anyhow!("Invalid _pA1 value"))?)?;

    let p_b00 = U256::from_dec_str(proof["_pB00"].as_str().ok_or_else(|| anyhow!("Invalid _pB00 value"))?)?;
    let p_b01 = U256::from_dec_str(proof["_pB01"].as_str().ok_or_else(|| anyhow!("Invalid _pB01 value"))?)?;
    let p_b10 = U256::from_dec_str(proof["_pB10"].as_str().ok_or_else(|| anyhow!("Invalid _pB10 value"))?)?;
    let p_b11 = U256::from_dec_str(proof["_pB11"].as_str().ok_or_else(|| anyhow!("Invalid _pB11 value"))?)?;

    let p_c0 = U256::from_dec_str(proof["_pC0"].as_str().ok_or_else(|| anyhow!("Invalid _pC0 value"))?)?;
    let p_c1 = U256::from_dec_str(proof["_pC1"].as_str().ok_or_else(|| anyhow!("Invalid _pC1 value"))?)?;

    let pub_signal0 = U256::from_dec_str(proof["_pubSignals0"].as_str().ok_or_else(|| anyhow!("Invalid _pubSignals0 value"))?)?;

    // Create the array of tokens to encode the proof data
    let tokens = vec![
        Token::FixedArray(vec![Token::Uint(p_a0), Token::Uint(p_a1)]),
        Token::FixedArray(vec![
            Token::FixedArray(vec![Token::Uint(p_b00), Token::Uint(p_b01)]),
            Token::FixedArray(vec![Token::Uint(p_b10), Token::Uint(p_b11)]),
        ]),
        Token::FixedArray(vec![Token::Uint(p_c0), Token::Uint(p_c1)]),
        Token::FixedArray(vec![Token::Uint(pub_signal0)]),
    ];

    // Return the encoded proof
    Ok(ethers::abi::encode(&tokens))
}

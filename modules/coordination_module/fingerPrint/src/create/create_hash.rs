use ethers::utils::keccak256;
use hex;
use serde_json;

pub fn create_fingerprint_hash(fingerprint: &crate::Fingerprint) -> Result<String, Box<dyn std::error::Error>> {
    let json = serde_json::to_string(fingerprint)?;
    let hash = keccak256(json.as_bytes());
    println!("Fingerprint hash:");
    println!("\x1b[32;1m0x{}\x1b[0m", hex::encode(hash));
    Ok(format!("0x{}", hex::encode(hash)))
}

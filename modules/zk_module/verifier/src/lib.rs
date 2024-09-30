pub mod encoding;
pub mod check;

use serde_json::Value;
use check::check_proof::run_proof_verification;
use anyhow::Result;

/// Runs the zk-SNARK proof verification process by using the `run_proof_verification` function from the verifier module.
pub async fn verify_proof_with_json(proof: Value) -> Result<()> {
    run_proof_verification(proof).await
}

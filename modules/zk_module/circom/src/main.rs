use anyhow::Result;
use circom::call_js_proof_and_verify;

fn main() -> Result<()> {
    let js_file_path = "./utils/preprocess_input.js";

    let verify_result = call_js_proof_and_verify(js_file_path)?;

    println!("{}", verify_result);

    Ok(())
}

use std::process::Command;
use anyhow::Result;

pub fn call_js_proof_and_verify(js_file_path: &str) -> Result<bool> {

    let output = Command::new("node")
        .arg(js_file_path)
        .output()?;

    if !output.stderr.is_empty() {
        eprintln!("Error output: {}", std::str::from_utf8(&output.stderr)?);
    }

    let result_str = std::str::from_utf8(&output.stdout)?.trim();

    Ok(result_str == "true")
}

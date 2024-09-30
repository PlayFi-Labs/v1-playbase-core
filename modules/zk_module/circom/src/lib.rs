pub mod proof;

use std::process::{Command, Stdio};
use anyhow::{Result, Context};
use serde_json::Value;
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

/// Calls a JavaScript proof generation script and returns the result as a JSON value.
pub fn call_js_proof(script_path: &str) -> Result<Value> {
    println!("Executing node script: {}", script_path);
    
    let output = Command::new("node")
        .arg(script_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to execute node process")?;

    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error in JS script stderr: {}", stderr);
    }

    if !output.status.success() {
        eprintln!("JS script exited with non-zero status");
        return Err(anyhow::anyhow!("Failed to generate proof. Script exited with non-zero status."));
    }

    // Capturamos la salida de stdout (la prueba generada)
    let stdout = String::from_utf8(output.stdout).context("Failed to capture JS stdout")?;
    println!("JS script output: {}", stdout);

    // Intentamos parsear el stdout como JSON
    let proof_data: Value = serde_json::from_str(&stdout).context("Failed to parse proof JSON from JS stdout")?;

    Ok(proof_data)
}
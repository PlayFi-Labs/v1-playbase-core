use json_comparator::run_json_comparator;
use fingerprint::{run_fingerprint, Fingerprint};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ejecutar json_comparator y obtener el JSON con mejor similitud
    let best_json_str = match run_json_comparator() {
        Some(json) => json,
        None => {
            println!("No JSON objects met the similarity threshold.");
            return Ok(());
        }
    };

    // Parsear el JSON resultante para crear un objeto Fingerprint
    let best_json: Value = serde_json::from_str(&best_json_str)?;
    let fingerprint = Fingerprint {
        gamer: best_json["character"].as_str().unwrap_or_default().to_string(),
        strikes: 0, // Asigna un valor adecuado aquí
        place: best_json["place"].as_str().unwrap_or_default().to_string(),
        weapon: best_json["ability"].as_str().unwrap_or_default().to_string(),
        place2: best_json["place2"].as_str().unwrap_or_default().to_string(),
    };

    // Ejecutar el proceso de fingerprint con el objeto Fingerprint resultante
    run_fingerprint(fingerprint).await?;

    Ok(())
}

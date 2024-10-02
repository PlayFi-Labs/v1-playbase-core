use tokio::task;
use serde_json::Value;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Spawn the cm_write task to run in parallel
    let write_task = task::spawn(async {
        if let Err(e) = coordination_module::cm_write().await {
            eprintln!("Error in cm_write: {}", e);
        } else {
            println!("cm_write executed successfully.");
        }
    });

    // Spawn the cm_query task to run in parallel with a simulated JSON
    let query_task = task::spawn(async {
        println!("Starting cm_query task...");

        let generated_json = serde_json::json!({
            "user": "Atujlssasd",
            "game": "mgkmkwnheu",
            "character": "mgkmkwnheu",
            "strikes": 0,
            "place": "qlomlmqxmt",
            "place2": "xingsaseys"
        });

        println!("Generated JSON for query: {}", generated_json);

        match coordination_module::cm_query(&generated_json).await {
            Ok(result) => {
                if result {
                    println!("Fingerprint found on the blockchain.");
                } else {
                    println!("Fingerprint not found on the blockchain.");
                }
            }
            Err(e) => eprintln!("Error in cm_query: {}", e),
        }

        println!("Finished cm_query task.");
    });

    // Wait for both tasks to complete
    let _ = tokio::try_join!(write_task, query_task)?;

    println!("\n====================");
    println!("All tasks have been completed.");
    println!("====================\n");

    Ok(())
}

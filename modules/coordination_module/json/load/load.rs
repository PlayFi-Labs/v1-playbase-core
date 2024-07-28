use std::fs;

/// Asynchronously loads JSON objects from a specified directory.
///
/// # Parameters
/// - `directory`: The directory from which JSON files will be loaded.
///
/// # Returns
/// - `Vec<String>`: A vector containing the contents of the loaded JSON files as strings.
pub async fn load_json_objects(directory: &str) -> Vec<String> {
    let mut json_objects = Vec::new();
    let paths = fs::read_dir(directory).unwrap();

    // Iterate over each path in the specified directory
    for path in paths {
        let path = path.unwrap().path();
        
        // Check if the file extension is ".json"
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let json_str = fs::read_to_string(&path).unwrap();
            json_objects.push(json_str);
        }
    }

    json_objects
}
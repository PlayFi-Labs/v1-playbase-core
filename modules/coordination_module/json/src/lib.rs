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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    use serde_json::Value;

    #[tokio::test]
    async fn test_load_json_objects() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        let file_path1 = dir_path.join("file1.json");
        let mut file1 = File::create(&file_path1).unwrap();
        writeln!(file1, r#"{{"key": "value1"}}"#).unwrap();

        let file_path2 = dir_path.join("file2.json");
        let mut file2 = File::create(&file_path2).unwrap();
        writeln!(file2, r#"{{"key": "value2"}}"#).unwrap();

        let json_objects = load_json_objects(dir_path.to_str().unwrap()).await;

        let expected_json1: Value = serde_json::from_str(r#"{"key": "value1"}"#).unwrap();
        let expected_json2: Value = serde_json::from_str(r#"{"key": "value2"}"#).unwrap();

        let loaded_json1: Value = serde_json::from_str(&json_objects[0]).unwrap();
        let loaded_json2: Value = serde_json::from_str(&json_objects[1]).unwrap();

        assert_eq!(json_objects.len(), 2);
        assert!(loaded_json1 == expected_json1 || loaded_json1 == expected_json2);
        assert!(loaded_json2 == expected_json1 || loaded_json2 == expected_json2);
    }

    #[tokio::test]
    async fn test_load_json_objects_empty_directory() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        let json_objects = load_json_objects(dir_path.to_str().unwrap()).await;

        assert!(json_objects.is_empty());
    }
}
use std::fs;

pub async fn load_json_objects(directory: &str) -> Vec<String> {
    let mut json_objects = Vec::new();
    let paths = fs::read_dir(directory).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let json_str = fs::read_to_string(&path).unwrap();
            json_objects.push(json_str);
        }
    }

    json_objects
}
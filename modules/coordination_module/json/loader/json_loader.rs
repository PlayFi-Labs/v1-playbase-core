use std::fs;

fn main() {
    fn load_json_objects(directory: &str) -> Vec<String> {
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
    let json_objects = load_json_objects("modules/coordination_module/json/src/json_objects");
    
    // Imprime los objetos JSON cargados para verificar que se han cargado correctamente
    for (i, json) in json_objects.iter().enumerate() {
        println!("JSON Object {}: {}", i + 1, json);
    }
}


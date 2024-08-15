use anyhow::Result;
use chrono::{Utc};
use ethereum_types::H160;
use rand::*;
pub type ByteArray32 = [u8; 32];
pub const JSON_PATH: &str = "json_records";
use std::{fs::{create_dir_all, File, OpenOptions}, hash::{DefaultHasher, Hash, Hasher}, path::Path};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
// DB Sruct
pub struct JsonRecords {
    pub game: String,
    pub character: String,
    pub ability: String,
    pub place: String,
    pub place2: String,

    pub aimodel: u32, // 0 to 4,294,967,295
    pub aiversion: u32, // 0 to 4,294,967,295
    pub ainode: u32, // 0 to 4,294,967,295
    pub uploader: H160, // Ethereum address
    pub timestamp: String, 
    pub source: u8, // 0=TV, 1=Sport, 2=LiveWeather, 4=Document
    pub sourcetype: u8,// Source Type
    pub hash_inputdata: ByteArray32,
}
pub fn json_random_values() -> JsonRecords {
    let mut rng = rand::thread_rng();
    
    let game = (0..10).map(|_| rng.gen_range(b'a'..b'z') as char).collect();
    let character = (0..10).map(|_| rng.gen_range(b'a'..b'z') as char).collect();
    let ability = (0..10).map(|_| rng.gen_range(b'a'..b'z') as char).collect();
    let place = (0..10).map(|_| rng.gen_range(b'a'..b'z') as char).collect();
    let place2 = (0..10).map(|_| rng.gen_range(b'a'..b'z') as char).collect();

    let aimodel: u32 = rng.gen_range(0..4_294_967_295); // 0 to 4,294,967,295
    let aiversion: u32 = rng.gen_range(0..4_294_967_295); // 0 to 4,294,967,295
    let ainode: u32 = rng.gen_range(0..4_294_967_295); // 0 to 4,294,967,295
    let uploader: H160 = H160::from_slice(&[0u8; 20]); // Ethereum address
    let timestamp = Utc::now().to_string();
    let source: u8 = rng.gen_range(0..4); // 0:TV, 1:Sport, 2:LiveWeather, 4:Document
    let sourcetype: u8 = rng.gen_range(0..4); 
    let hash_inputdata: ByteArray32 = [2; 32];
    JsonRecords { game, character, ability, place, place2, aimodel, aiversion, ainode, uploader, timestamp, source, sourcetype, hash_inputdata }
}

fn read_file(path: &Path) -> Result<File> {
    OpenOptions::new().read(true).write(true).open(path).map_err(|e| e.into())
}

pub fn read_one(path: &Path) -> Result<JsonRecords> {
    let reader = read_file(path)?;
    let metadata: JsonRecords = serde_json::from_reader(reader)?;
    Ok(metadata)
}
pub fn gen_one_random_json_record(path: &str, name_number: usize) -> Result<()> {
    let mut filename = name_number.to_string();
    filename.push_str(".json");
    let path = Path::new(path).join(filename);
    let metadata = json_values_create();
    create_json_file(&path, &metadata)
}
fn create_json_file(path: &Path, metadata: &JsonRecords) -> Result<()> {
    let serialized = serde_json::to_string_pretty(&metadata)?;
    std::fs::write(path, serialized)?;
    Ok(())
}

pub fn gen_many_json_record(path: &str, num: usize) -> Result<()> {
    create_dir_all(JSON_PATH).unwrap();
    for i in 1..=num {
        gen_one_random_json_record(&path, i)?;
    }
    Ok(())
}
fn json_values_create() -> JsonRecords {
    let json_random_val = json_random_values();
    let concat = concat_record_values(&json_random_val);
    // use SIPhash
    let hash = hash_record(&concat);
    let returndata = JsonRecords { hash_inputdata: u32_to_array(hash), ..json_random_val };
    returndata
}
pub fn gen_one_json_record(path: &str, filename: String) -> Result<JsonRecords> {
    let mut filename = filename;
    filename.push_str(".json");
    let path = Path::new(path).join(filename);
    let metadata = json_values_create();
    create_json_file(&path, &metadata)?;
    Ok(metadata)
}
pub fn gen_one_json_record_with_name(path: &str, filename: String, name: String) -> Result<JsonRecords> {
    let mut filename = filename;
    filename.push_str(".json");
    let path = Path::new(path).join(filename);
    let mut json_random_val = json_random_values();
    // Only the character field is set.
    json_random_val.character = name;
    let concat = concat_record_values(&json_random_val);
    let hash = hash_record(&concat);
    let metadata = JsonRecords { hash_inputdata: u32_to_array(hash), ..json_random_val };
    create_json_file(&path, &metadata)?;
    Ok(metadata)
}
pub fn concat_record_values(user_record: &JsonRecords) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}",
        user_record.game,
        user_record.character,
        user_record.ability,
        user_record.place,
        user_record.place2,
        user_record.aimodel,
        user_record.aiversion,
        user_record.ainode,
        user_record.uploader,
        user_record.timestamp,
        user_record.source,
        user_record.sourcetype,
    )
}
pub fn hash_record(input: &str) -> u32 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash_value = hasher.finish() % u32::MAX as u64;
    hash_value as u32
}

pub fn u32_to_array(value: u32) -> [u8; 32] {
    let mut array = [0u8; 32];
    let bytes = value.to_string().into_bytes();
    let length = bytes.len() as u8;
    
    array[0] = length;  // Store the length at the first position
    for i in 1..32 {
        array[i] = bytes[(i - 1) % bytes.len()];
    }
    array
}

pub fn array_to_u32(array: Vec<u8>) -> u32 {
    let length = array[0] as usize;
    let unique_bytes = &array[1..1 + length];
    let byte_string = String::from_utf8(unique_bytes.to_vec()).unwrap();
    byte_string.parse::<u32>().unwrap()
}

use lazy_static::lazy_static;
use std::fs::File;
use std::io::BufReader;
use serde_json;
use serde::Deserialize;


lazy_static! {
    pub static ref CONFIG: Config = {
        println!("LAZY STATIC!");
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/config.json");
        println!("PATH: {}", path);

        // Open the file in read-only mode with buffer.
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `Config`.
        serde_json::from_reader(reader).unwrap()
    };
 }

 #[derive(Debug, Deserialize)]
 #[serde(rename_all = "camelCase")]
pub struct Config {
    pub jdbc_url: String,
    pub username: String,
    pub password: String,
    pub secret_key: String,
    pub maximum_pool_size: u8,
    pub codes: Vec<u8>,
}

impl Config {}
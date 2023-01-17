use super::types::MoveModule;
use std::fs;
use std::path::Path;

pub fn parse_module(path: &Path) -> MoveModule {
    let data = fs::read_to_string(path).expect("Read json data from file failed!");
    let m: MoveModule = serde_json::from_str(&data).unwrap();
    println!("{:?}, {:?}", m.address, m.exposed_functions.len());
    m
}

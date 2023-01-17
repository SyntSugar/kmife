use crate::config::utils::read_yaml;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    program_root: String,
    cli_name: String,
    compile_platform: String,
}

impl Config {
    pub fn new() -> Self {
        read_yaml()
    }
}

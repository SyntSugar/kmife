use crate::config::types::Config;
use serde_yaml;

pub fn read_yaml() -> Config {
    let yaml = include_str!("config.yaml");
    let config: Config = serde_yaml::from_str(yaml).expect("config.yaml read failed!");

    config
}

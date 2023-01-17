mod config;
mod parse_move;
use std::path::Path;

use crate::config::types::Config;
use crate::parse_move::utils::parse_module;

fn main() {
    println!("Welcome to use kmife.\n");

    let config = Config::new();
    println!("{:?}", config);

    let path = Path::new("/Users/hgamiui9/github/kmife/src/clmmpool.json");
    let _module = parse_module(path);
}

use atelier_data::configs::Config;
use std::{env, fs, process};

fn main() {
    let config = match load_config("Config.toml") {
        Ok(cfg) => cfg,
        Err(cfg) => {
            eprintln!("Configuration parsing error: {}", e);
            process::exit(1);
        }
    };
}

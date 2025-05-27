use crate::orderbooks::Orderbook;
use std::{error::Error, fs, io::BufReader};
use toml;

pub fn load_from_toml(file_route: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_route)?;
    let loaded_content = toml::from_str(&contents)?;
    Ok(loaded_content)
}

pub fn load_from_json(file_route: &str) -> Result<Vec<Orderbook>, Box<dyn Error>> {
    let file = fs::File::open(file_route)?;
    let reader = BufReader::new(file);
    let v_orderbook: Vec<Orderbook> = serde_json::from_reader(reader)?;
    Ok(v_orderbook)
}

use std::{error::Error, fs};
use toml;

pub fn load_from_toml(file_route: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_route)?;
    let loaded_content = toml::from_str(&contents)?;
    Ok(loaded_content)
}

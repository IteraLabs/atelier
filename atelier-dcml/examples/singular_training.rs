/// Conduct a Singular Training Process
use atelier_data::{data, templates};
use std::{env, error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    // --- Set up working directory
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(manifest_dir)
        .parent()
        .expect("Failed to get workspace root");

    // --- Load config data
    let config_file = workspace_root
        .join("atelier-dcm")
        .join("experiments")
        .join("single_training_00.toml")
        .to_str()
        .unwrap()
        .to_owned();

    let config = templates::Config::load_from_toml(&config_file)
        .unwrap()
        .clone();

    let _exp_id = &config.experiments[0].id;

    // --- Load Computed Features
    let data_file = workspace_root
        .join("atelier-dcm")
        .join("datasets")
        .join("exp_00_ai_00_binance_ob.json");

    let _v_orderbook = data::load_from_json(&data_file.to_str().unwrap().to_owned())?;

    Ok(())
}

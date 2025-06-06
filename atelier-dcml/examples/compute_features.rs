use atelier_core::data;
use atelier_dcml::features;
/// Compute Features
use std::{env, error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    // --- Set up working directory
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(manifest_dir)
        .parent()
        .expect("Failed to get workspace root");

    // --- Load input data
    let data_file = workspace_root
        .join("atelier-dcml")
        .join("datasets")
        .join("exp_00_ai_00_binance_ob.json");

    let ob_data = data::load_from_json(&data_file.to_str().unwrap().to_owned())?;

    let i_spread = features::compute_spread(&ob_data[0]);
    let i_midprice = features::compute_midprice(&ob_data[0]);
    let i_w_midprice = features::compute_w_midprice(&ob_data[0]);
    let i_imb = features::compute_imb(&ob_data[0]);
    let i_vwap = features::compute_vwap(&ob_data[0], 5 as usize);
    let i_tav = features::compute_tav(&ob_data[0], 0.0001);

    println!("i_spread: {:?}", i_spread);
    println!("i_midprice: {:?}", i_midprice);
    println!("i_w_midprice: {:?}", i_w_midprice);
    println!("i_imb: {:?}", i_imb);
    println!("i_vwap: {:?}", i_vwap);
    println!("i_tav: {:?}", i_tav);

    Ok(())
}

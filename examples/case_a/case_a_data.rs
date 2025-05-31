//! Single Market Synthetic Data Generation

use atelier_core::templates;
use atelier_synth::synthbooks::progressions;
use atelier_dcml::features;
use std::{env, path::Path};

#[tokio::main]
pub async fn main() {

    // --- Setup working directory
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(manifest_dir)
        .parent()
        .expect("Failed to get workspace root");

    // --- Generate Data : (1st) Orderbook
    println!("Single Market Synthetic Data Generation");

    // --- Template file (toml)
    let template_file = workspace_root
        .join("examples")
        .join("case_a")
        .join("config_a.toml");

    let template = templates::Config::load_from_toml(template_file.to_str().unwrap())
        .unwrap()
        .clone();
    
    // --- Extract parameters from template
    let _exp_id = &template.experiments[0].id;
    let n_progres = template.experiments[0].n_progressions as usize;
    let template_orderbook = template.exchanges[0].orderbook.clone().unwrap();
    let template_model = template.models[0].clone();

    // --- Create progressions
    let orderbook = progressions(template_orderbook, template_model, n_progres).await;

    // --- Create Features
    let selected_features = ["spread", "midprice", "w_midprice", "vwap", "imb", "tav"];
    let results_vec = features::compute_features(
        &orderbook.unwrap(),
        &selected_features,
        features::FeaturesOutput::HashMap);
    
    println!("Features: {:?}", results_vec);

}

use atelier_core::templates;
use atelier_dcm::{agents::DistributedAgent, dataset, features, targets, training};
use std::{env, path::Path};
use tch::{Kind, Tensor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // --- Load input data
    let data_file = workspace_root
        .join("atelier-dcm")
        .join("datasets")
        .join("exp_00_ai_00_binance_ob.json");

    let v_orderbook = dataset::read_json(&data_file.to_str().unwrap().to_owned())?;

    // --- Features computation
    let f1 = features::ob_vwap(&v_orderbook, 2 as usize)?;
    let f1_tensor = Tensor::from_slice(&f1).unsqueeze(1);
    let f2 = features::ob_wmidprice(&v_orderbook)?;
    let f2_tensor = Tensor::from_slice(&f2).unsqueeze(1);
    let x_tensor = Tensor::cat(&[f1_tensor, f2_tensor], 1).to_kind(Kind::Float);

    // --- Pre-processing features

    // Shift 1 along dim=0 rows
    let xs_tensor = x_tensor.roll(&[1], &[0]);

    // Standardize values
    let epsilon = 1e-8;
    let xs_1 = &xs_tensor - xs_tensor.mean(Kind::Float);
    let xs_2 = xs_tensor.std(true) + epsilon;
    let features = (xs_1 / xs_2).to_kind(Kind::Float);

    // --- Target data
    let ys_vec = targets::ob_price_direction(&v_orderbook)?;
    let labels = Tensor::from_slice(&ys_vec)
        .unsqueeze(1)
        .to_kind(Kind::Float);

    // --- Regularization Params
    let lambda_1 = 0.015;
    let lambda_2 = 0.015;
    let eta = 0.0;
    let loss = Tensor::from(1e10);
    let accuracy = Tensor::from(1.0);

    let mut agent =
        DistributedAgent::new(features, labels, lambda_1, lambda_2, eta, loss, accuracy);

    training::single_training(&mut agent, 100, 0.04);

    Ok(())
}

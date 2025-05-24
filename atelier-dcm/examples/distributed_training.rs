use atelier_core::training;
use atelier_dcm::{
    agents::DistributedAgent, dataset, features, targets, training::distributed_training,
};

use std::{env, path::Path};
use tch::{display, Kind, Tensor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    display::set_print_options_full();

    // --- experiment parameters
    let n_agents = 9;
    let n_iterations = 500;

    // --- Agents
    let mut agents: Vec<_> = vec![];

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(manifest_dir)
        .parent()
        .expect("Failed to get workspace root");

    // --- Load input data
    let data_route = workspace_root.join("atelier-dcm").join("datasets");

    // --- Files
    let mut data_files = vec![
        String::from("exp_00_ai_00_binance_ob.json"),
        String::from("exp_00_ai_01_gate_ob.json"),
        String::from("exp_00_ai_02_okx_ob.json"),
        String::from("exp_00_eu_00_kraken_ob.json"),
        String::from("exp_00_eu_01_bitstamp_ob.json"),
        String::from("exp_00_eu_02_bitfinex_ob.json"),
        String::from("exp_00_am_00_coinbase_ob.json"),
        String::from("exp_00_am_01_bitso_ob.json"),
        String::from("exp_00_am_02_gemini_ob.json"),
    ];

    // --- Agents Formation --- //
    for i in 0..n_agents {
        println!("\nAgent {:?} preparation", i);

        // --- Features --- //
        let i_route = data_route.join(&data_files.pop().unwrap());
        println!("{:?}", i_route);

        let v_orderbook = dataset::read_json(i_route.to_str().unwrap())?;

        let wmid_price: Vec<f64> = features::ob_wmidprice(&v_orderbook)?;
        let vwap_price: Vec<f64> = features::ob_vwap(&v_orderbook, 2 as usize)?;
        let wmid_price_tensor = Tensor::from_slice(&wmid_price).unsqueeze(1);
        let vwap_price_tensor = Tensor::from_slice(&vwap_price).unsqueeze(1);

        // --- Input data conversion to Tensor
        let xs_vec = [wmid_price_tensor, vwap_price_tensor];
        let xs_pre = Tensor::cat(&xs_vec, 1).to_kind(Kind::Float);

        // Pre-processing: shift=1 along dim=0 (rows)
        let xs = xs_pre.roll(&[1], &[0]);
        // Pre-processing: Standarize Features
        let features =
            ((&xs - xs.mean(Kind::Float)) / (xs.std(true) + 1e-8)).to_kind(Kind::Float);

        // --- Target data
        let ys_vec = targets::ob_price_direction(&v_orderbook)?;
        let labels = Tensor::from_slice(&ys_vec)
            .unsqueeze(1)
            .to_kind(Kind::Float);

        // --- Regularization Params
        let lambda_1 = 0.015;
        let lambda_2 = 0.015;
        let eta = 0.11;
        let loss = Tensor::from(1e10);
        let accuracy = Tensor::from(1.0);

        // -- Agent formation
        let agnt = DistributedAgent::new(
            features, labels, lambda_1, lambda_2, eta, loss, accuracy,
        );

        agents.push(agnt);
    }

    let num_agents = 9;
    // --- Load input data
    let training_file = workspace_root
        .join("atelier-dcm")
        .join("experiments")
        .join("distributed_training_00.toml");

    let consensus_matrix =
        training::a_matrix(num_agents, training_file.to_str().unwrap());

    // Run distributed training
    distributed_training(&mut agents, n_iterations, consensus_matrix.copy());

    println!("\n------ Finished Distributed Training ------\n");

    // Inspect final parameters
    for (i, agent) in agents.iter().enumerate() {
        println!("Agent {} loss: {} acc: {}", i, agent.loss, agent.accuracy);
    }

    Ok(())
}

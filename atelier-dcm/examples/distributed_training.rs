use atelier_dcm::{
    agents::DistributedAgent, dataset, features, targets, training::distributed_training,
};
use tch::{display, Kind, Tensor};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    display::set_print_options_full();

    // --- experiment parameters
    let n_agents = 3;
    let n_iterations = 11;

    // --- Agents
    let mut agents: Vec<_> = vec![];

    // --- Files
    let mut data_files = vec![
        String::from("americas_orderbook.json"),
        String::from("asia_orderbook.json"),
        String::from("europe_orderbook.json"),
    ];

    // --- Agents Formation --- //
    for i in 0..n_agents {
        println!("\nAgent {:?} preparation", i);

        // --- Features --- //
        let v_orderbook_2 = dataset::read_json(&data_files.pop().unwrap())?;

        let wmid_price: Vec<f64> = features::ob_wmidprice(&v_orderbook_2)?;
        let vwap_price: Vec<f64> = features::ob_vwap(&v_orderbook_2, 2 as usize)?;
        let wmid_price_tensor = Tensor::from_slice(&wmid_price).unsqueeze(1);
        let vwap_price_tensor = Tensor::from_slice(&vwap_price).unsqueeze(1);

        // --- Input data conversion to Tensor
        let xs_vec = [wmid_price_tensor, vwap_price_tensor];
        let xs_pre = Tensor::cat(&xs_vec, 1).to_kind(Kind::Float);

        // Pre-processing: shift=1 along dim=0 (rows)
        let xs = xs_pre.roll(&[1], &[0]);
        // Pre-processing: Standarize Features
        let features = ((&xs - xs.mean(Kind::Float)) / (xs.std(true) + 1e-8)).to_kind(Kind::Float);

        // --- Target data
        let ys_vec = targets::ob_price_direction(&v_orderbook_2)?;
        let labels = Tensor::from_slice(&ys_vec)
            .unsqueeze(1)
            .to_kind(Kind::Float);

        // --- Regularization Params
        let lambda_1 = 0.1;
        let lambda_2 = 0.01;
        let eta = 0.33;
        let loss = Tensor::from(1e10);

        // -- Agent formation
        let agnt = DistributedAgent::new(features, labels, lambda_1, lambda_2, eta, loss);

        agents.push(agnt);
    }

    // Run distributed training
    distributed_training(&mut agents, n_iterations);

    println!("\n------ Finished Distributed Training ------\n");

    // Inspect final parameters
    for (i, agent) in agents.iter().enumerate() {
        println!(
            "Agent {} weights: {:?}, loss: {}",
            i, agent.theta, agent.loss
        );
    }

    for (i, agent) in agents.iter().enumerate() {
        // let i_features = agent.features.i(0);
        let i_forecasted = agent.forward(&agent.labels, &agent.labels);
        let i_observed = &agent.labels;

        println!(
            "Agent {} forecasted: {:?}, observed: {}",
            i, i_forecasted, i_observed
        );
    }

    Ok(())
}

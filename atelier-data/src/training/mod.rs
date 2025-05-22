use serde::Deserialize;
use std::fs;
use tch::{Kind, Tensor};
use toml;

#[derive(Deserialize)]
struct Connection {
    from: usize,
    to: usize,
    weight: f64,
}

#[derive(Deserialize)]
struct Training {
    epochs: u32,
    agent_connections: Vec<Connection>,
}

#[derive(Deserialize)]
struct Config {
    training: Vec<Training>,
}

pub fn a_matrix(num_agents: i64, config_file: &str) -> Tensor {
    // Read and parse the TOML file
    let config_str = fs::read_to_string(config_file)
        .expect(&format!("Failed to read config file: {}", config_file));
    let config: Config = toml::from_str(&config_str)
        .expect(&format!("Failed to parse TOML from file: {}", config_file));

    // Create a zero-filled matrix as a flat vector
    let mut matrix_data = vec![0.0; (num_agents * num_agents) as usize];

    // Fill in the matrix with connection weights
    if let Some(training) = config.training.first() {
        for conn in &training.agent_connections {
            let from = conn.from;
            let to = conn.to;

            // Verify indices are within bounds
            if from < num_agents as usize && to < num_agents as usize {
                // Calculate 1D index from 2D coordinates (row-major order)
                let index = from * num_agents as usize + to;
                matrix_data[index] = conn.weight;
            }
        }
    }

    // Convert to tensor and reshape
    Tensor::from_slice(&matrix_data)
        .reshape(&[num_agents, num_agents])
        .to_kind(Kind::Float)
}

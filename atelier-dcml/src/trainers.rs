
use serde::Deserialize;
use std::{fs, error::Error};

use atelier_core::data;
use crate::{models, functions, optimizers, metrics};

#[derive(Debug, Deserialize)]
struct Connection {
    from: usize,
    to: usize,
    weight: f64,
}

#[derive(Debug, Deserialize)]
struct Training {
    epochs: u32,
    agents: u32,
    agent_connections: Vec<Connection>,
}

#[derive(Debug, Deserialize)]
struct TrainingTemplate {
    training: Vec<Training>,
}

#[derive(Debug, Deserialize)]
pub struct ConnectionsMatrix {
    values: Vec<f64>
}

impl ConnectionsMatrix {

    pub fn new(self, n_rows: &usize, n_cols: &usize) -> Self {
        let matrix_values = vec![0.0; (n_rows * n_cols) as usize];
        ConnectionsMatrix { values: matrix_values}
    }

    pub fn fill(mut self, connections_file: &str) -> Result<Self, Box<dyn Error>> { 
    
        // Read and parse the TOML file
        let config_str = fs::read_to_string(connections_file)
            .expect(&format!("Failed to read config file: {}", connections_file));
        let template: TrainingTemplate = toml::from_str(&config_str)
            .expect(&format!("Failed to parse TOML from file: {}", connections_file));
        let n_agents = template.training.first().unwrap().agents as u32;

        // Fill in the matrix with connection weights
        if let Some(training) = template.training.first() {
            for conn in &training.agent_connections {
                let from = conn.from as u32;
                let to = conn.to as u32;
                println!("from: {:?}", from);
                println!("to: {:?}", to);

                if from < n_agents && to < n_agents {
                    let index = from * n_agents + to;
                    self.values[index as usize] = conn.weight;
                }
            }
        }
        Ok(self)
    }
}

/// Singular training
///
/// Models: Model definition
/// Loss: A function that implements 
/// Dataset: Features, Target
/// 

pub struct Singular {
    data: data::Dataset,
    model: models::LogisticClassifier,
    loss: functions::CrossEntropy,
    optimizer: optimizers::GradientDescent,
    metrics: metrics::Metrics,
}

pub struct SingularBuilder {
    data: Option<data::Dataset>,
    model: Option<models::LogisticClassifier>,
    
}


pub struct Distributed {
    data: Vec<data::Dataset>,
    model: Vec<models::LogisticClassifier>,
    loss: Vec<functions::CrossEntropy>,
    optimizer: Vec<optimizers::GradientDescent>,
    metrics: Vec<metrics::Metrics>,
    topology: ConnectionsMatrix,
}


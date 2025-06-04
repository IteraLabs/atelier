/// Trainer Environment

use serde::Deserialize;
use std::{fs, error::Error};
use tch::Tensor;
use atelier_core::data;
use crate::{models, models::Model, functions, optimizers, metrics};

pub enum TrainType {
    Batch,
}

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
    model: models::LinearModel,
    loss: functions::Classification,
    optimizer: optimizers::Gradient,
    metrics: metrics::Metrics,
}

impl Singular {

    pub fn builder() -> SingularBuilder {
        SingularBuilder::new()
    }

    pub fn train(self, epochs: u32) {

        println!("model: {:?}", self.model);

        for i_epoch in 0..epochs {
            
            println!("epoch: {:?}", i_epoch);

        }
    }

        // forward(&features_tensor);
        // load dataset
        // specifiy loss function regularization
        // specify optimizer
        // specify metrics
        // train epochs
        // export results

}

pub struct SingularBuilder {
    data: Option<data::Dataset>,
    model: Option<models::LinearModel>,
    loss: Option<functions::Classification>,
    optimizer: Option<optimizers::Gradient>,
    metrics: Option<metrics::Metrics>,
}

impl SingularBuilder {
    
    pub fn new() -> Self {

        SingularBuilder {
            data: None,
            model: None,
            loss: None,
            optimizer: None,
            metrics: None,
        }
    }

    pub fn data(mut self, data: data::Dataset) -> Self {
        self.data = Some(data);
        self
    }

    pub fn model(mut self, model: models::LinearModel) -> Self {
        self.model = Some(model);
        self
    }

    pub fn loss(mut self, loss: functions::Classification) -> Self {
        self.loss = Some(loss);
        self
    }

    pub fn optimizer(mut self, optimizer: optimizers::Gradient) -> Self {
        self.optimizer = Some(optimizer);
        self
    }

    pub fn metrics(mut self, metrics: metrics::Metrics) -> Self {
        self.metrics = Some(metrics);
        self
    }

    pub fn build(self) -> Result<Singular, &'static str> {
    
        let data = self.data.ok_or("Missing data")?;
        let model = self.model.ok_or("MIssing model")?;
        let loss = self.loss.ok_or("Missing loss")?;
        let optimizer = self.optimizer.ok_or("Missing optimizer")?;
        let metrics = self.metrics.ok_or("Missing metrics")?;
        
        Ok(Singular {
            data,
            model,
            loss,
            optimizer,
            metrics
        })
    }


}


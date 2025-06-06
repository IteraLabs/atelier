use crate::{
    functions,
    functions::{Regularized, RegType},
    metrics,
    models,
    models::Model,
    optimizers,
    optimizers::Optimizer,
};

use atelier_core::data;
/// Trainer Environment
use serde::Deserialize;
use std::{error::Error, fs};

use tch::Kind;

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
    values: Vec<f64>,
}

impl ConnectionsMatrix {

    pub fn new(self, n_rows: &usize, n_cols: &usize) -> Self {
        let matrix_values = vec![0.0; (n_rows * n_cols) as usize];
        ConnectionsMatrix {
            values: matrix_values,
        }
    }

    pub fn fill(mut self, connections_file: &str) -> Result<Self, Box<dyn Error>> {

        // Read and parse the TOML file
        let config_str = fs::read_to_string(connections_file)
            .expect(&format!("Failed to read config file: {}", connections_file));
        let template: TrainingTemplate = toml::from_str(&config_str).expect(&format!(
            "Failed to parse TOML from file: {}",
            connections_file
        ));

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

/// 
/// Singular training
///
/// Models: Model definition
/// Loss: A function that implements
/// Dataset: Features, Target
///

#[derive(Debug)]
pub struct Singular {
    data: data::Dataset,
    model: models::LinearModel,
    loss: functions::CrossEntropy,
    optimizer: optimizers::GradientDescent,
    metrics: metrics::Metrics,
}

impl Singular {
    
    pub fn new() -> SingularBuilder {
        SingularBuilder::new()
    }
    
    pub fn train(&mut self, epochs: u32) -> Result<(), Box<dyn Error>> {
         
        let (features, targets) = &self.data.clone().from_vec_to_tensor();
         
        for epoch in 0..epochs { 

            // --- Forward Step --- //
            let y_hat = self.model.forward(&features);
            
            // --- Compute Loss --- //
            let loss = self.loss.compute_loss(&y_hat, &targets);

            let reg_param_c = 1.9;
            let reg_param_lambda = 0.8;

            let reg_loss = self.loss.regularize(
                &self.model.weights,
                &RegType::Elasticnet,
                vec![reg_param_c, reg_param_lambda]
                ).sum(Kind::Float);
            
            let total_loss = &loss + &reg_loss;
            total_loss.backward();
            
            // --- Compute Gradients --- //
            let c_w = self.model.weights.grad();
            let c_b = self.model.bias.grad();
            
            // --- Compute Step of Learning Algorithm
            self.optimizer.step(
                &mut self.model.weights,
                &mut self.model.bias,
                &c_w,
                &c_b);
            
            // --- Reset gradient value on weights and bias
            self.model.weights.zero_grad();
            self.model.bias.zero_grad();
            
            // --- Get Metrics --- //
            let metrics = self.metrics.compute_all(&y_hat, &targets);
            
            println!("\n--- epoch {:?} --- loss {:?} --- accuracy: {:?}",
                epoch, &loss, metrics["accuracy"]);
        }
        Ok(()) 
    }

    pub fn save_model(self, file_route: &str) {
        // --- Save a model's weight
        let _ = self.model.save_model(file_route);
    }

    pub fn load_model(mut self, file_route: &str) {
        // --- Load a model's weight
        let _ = self.model.load_model(file_route);
    }

}

pub struct SingularBuilder {
    data: Option<data::Dataset>,
    model: Option<models::LinearModel>,
    loss: Option<functions::CrossEntropy>,
    optimizer: Option<optimizers::GradientDescent>,
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

    pub fn loss(mut self, loss: functions::CrossEntropy) -> Self {
        self.loss = Some(loss);
        self
    }

    pub fn optimizer(mut self, optimizer: optimizers::GradientDescent) -> Self {
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
            metrics,
        })
    }
}


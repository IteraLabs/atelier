/// Convex Linear Models

use tch::{Tensor, Kind, Device};
use std::f64;

pub trait Model {
    fn forward(&self, features: &Tensor) -> Tensor;
    fn compute_gradient(&self, features: &Tensor, targets: &Tensor) -> Tensor;
}

#[derive(Debug)]
pub struct LogisticClassifier {
    id: String,
    weights: Tensor,
}

impl LogisticClassifier {

    pub fn builder() -> LinearModelBuilder {
        LinearModelBuilder::new()
    }

}

impl Model for LogisticClassifier {

    fn forward(&self, features: &Tensor) -> Tensor {
        let logits = features.matmul(&self.weights.unsqueeze(-1)).squeeze();
        logits.sigmoid()
    }

    fn compute_gradient(&self, features: &Tensor, targets: &Tensor) -> Tensor {
        let preds = self.forward(features);
        let n_samples = features.size()[0] as f64;
        let error = &preds - targets;
        let gradient = features.transpose(0,1)
            .matmul(&error.unsqueeze(-1))
            .squeeze() / n_samples;
        gradient
    }

}

#[derive(Debug)]
pub struct LinearModelBuilder {
    id: Option<String>,
    weights: Option<Tensor>,
    fan_in: Option<i64>,
    fan_out: Option<i64>,
    device: Device
}

impl LinearModelBuilder {
    
    pub fn new() -> Self {
        LinearModelBuilder { 
            id: None,
            weights: None,
            fan_in: None,
            fan_out: Some(1),
            device: Device::Cpu
        }
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn weights(mut self, weights: Tensor) -> Self {
        self.weights = Some(weights);
        self
    }

    pub fn device(mut self, device: Device) -> Self {
        self.device = device;
        self
    }

    pub fn input_features(mut self, features: i64) -> Self {
        self.fan_in = Some(features);
        self
    }

    pub fn output_units(mut self, units: i64) -> Self {
        self.fan_out = Some(units);
        self
    }

    fn glorot_uniform_init(fan_in: i64, fan_out: i64, device: Device) -> Tensor {

        let limit = (6.0 / (fan_in + fan_out) as f64).sqrt();
        let weights = Tensor::rand(&[fan_in], (Kind::Float, device));
        let scaled_weights = &weights * (2.0 * limit) - limit;
        
        scaled_weights
    }

    pub fn glorot_uniform(mut self) -> Result<Self, &'static str> {
        
        let fan_in = self
            .fan_in.ok_or("fan_in must be specified")?;
        
        let fan_out = self
            .fan_out.ok_or("fan_out must be specified")?;
        
        let weights = Self::glorot_uniform_init(fan_in, fan_out, self.device);
        self.weights = Some(weights);
        Ok(self)

    }

    fn glorot_normal_init(fan_in: i64, fan_out: i64, device: Device) -> Tensor {

        let std = (2.0 / (fan_in + fan_out) as f64).sqrt();
        let weights = Tensor::randn(&[fan_in], (Kind::Float, device));
        &weights * std

    }

    pub fn glorot_normal(mut self) -> Result<Self, &'static str> {
        let fan_in = self
            .fan_in.ok_or("fan_in must be specified for Glorot initialization")?;
        let fan_out = self.
            fan_out.ok_or("fan_out must be specified for Glorot initialization")?;
        
        let weights = Self::glorot_normal_init(fan_in, fan_out, self.device);
        self.weights = Some(weights);
        Ok(self)
    }

    pub fn build(self) -> Result<LogisticClassifier, &'static str> {
        let id = self.id.ok_or("Missing id")?;
        let weights = self.weights.ok_or("Missing weights")?;

        Ok(LogisticClassifier { id, weights })
    }
}


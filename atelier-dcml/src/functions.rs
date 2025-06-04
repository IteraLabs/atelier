use tch::{Tensor, Kind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum RegType {
    L1,
    L2,
    Elasticnet,
}

pub enum Classification {
    CrossEntropy,
}

pub trait Regularized {
    fn regularize(&self, operation: &RegType, params: Vec<f64>) -> Tensor;
}

#[derive(Debug)]
pub struct CrossEntropyBuilder<'a> {
    weights: Option<&'a Tensor>,
    y_true: Option<&'a Tensor>,
    y_hat: Option<&'a Tensor>,
    epsilon: Option<f64>,
}

impl<'a> CrossEntropyBuilder<'a> {

    pub fn new() -> Self {
        CrossEntropyBuilder {
            weights: None,
            y_true: None,
            y_hat: None,
            epsilon: None,
        }
    }

    pub fn weights(mut self, weights: &'a Tensor) -> Self {
        self.weights = Some(&weights);
        self
    }

    pub fn y_true(mut self, y: &'a Tensor) -> Self {
        self.y_true = Some(y);
        self
    }

    pub fn y_hat(mut self, y_hat: &'a Tensor) -> Self {
        self.y_hat = Some(y_hat);
        self
    }

    pub fn epsilon(mut self, epsilon: f64) -> Self {
        self.epsilon = Some(epsilon);
        self
    }

    pub fn build(self) -> Result<CrossEntropy, &'static str> {

        let weights = self.weights.ok_or("Missing Weights value")?;
        let y_true = self.y_true.ok_or("Missing y value")?;
        let y_hat = self.y_hat.ok_or("Missing y_hat value")?;
        let epsilon = self.epsilon.ok_or("Missing epsilon value")?;

        Ok(CrossEntropy {
            weights: weights.copy(),
            y_true: y_true.copy(),
            y_hat: y_hat.copy(),
            epsilon})
    }
}

#[derive(Debug)]
pub struct CrossEntropy {
    pub weights: Tensor,
    pub y_true: Tensor,
    pub y_hat: Tensor,
    pub epsilon: f64,
}

impl CrossEntropy {

    pub fn builder<'a>() -> CrossEntropyBuilder<'a> {
        CrossEntropyBuilder::new()
    }

    pub fn compute_loss(self) -> Tensor {
        let bce = self.y_hat.cross_entropy_for_logits(&self.y_true);
        bce
    }


}

impl Regularized for CrossEntropy {

    fn regularize(&self, operation: &RegType, params: Vec<f64>) -> Tensor {

        let r_c: f64 = params[0];
        let r_lambda: f64 = params[1];

        let r_l1 = self.weights.abs().sum(Kind::Float) * r_lambda;
        let r_l2 = self.weights.pow(&Tensor::from(2.0)).sum(Kind::Float) * r_lambda;
       
        let regularized = match operation {

            RegType::L1 => r_c * r_l1,
            RegType::L2 => r_c * r_l2,
            RegType::Elasticnet => {
                let r_elasticnet = r_c * (r_lambda * r_l1 + (1.0 - r_lambda) * r_l2);
                Tensor::from(r_elasticnet)
            }

        };

        regularized

    }

}


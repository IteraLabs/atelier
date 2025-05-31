/// Convex Linear Models

use tch::Tensor;

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
}

impl LinearModelBuilder {
    
    pub fn new() -> Self {
        LinearModelBuilder { 
            id: None,
            weights: None 
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

    pub fn build(self) -> Result<LogisticClassifier, &'static str> {
        let id = self.id.ok_or("Missing id")?;
        let weights = self.weights.ok_or("Missing weights")?;

        Ok(LogisticClassifier { id, weights })
    }
}


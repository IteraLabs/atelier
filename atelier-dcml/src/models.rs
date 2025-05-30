use tch::Tensor;

pub trait ModelSVM {
    fn export_weights(&self) -> Vec<i32>;
}

pub trait Model {
    fn forward(&self, features: &Tensor) -> Tensor;
}

#[derive(Debug)]
pub struct LogisticRegressor {
    id: String,
    weights: Tensor,
}

impl LogisticRegressor {
    pub fn builder() -> LogisticRegressorBuilder {
        LogisticRegressorBuilder::new()
    }

    pub fn forward(&self, features: &Tensor) -> Tensor {
        let logits = features.matmul(&self.weights.unsqueeze(-1)).squeeze();
        let probs = logits.sigmoid();
        probs
    }

}

#[derive(Debug)]
pub struct LogisticRegressorBuilder {
    id: Option<String>,
    weights: Option<Tensor>,
}

impl LogisticRegressorBuilder {
    pub fn new() -> Self {
        LogisticRegressorBuilder { 
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

    pub fn build(self) -> Result<LogisticRegressor, &'static str> {
        let id = self.id.ok_or("Missing id")?;
        let weights = self.weights.ok_or("Missing weights")?;

        Ok(LogisticRegressor { id, weights })
    }
}


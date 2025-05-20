use tch::Tensor;

#[derive(Debug)]
pub struct LogisticRegressorBuilder {
    weights: Option<Tensor>,
}

impl LogisticRegressorBuilder {
    pub fn new() -> Self {
        LogisticRegressorBuilder { weights: None }
    }

    pub fn weights(mut self, weights: Tensor) -> Self {
        self.weights = Some(weights);
        self
    }

    pub fn build(self) -> Result<LogisticRegressor, &'static str> {
        let weights = self.weights.ok_or("Missing weights")?;

        Ok(LogisticRegressor { weights })
    }
}

#[derive(Debug)]
pub struct LogisticRegressor {
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

    //pub fn forecast(&self, features: &Tensor, threshold: &f32) -> Tensor {
    //   let fwd = self.forward(features);
    //
    //}
}

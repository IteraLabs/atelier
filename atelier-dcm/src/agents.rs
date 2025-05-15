use tch::{Device, Kind, Tensor};

#[derive(Debug)]
pub struct AgetMetrics {
    pub loss: Tensor,
}

pub struct DistributedAgent {
    
    // Model parameters (weights)
    pub theta: Tensor,
    
    // Regularization parameters
    pub lambda1: f64,
    pub lambda2: f64,
    
    // Learning rate
    pub eta: f64,
    
    // Agent's data (features + labels)
    pub features: Tensor,
    pub labels: Tensor,
    
    // Learning metrics
    pub loss: Tensor,
}

impl DistributedAgent {
    pub fn new(
        features: Tensor,
        labels: Tensor,
        lambda1: f64,
        lambda2: f64,
        eta: f64,
        loss: Tensor,
    ) -> Self {
        let n_features = features.size()[1];

        Self {
            theta: Tensor::zeros(&[n_features], (Kind::Float, Device::Cpu)),
            lambda1,
            lambda2,
            eta,
            features,
            labels,
            loss,
        }
    }

    pub fn forward(&self, features: &Tensor) -> Tensor {
        let logits = features.matmul(&self.theta.unsqueeze(-1)).squeeze();
        let preds = logits.sigmoid();
        preds
    }

    pub fn compute_gradient(&self) -> Tensor {
        // Logistic loss gradient
        let logits = self.features.matmul(&self.theta.unsqueeze(-1)).squeeze();
        let preds = logits.sigmoid();
        let error = &preds - &self.labels.squeeze();

        // X^T * (preds - y) / n_samples
        let grad_loss = self
            .features
            .transpose(0, 1)
            .matmul(&error.unsqueeze(-1))
            .squeeze()
            / self.features.size()[0] as f64;

        // Elastic net regularization
        let grad_l1 = self.theta.sign() * self.lambda1;
        let grad_l2 = &self.theta * (2.0 * self.lambda2);

        grad_loss + grad_l1 + grad_l2
    }

    pub fn compute_loss(&self) -> Tensor {
        // Logistic loss gradient
        let logits = self.features.matmul(&self.theta.unsqueeze(-1)).squeeze();
        let preds = logits.sigmoid();
        let error = &preds - &self.labels.squeeze();
        let form = error.pow(&Tensor::from(2.0)).mean(Kind::Float);
        form
    }
}

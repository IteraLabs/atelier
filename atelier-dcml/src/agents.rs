use tch::{self, Device, Kind, Tensor};
use crate::functions;
use crate::functions::Regularized;

// ----------------------------------------------------------------------------------- //
// ----------------------------------------------------------------------------------- //

pub struct DistributedAgent {
    // Model parameters (weights)
    pub weights: Tensor,

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
    pub accuracy: Tensor,
}

impl DistributedAgent {
    pub fn new(
        features: Tensor,
        labels: Tensor,
        lambda1: f64,
        lambda2: f64,
        eta: f64,
        loss: Tensor,
        accuracy: Tensor,
    ) -> Self {
        let n_features = features.size()[1];

        Self {
            weights: Tensor::zeros(&[n_features], (Kind::Float, Device::Cpu)),
            lambda1,
            lambda2,
            eta,
            features,
            labels,
            loss,
            accuracy,
        }
    }

    pub fn forward(&self, features: &Tensor) -> Tensor {
        let logits = features.matmul(&self.weights.unsqueeze(-1)).squeeze();
        let preds = logits.sigmoid();
        preds
    }

    pub fn compute_gradient(&self) -> Tensor {
        // Logistic loss gradient
        let logits = self.features.matmul(&self.weights.unsqueeze(-1)).squeeze();
        let preds = logits.sigmoid();
        let error = &preds - &self.labels.squeeze();

        // error = y_hat - y
        // (X^T * error) / n_samples
        let grad_loss = self
            .features
            .transpose(0, 1)
            .matmul(&error.unsqueeze(-1))
            .squeeze()
            / self.features.size()[0] as f64;

        // Elastic net regularization
        let grad_l1 = self.weights.abs().sum(Kind::Float) * self.lambda1;

        let grad_l2 = self.weights.pow(&Tensor::from(2.0)).sum(Kind::Float) * self.lambda2;

        grad_loss + grad_l1 + grad_l2
    }
    
    pub fn compute_bce(&self) -> Tensor {
    
        let y_hat = self.forward(&self.features);

        let bce = functions::CrossEntropy::builder()
            .weights(&self.weights)
            .y(&self.labels)
            .y_hat(&y_hat)
            .epsilon(1e-4)
            .build()
            .expect("Failed new BCE creation");

        let r_bce = bce.regularize(&functions::RegType::Elasticnet, vec![1.1, 0.4]) ;
        r_bce

    }

    pub fn compute_loss(&self) -> Tensor {
        let logits = self.features.matmul(&self.weights.unsqueeze(-1)).squeeze();
        let preds = logits.sigmoid();

        // Manual binary cross-entropy calculation
        // -[y * log(p) + (1-y) * log(1-p)]
        let labels: tch::Tensor = self.labels.squeeze();
        let epsilon: f64 = 1e-7; // Small constant to avoid log(0)

        let p_safe: tch::Tensor = preds.clamp(epsilon, 1.0 - epsilon);
        let loss_1 = &labels * p_safe.log();
        let loss_2_a = Tensor::from(1.0) - &labels;
        let loss_2_b = (Tensor::from(1.0) - p_safe).log();

        let loss: tch::Tensor = -(loss_1 + (loss_2_a) * (loss_2_b));
        let bce_loss = loss.mean(Kind::Float); // Average over all samples

        // Add regularization
        let l1_reg = self.weights.abs().sum(Kind::Float) * self.lambda1;
        let l2_reg = self.weights.pow(&Tensor::from(2.0)).sum(Kind::Float) * self.lambda2;

        bce_loss + l1_reg + l2_reg
    }

    pub fn compute_accuracy(&self, p_threshold: f64) -> Tensor {
        let logits = self.features.matmul(&self.weights.unsqueeze(-1)).squeeze();
        let preds = logits.sigmoid();

        // Apply threshold to get binary predictions (0 or 1)
        let preds_binary = preds.ge(p_threshold).to_kind(tch::Kind::Int64);

        // Get true labels as int64
        let labels_int = self.labels.squeeze().to_kind(tch::Kind::Int64);

        // Calculate confusion matrix components using tensor operations

        // True Positives: prediction=1, actual=1
        let tp = (&preds_binary * &labels_int).sum(Kind::Float);

        // False Positives: prediction=1, actual=0
        let fp = (&preds_binary * (Tensor::from(1.0) - &labels_int)).sum(Kind::Float);

        // False Negatives: prediction=0, actual=1
        let fn_ = ((Tensor::from(1.0) - &preds_binary) * &labels_int).sum(Kind::Float);

        // True Negatives: prediction=0, actual=0
        let tn = ((Tensor::from(1.0) - &preds_binary)
            * (Tensor::from(1.0) - &labels_int))
            .sum(Kind::Float);

        // Calculate accuracy as (TP + TN) / total samples
        let total = &tp + &fp + &fn_ + &tn;
        let accuracy = (&tp + &tn) / total;

        accuracy
    }
}

/// Metrics

use tch::{Tensor, Kind};

pub fn compute_accuracy(y: Tensor, y_hat: Tensor, p_threshold: f64) -> Tensor {


    // Apply threshold to get binary predictions (0 or 1)
    let preds_binary = y.ge(p_threshold).to_kind(tch::Kind::Int64);

    // Get true labels as int64
    let labels_int = y_hat.squeeze().to_kind(tch::Kind::Int64);

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

// ROC
// AUC
// F1
// Recall
// Confusion_Matrix


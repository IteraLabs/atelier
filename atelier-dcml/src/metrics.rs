
use tch::{Tensor, Kind};
use std::collections::HashMap;


// Metric Trait for all metrics
pub trait Metric {
    fn id(&self) -> &str;
    fn metric_type(&self) -> MetricType;
    fn compute(&self, y: &Tensor, y_pred: &Tensor, threshold: Option<f64>) -> MetricValue;
    fn update(&mut self, value: MetricValue);
    fn latest(&self) -> Option<&MetricValue>;
    fn history(&self) -> &Vec<MetricValue>;
    fn reset(&mut self);
}

#[derive(Debug, Clone)]
pub enum MetricType {
    Numerical,
    Categorical,
    Matrix,
}

#[derive(Debug, Clone)]
pub enum MetricValue {
    Scalar(f64),
    Matrix(Vec<Vec<f64>>),
    Multiple(HashMap<String, f64>),
}

impl MetricValue {

    pub fn as_scalar(&self) -> Option<f64> {
        match self {
            MetricValue::Scalar(val) => Some(*val),
            _ => None,
        }
    }

    pub fn as_matrix(&self) -> Option<&Vec<Vec<f64>>> {
        match self {
            MetricValue::Matrix(mat) => Some(mat),
            _ => None,
        }
    }

    pub fn as_multiple(&self) -> Option<&HashMap<String, f64>> {
        match self {
            MetricValue::Multiple(mul) => Some(mul),
            _ => None,
        }
    }
}

// Helper struct for confusion matrix calculations
#[derive(Debug, Clone)]
pub struct ConfusionMatrixComponents {
    pub true_positive: f64,
    pub false_positive: f64,
    pub false_negative: f64,
    pub true_negative: f64,
}

impl ConfusionMatrixComponents {

    pub fn from_tensors(y: &Tensor, y_pred: &Tensor, threshold: Option<f64>) -> Self {
        
        // Apply threshold to get binary predictions
        let threshold: f64 = threshold.unwrap_or(0.5);
        let preds_binary: Tensor = y_pred.ge(threshold).to_kind(Kind::Float);
        let labels: Tensor = y.to_kind(Kind::Float);

        // Calculate confusion matrix components
        let tp = (&preds_binary * &labels).sum(Kind::Float);
        let fp = (&preds_binary * (Tensor::from(1.0) - &labels)).sum(Kind::Float);
        let fn_ = ((Tensor::from(1.0) - &preds_binary) * &labels).sum(Kind::Float);
        let tn = ((Tensor::from(1.0) - &preds_binary) * (Tensor::from(1.0) - &labels))
            .sum(Kind::Float);

        ConfusionMatrixComponents {
            true_positive: f64::from(tp.double_value(&[])),
            false_positive: f64::from(fp.double_value(&[])),
            false_negative: f64::from(fn_.double_value(&[])),
            true_negative: f64::from(tn.double_value(&[])),
        }
    }

    pub fn total(&self) -> f64 {
        self.true_positive + self.false_positive + self.false_negative + self.true_negative
    }
}


// Concrete metric implementations
#[derive(Debug)]
pub struct Accuracy {
    id: String,
    values: Vec<MetricValue>,
}

impl Accuracy {
    pub fn new() -> Self {
        Accuracy {
            id: "accuracy".to_string(),
            values: Vec::new(),
        }
    }
}

impl Metric for Accuracy {

    fn id(&self) -> &str { &self.id }
    fn metric_type(&self) -> MetricType { MetricType::Numerical }

    fn compute(&self, y: &Tensor, y_pred: &Tensor, thr: Option<f64>) -> MetricValue {
        let threshold = thr.unwrap_or(0.5);
        let cm = ConfusionMatrixComponents::from_tensors(y, y_pred, Some(threshold));
        let accuracy = (cm.true_positive + cm.true_negative) / cm.total();
        MetricValue::Scalar(accuracy)
    }

    fn update(&mut self, value: MetricValue) {
        self.values.push(value);
    }

    fn latest(&self) -> Option<&MetricValue> {
        self.values.last()
    }

    fn history(&self) -> &Vec<MetricValue> {
        &self.values
    }

    fn reset(&mut self) {
        self.values.clear();
    }
}


#[derive(Debug)]
pub struct ConfusionMatrix {
    id: String,
    values: Vec<MetricValue>,
}

impl ConfusionMatrix {
    pub fn new() -> Self {
        ConfusionMatrix {
            id: "confusion_matrix".to_string(),
            values: Vec::new(),
        }
    }
}

impl Metric for ConfusionMatrix {
    fn id(&self) -> &str {
        &self.id
    }

    fn metric_type(&self) -> MetricType {
        MetricType::Matrix
    }

    fn compute(&self, y: &Tensor, y_pred: &Tensor, threshold: Option<f64>) -> MetricValue {
        let threshold = threshold.unwrap_or(0.5);
        let cm = ConfusionMatrixComponents::from_tensors(y, y_pred, Some(threshold));
        
        // Return as 2x2 matrix: [[TN, FP], [FN, TP]]
        let matrix = vec![
            vec![cm.true_negative, cm.false_positive],
            vec![cm.false_negative, cm.true_positive],
        ];
        
        MetricValue::Matrix(matrix)
    }

    fn update(&mut self, value: MetricValue) {
        self.values.push(value);
    }

    fn latest(&self) -> Option<&MetricValue> {
        self.values.last()
    }

    fn history(&self) -> &Vec<MetricValue> {
        &self.values
    }

    fn reset(&mut self) {
        self.values.clear();
    }
}


#[derive(Debug)]
pub struct ComprehensiveMetrics {
    id: String,
    values: Vec<MetricValue>,
}

impl ComprehensiveMetrics {

    pub fn new() -> Self {
        ComprehensiveMetrics {
            id: "comprehensive_metrics".to_string(),
            values: Vec::new(),
        }
    }

}

impl Metric for ComprehensiveMetrics {
    fn id(&self) -> &str {
        &self.id
    }

    fn metric_type(&self) -> MetricType {
        MetricType::Matrix
    }

    fn compute(
        &self,
        y_true: &Tensor,
        y_pred: &Tensor,
        threshold: Option<f64>
    ) -> MetricValue {

        let cm = ConfusionMatrixComponents::from_tensors(y_true, y_pred, threshold);
        
        let accuracy = (cm.true_positive + cm.true_negative) / cm.total();
        let precision = if cm.true_positive + cm.false_positive > 0.0 {
            cm.true_positive / (cm.true_positive + cm.false_positive) } 
            else { 0.0 };
        let recall = if cm.true_positive + cm.false_negative > 0.0 {
            cm.true_positive / (cm.true_positive + cm.false_negative) } 
            else { 0.0 };
        let specificity = if cm.true_negative + cm.false_positive > 0.0 {
            cm.true_negative / (cm.true_negative + cm.false_positive) } 
            else { 0.0 };
        let f1 = if precision + recall > 0.0 {
            2.0 * (precision * recall) / (precision + recall) } 
            else { 0.0 };
        
        let mut results = HashMap::new();
        
        results.insert("accuracy".to_string(), accuracy);
        results.insert("precision".to_string(), precision);
        results.insert("recall".to_string(), recall);
        results.insert("specificity".to_string(), specificity);
        results.insert("f1_score".to_string(), f1);
        results.insert("tp".to_string(), cm.true_positive);
        results.insert("fp".to_string(), cm.false_positive);
        results.insert("fn".to_string(), cm.false_negative);
        results.insert("tn".to_string(), cm.true_negative);
        
        MetricValue::Multiple(results)
    }

    fn update(&mut self, value: MetricValue) {
        self.values.push(value);
    }

    fn latest(&self) -> Option<&MetricValue> {
        self.values.last()
    }

    fn history(&self) -> &Vec<MetricValue> {
        &self.values
    }

    fn reset(&mut self) {
        self.values.clear();
    }
}

// Container for multiple metrics
pub struct Metrics {
    metrics: Vec<Box<dyn Metric>>,
    threshold: f64,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            metrics: Vec::new(),
            threshold: 0.5,
        }
    }

    pub fn with_threshold(threshold: f64) -> Self {
        Metrics {
            metrics: Vec::new(),
            threshold,
        }
    }

    pub fn add_metric(&mut self, metric: Box<dyn Metric>) {
        self.metrics.push(metric);
    }

    pub fn set_threshold(&mut self, threshold: f64) {
        self.threshold = threshold;
    }

    pub fn compute_all(
        &mut self,
        y_true: &Tensor,
        y_pred: &Tensor
    ) -> HashMap<String, MetricValue> {
   
        let mut results = HashMap::new();
        
        for metric in self.metrics.iter_mut() {
            let value = metric.compute(y_true, y_pred, Some(self.threshold));
            metric.update(value.clone());
            results.insert(metric.id().to_string(), value);
        }
        
        results
    }

    pub fn get_latest(&self, metric_id: &str) -> Option<&MetricValue> {
        self.metrics
            .iter()
            .find(|m| m.id() == metric_id)
            .and_then(|m| m.latest())
    }

    pub fn get_history(&self, metric_id: &str) -> Option<&Vec<MetricValue>> {
        self.metrics
            .iter()
            .find(|m| m.id() == metric_id)
            .map(|m| m.history())
    }

    pub fn reset_all(&mut self) {
        for metric in self.metrics.iter_mut() {
            metric.reset();
        }
    }

    pub fn list_metrics(&self) -> Vec<&str> {
        self.metrics.iter().map(|m| m.id()).collect()
    }
}

// Convenience constructors
impl Metrics {
    pub fn classification_suite() -> Self {
        let mut metrics = Metrics::new();
        metrics.add_metric(Box::new(Accuracy::new()));
        //metrics.add_metric(Box::new(F1Score::new()));
        //metrics.add_metric(Box::new(Recall::new()));
        metrics.add_metric(Box::new(ConfusionMatrix::new()));
        metrics.add_metric(Box::new(ComprehensiveMetrics::new()));
        metrics
    }

    pub fn basic_classification() -> Self {
        let mut metrics = Metrics::new();
        metrics.add_metric(Box::new(Accuracy::new()));
        //metrics.add_metric(Box::new(F1Score::new()));
        metrics
    }
}

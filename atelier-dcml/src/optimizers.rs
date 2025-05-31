/// Optmizers and Learning Algorithms

pub trait Optimizer {
    fn step(&mut self, params: &mut Vec<f64>, gradients: &[f64]);
    fn reset(&mut self);
}

pub struct GradientDescent {
    learning_rate: f64,
}

impl GradientDescent {
    pub fn builder() -> OptimizerBuilder {
        OptimizerBuilder::new()
    }
}

impl Optimizer for GradientDescent {

    // GD iteration
    fn step(&mut self, params: &mut Vec<f64>, gradients: &[f64]) {
        for (param, grad) in params.iter_mut().zip(gradients.iter()) {
            *param -= self.learning_rate * grad;
        }
    }
    // GD has no state to reset
    fn reset(&mut self) { }

}

pub struct OptimizerBuilder {
    learning_rate: Option<f64>,
}

impl OptimizerBuilder {

    pub fn new() -> Self {
        OptimizerBuilder {
            learning_rate: None
        }
    }

    pub fn learning_rate(mut self, learning_rate: f64) -> Self {
        self.learning_rate = Some(learning_rate);
        self
    }

    pub fn build(self) -> Result<GradientDescent, &'static str> {
        let learning_rate = self.learning_rate.ok_or("Missing Learning Rate")?;
        Ok(GradientDescent { learning_rate })
    }
}


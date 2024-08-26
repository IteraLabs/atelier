use rand::prelude::*;
use rand_distr::StandardNormal;

pub fn pdf_standard_normal() -> f64 {
    let val: f64 = thread_rng().sample(StandardNormal);
    val
}

pub trait Distribution {
    fn sample(&self, n: usize) -> Vec<f64>;
}

pub struct Poisson {
    pub lambda: f64,
}

impl Distribution for Poisson {
    fn sample(&self, n: usize) -> Vec<f64> {
        let mut samples = Vec::new();

        for _ in 0..n {
            // Inverse Method for Random Sampling
            let mut x = 0;
            let mut p = 1.0;
            loop {
                let u = rand::random::<f64>();
                p *= u;
                if p < (p.exp() * -self.lambda).exp() {
                    break;
                }
                x += 1;
            }

            samples.push(x as f64);
        }
        samples
    }
}

pub struct Exponential {
    pub lambda: f64,
}

impl Distribution for Exponential {
    fn sample(&self, n: usize) -> Vec<f64> {
        let mut samples = Vec::new();

        // Inverse Method for Random Sampling
        for _ in 0..n {
            let x = (-1.0 / self.lambda) * (rand::random::<f64>().ln()) as f64;
            samples.push(x);
        }
        samples
    }
}

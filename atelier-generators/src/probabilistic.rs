//! # Probabilistic generators
//!
//! This module provides implementations for sampling from various probability
//! distributions, including:
//!
//! - Normal
//! - Poisson
//! - Exponential
//!
//! ## References
//!
//! - [rand_distr](https://docs.rs/rand_distr/latest/rand_distr/)
use rand::prelude::*;
use rand::distr::Uniform;
use rand_distr::Normal;

pub enum Distributions {
    Uniform,
    Normal,
    Poisson,
    Exponential
}

pub trait Sampling {
    fn sample(&self, n: usize) -> Vec<f64>;
}

pub struct UniformDistribution {
    pub lower: f64,
    pub upper: f64,
}

pub fn uniform_return(
    lower: f64,
    upper: f64,
    n: usize,
    ) -> Vec<f64> {

    let uniform = UniformDistribution { lower, upper };
    let returns = uniform.sample(n);
    returns
}

impl Sampling for UniformDistribution {

    fn sample(&self, n: usize) -> Vec<f64> {
        let mut rng = rand::rng();
        let uni = Uniform::new(self.lower, self.upper).unwrap();
        (0..n).map(|_| rng.sample(uni)).collect()
    }

}

pub struct NormalDistribution {
    pub mu: f64,
    pub sigma: f64,
}

impl Sampling for NormalDistribution {
    fn sample(&self, n: usize) -> Vec<f64> {
        if self.mu == 0.0 && self.sigma == 1.0 {
            let std_normal = Normal::new(0.0, 1.0).unwrap();
            let v_std_normal: Vec<f64> =
                std_normal.sample_iter(&mut rand::rng()).take(n).collect();
            v_std_normal
        } else {
            let normal = Normal::new(0.0, 1.0).unwrap();
            let v_normal: Vec<f64> =
                normal.sample_iter(&mut rand::rng()).take(n).collect();
            v_normal
        }
    }
}

pub struct Poisson {
    pub lambda: f64,
}

impl Sampling for Poisson {
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

impl Sampling for Exponential {
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

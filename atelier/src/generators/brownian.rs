//! # Brownian Motion
//!
//! ## Geometric
//!
//! The dynamics of GBM can be described by the following
//! Stochastic Differential Equation:
//!
//! $$
//!  dS_{t} = \mu S_{t} dt + \sigma S_{t} dW_{t}
//! $$
//!
//! Where:
//!
//! $S_{t}$: The price of the asset at time $t$. \
//! $\mu$: The drift coefficient, representing the expected return. \
//! $\sigma$: The diffusion coefficient, representing the degree of variation in returns. \
//! $dW_{t}: A $Wiener$ process as the source of randomness within the model.
//!
//! ## Implementation
//! $\mu$: Is sometimes used to represent a $trend$ in the prices. \
//! $\sigma$: Is sometimes used to represent the $volatility$ of the asset's return. \
//! $dS_{t}$: is a non-negative, log-normally distributed, i.i.d process. Frequently
//! used for Pricing Options (Black-Scholes), Value-at-risk (VaR) calculations, dummy
//! examples of prices progression.
//!
//! ## References
//! [Brownian motion](https://en.wikipedia.org/wiki/Brownian_motion)
use crate::generators::probabilistic;
use crate::messages::errors;

pub fn gbm_return(
    s0: f64,
    mu: f64,
    sigma: f64,
    t: f64,
) -> core::result::Result<f64, errors::GeneratorError> {
    let dwt = probabilistic::pdf_standard_normal();
    let drift = mu * s0 * t;
    let diffusion = sigma * s0 * dwt;
    let dst = drift + diffusion;
    Ok(dst)
}

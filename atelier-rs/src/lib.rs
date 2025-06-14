//! atelier-rs
//!
//! Is a computational Framework/Engine for Market Microstructure High Frequency
//! Modeling. Provides the capacity to perform Synthetic Simulations, and/or,
//! Historical Market Reconstruction/Replays.
//!

/// Core data structures and I/O tools.
pub use atelier_data::*;

/// Distributed Convex Machine Learning.
pub use atelier_dcml::*;

/// Probabilistic generators and events simulation.
pub use atelier_generators::*;

/// Standardized results, errors and successes.
pub use atelier_results::*;

/// Synthetic Data Generation.
pub use atelier_synth::*;

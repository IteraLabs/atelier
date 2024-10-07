//! # Atelier
//! Provides a computational framework that can be used as a workshop to develop Financial Machine Learning models, especifically battle tested to the level of Market microstructure.
//! Features include:
//! * **Market Simulation**: A synthetic simulation of a market, as detailed as the provided configuration.
//! * **Market Replay**: A high-fidelity reproduction of what actually happened, all the way down to the data granularity that is provided.

#![allow(dead_code)]

/// Data structures and core datatypes.
pub mod data;

/// Data Generation from parametric probability distributions.
pub mod generators;

/// Calculations and metrics about performance of many kinds.
pub mod metrics;

/// Messages structures and functionalities for: Errors, Events, Logs.
pub mod messages;

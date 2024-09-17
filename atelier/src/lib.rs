//! # Atelier
//! Provides a computational framework that can be used as a workshop to develop Financial Machine Learning models, especifically battle tested to the level of Market microstructure.
//! Features include:
//! * **Market Simulation**: A synthetic simulation of a market, as detailed as the provided configuration.
//! * **Market Replay**: A high-fidelity reproduction of what actually happened, all the way down to the data granularity that is provided.

#![allow(dead_code)]

/// Definition of data structures and core datatypes
pub mod data;

/// Tools to create generators of data from specification
/// of parametric probability distributions.
pub mod simulation;

/// Calculations and metrics useful to test performance of many kinds.
pub mod metrics;

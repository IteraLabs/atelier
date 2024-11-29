//! # Atelier
//! Provides a computational framework that can be used as a workshop to develop Financial Machine Learning models, especifically battle tested to the level of Market microstructure.
//! Features include:
//! * **Market Simulation**: A synthetic simulation of a market, as detailed as the provided configuration.
//! * **Market Replay**: A high-fidelity reproduction of what actually happened, all the way down to the data granularity that is provided.

#![allow(dead_code)]
#![allow(warnings)]

/// Core data types and data structures.
pub mod data;

/// The Order Management and Matching Engine logics.
pub mod engine;

/// Market event generator.
pub mod events;

/// Stochastic data generation with probabilistic models.
pub mod generators;

/// Messages structures for: Errors, Success, Events, and Logs.
pub mod messages;

/// Metrics calculation about various aspects and processes.
pub mod metrics;


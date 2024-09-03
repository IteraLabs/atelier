//! # Atelier
//! [`Atelier`] provides a computational framework that can be used as a workshop to develop Financial Machine Learning models, especifically battle tested to the level of Market microstructure.
//! Features include:
//! * **Market Replay**: A high-fidelity reproduction of what actually happened, all the way down to the data granularity that is provided.
//! * **Market Simulation**: A synthetic simulation of a market, as detailed as the provided configuration.
//!
//! ## Example: Synthetic Orderbook
//! ```
//! use atelier::data::market::Orderbook;
//!
//! let bid_price = 50_000.00;
//! let ask_price = 50_100.00;
//! let tick_size = 100.0;
//! let n_levels = 200;
//! let n_orders = 300;
//!
//! let i_ob = Orderbook::synthetize(bid_price, ask_price,
//!        tick_size, n_levels, n_orders);
//! ```

#![allow(dead_code)]

/// Definition of data structures and core datatypes
pub mod data;

/// Tools to create generators of data from specification
/// of parametric probability distributions.
pub mod simulation;

/// Calculations and metrics useful to test performance of many kinds.
pub mod metrics;

//! atelier-data
//!
//! Core data structures and I/O tools for the atelier-rs engine.
//!

/// Configurations and experiments
pub mod templates;

/// Dataset defintion and tools
pub mod data;

/// Implementation of orders
pub mod orders;

/// Orders-Price-Volume levels for Orderbooks.
pub mod levels;

/// Single thread Orderbook structure.
pub mod orderbooks;

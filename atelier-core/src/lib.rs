//! # atelier-core
//!
//! Data structure definition.

#![allow(dead_code)]

/// Configurations and experiments
pub mod templates;

/// Data Input/Output operations
pub mod data;

/// Implementation of orders: Market, Limit
pub mod orders;

/// Price-Volume levels for Orderbooks.
pub mod levels;

/// Single thread Orderbook structure.
pub mod orderbooks;

/// Results messages: Errors, and, Success.
pub mod results;

/// Training environment
pub mod training;

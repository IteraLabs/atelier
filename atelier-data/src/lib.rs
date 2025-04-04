//! # atelier-data
//!
//! Data structure definition.

/// Implementation of orders: Market, Limit
pub mod orders;

/// Price-Volume levels for Orderbooks.
pub mod levels;

/// Single thread Orderbook structure.
pub mod orderbooks;

/// Results messages: Errors, and, Success.
pub mod results;

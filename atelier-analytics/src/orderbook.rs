//! Orderbook metrics

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MetricResult {
    Value(f64),
    Values(Vec<f64>),
}

/// A trait for computing market metrics based on Bids and Asks levels
/// at a specified depth.
///
/// Any implementation of this trait does not consume the inputs.
///
/// # Parameters
/// - `levels`: A generic collection of levels [(price, amount), (price, amount)]
/// - `depth`: The depth level from which to compute the metric.
///
/// # Returns
/// Returns the computed market metric as a `f64`.

pub trait OrderBookMetric<V> {
    fn compute(levels: &V, depth: usize) -> MetricResult;
}

// ----------------------------------------------------------------------------------- //
// ----------------------------------------------------------------------------------- //
/// The Spread as a market metric.

pub struct Spread;
impl OrderBookMetric<Vec<f64>> for Spread {
    /// Compute the Spread from the given bids and asks.
    ///
    /// # Parameters
    /// - `levels_prices` : A Vec<f64> with the Bid and Ask price.
    ///
    /// # Returns
    /// Returns the MetricResult::Value(f64)
    ///
    fn compute(levels_prices: &Vec<f64>, _depth: usize) -> MetricResult {
        MetricResult::Value(levels_prices[1] - levels_prices[0] as f64)
    }
}

// ----------------------------------------------------------------------------------- //
// ----------------------------------------------------------------------------------- //
/// The midprice as a market metric.

pub struct Midprice;
impl OrderBookMetric<Vec<f64>> for Midprice {
    /// Computes the Midprice from the given bids and asks at the specified depth.
    ///
    /// # Parameters
    /// - `levels_prices` : A Vec<f64> with the Bid and Ask price.
    ///
    /// # Returns
    /// Returns the MetricResult::Value(f64)
    ///
    fn compute(levels_prices: &Vec<f64>, _depth: usize) -> MetricResult {
        MetricResult::Value((levels_prices[1] + levels_prices[0]) / 2.0 as f64)
    }
}


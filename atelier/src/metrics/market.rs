use crate::data::market::Level;

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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MetricResult {
    Value(f64),
    Values(Vec<f64>),
}

pub trait OrderBookMetric<V> {
    fn compute(levels: &V, depth: usize) -> MetricResult;
}

// ----------------------------------------------------------------------------------- //
// ----------------------------------------------------------------------------------- //
/// A struct to represent the Spread as a market metric.

pub struct Spread;
impl OrderBookMetric<Vec<f64>> for Spread {
    /// Compute the Spread from the given bids and asks.
    /// # Parameters
    /// - `bids` : A f64 with the Bid price.
    /// - `asks` : A f64 with the Ask price.
    ///
    /// # Returns
    /// Returns the Spread as a `f64`.
    // fn compute(bids: &f64, asks: &f64, _depth: usize) -> f64 {
    fn compute(levels_prices: &Vec<f64>, _depth: usize) -> MetricResult {
        MetricResult::Value(levels_prices[1] - levels_prices[0] as f64)
    }
}

// ----------------------------------------------------------------------------------- //
// ----------------------------------------------------------------------------------- //
/// A struct to represent the midprice as a market metric.

pub struct Midprice;
impl OrderBookMetric<Vec<f64>> for Midprice {
    /// Computes the Midprice from the given bids and asks at the specified depth.
    /// # Parameters
    /// - `bids` : A vector of tupples where each contains the price and volume for the bids.
    /// - `asks` : A vector of tupples where each contains the price and volume for the asks.
    /// - `depth`: The index specifying up to which level of the order book to compute the midprice from.
    ///
    /// # Returns
    /// Returns the Midprice as a `f64`.
    fn compute(levels_prices: &Vec<f64>, _depth: usize) -> MetricResult {
        MetricResult::Value((levels_prices[1] + levels_prices[0]) / 2.0 as f64)
    }
}

// ----------------------------------------------------------------------------------- //
// ----------------------------------------------------------------------------------- //
/// A struct to represent the Volume-Weighted Average Price as a market metric.

pub struct VWAP;
impl OrderBookMetric<(Vec<Vec<f64>>, Vec<Vec<f64>>)> for VWAP {
    /// Computes the Volume-Weighted Average Price (VWAP) from the given
    /// bids and asks.
    /// # Parameters
    /// - `bids`: A vector of tupples, where each contains the price and volume for the bids.
    /// - `asks`: A vector of tupples, where each contains the price and volume for the asks.
    /// - `depth`: The index specifying up to which level of the orderbook to compute the VWAP.
    ///
    /// # Returns
    /// Returns the VWAP as a `f64`.

    // fn compute(bids: &Vec<Vec<f64>>, asks: &Vec<Vec<f64>>, depth: usize) -> MetricResult {
    fn compute(levels: &(Vec<Vec<f64>>, Vec<Vec<f64>>), depth: usize) -> MetricResult {
        let mut bids_mult: Vec<f64> = vec![];
        let mut asks_mult: Vec<f64> = vec![];
        let mut vol_sum: Vec<f64> = vec![];

        for i_level in 0..=depth {
            bids_mult.push(levels.0[i_level][0] * levels.0[i_level][1]);

            asks_mult.push(levels.1[i_level][0] * levels.1[i_level][1]);

            vol_sum.push(levels.0[i_level][1] + levels.1[i_level][1]);
        }

        let r_vwap_1: f64 = bids_mult.iter().sum();
        let r_vwap_2: f64 = asks_mult.iter().sum();
        let r_vwap_3: f64 = vol_sum.iter().sum();

        MetricResult::Value((r_vwap_1 + r_vwap_2) / r_vwap_3 as f64)
    }
}

// ----------------------------------------------------------------------------------- //
// ----------------------------------------------------------------------------------- //
/// A struct to represent the OrderBook Volume Imbalance as a market metric.

pub struct VolumeImbalance;
impl OrderBookMetric<Vec<Vec<f64>>> for VolumeImbalance {
    /// Computes the Order book Volume Imbalance from the given bids and asks.
    ///
    /// # Parameters
    /// - `bids`: A vector of values, where each contains only the volume for the bids levels.
    /// - `asks`: A vector of values, where each contains only the volume for the asks levels.
    /// - `depth`: The index specifying up to which level of the orderbook to compute the Volume Imbalance.
    ///
    /// # Returns
    /// Returns the Volume Imbalance as a `f64`.

    fn compute(levels: &Vec<Vec<f64>>, depth: usize) -> MetricResult {
        let mut bids_mult: Vec<f64> = vec![];
        let mut asks_mult: Vec<f64> = vec![];

        for i_level in 0..=depth {
            bids_mult.push(levels[0][i_level]);
            asks_mult.push(levels[1][i_level]);
        }

        let r_obimb_1: f64 = bids_mult.iter().sum();
        let r_obimb_2: f64 = asks_mult.iter().sum();

        MetricResult::Value((r_obimb_1 - r_obimb_2) / (r_obimb_1 + r_obimb_2))
    }
}

// ----------------------------------------------------------------------------------- //
// ----------------------------------------------------------------------------------- //
/// A struct to represent the TickSize as the difference between price among levels

pub struct TickSize;
impl OrderBookMetric<Vec<f64>> for TickSize {
    /// Compute all the ticks in the provided Order Book, calculated as the
    /// difference between two adjacent price levels, for all levels, for boths sides.
    ///
    /// # Parameters
    /// - `levels`: A vector with all the Levels, agnostic of their side
    ///
    /// # Returns
    /// A vector with the difference among all pair of levels.

    fn compute(levels_prices: &Vec<f64>, depth: usize) -> MetricResult {
        let mut v_ticks = vec![];

        for i_price in 0..levels_prices.len() - 1 {
            v_ticks.push(levels_prices[i_price] - levels_prices[i_price + 1])
        }
        MetricResult::Values(v_ticks)
    }
}

// ----------------------------------------------------------------------------------- //
// ----------------------------------------------------------------------------------- //
/// A struct to represent the Amount of Orders from TOB up to a Depth of the Orderbook

pub struct OrdersAmount;
impl OrderBookMetric<Vec<Level>> for OrdersAmount {
    /// Compute the amount of orders present in all the Levels, up to the
    /// defined depth
    ///
    /// # Parameters
    /// - `levels`: A Vector of Level objects
    ///
    /// # Returns
    /// MetricResult with a single Value.
    ///

    fn compute(levels: &Vec<Level>, depth: usize) -> MetricResult {
        let mut v_orders = vec![];
        for i_level in 0..depth {
            v_orders.push(levels[i_level].orders.len() as f64);
        }
        MetricResult::Value(v_orders.into_iter().sum())
    }
}

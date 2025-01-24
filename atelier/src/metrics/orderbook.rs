use crate::data::market::Level;

/// An enum to register the outputs for Metrics

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

// ----------------------------------------------------------------------------------- //
// ----------------------------------------------------------------------------------- //
/// The Volume-Weighted Average Price as a market metric.

pub struct VWAP;
impl OrderBookMetric<(&Vec<Level>, &Vec<Level>)> for VWAP {
    /// Computes the Volume-Weighted Average Price (VWAP) from the given
    /// bids and asks.
    ///
    /// # Parameters
    /// - `levels`: A tupple with the vectors of bids and asks.
    /// - `depth`: The number of levels to compute results.
    ///
    /// # Returns
    /// Returns the MetricResult::Value(f64)
    ///

    fn compute(levels: &(&Vec<Level>, &Vec<Level>), depth: usize) -> MetricResult {
        let mut bids_mult: Vec<f64> = vec![];
        let mut asks_mult: Vec<f64> = vec![];
        let mut vol_sum: Vec<f64> = vec![];

        for i_level in 0..=depth - 1 {
            bids_mult.push(levels.0[i_level].price * levels.0[i_level].volume);
            asks_mult.push(levels.1[i_level].price * levels.1[i_level].volume);
            vol_sum.push(levels.0[i_level].volume + levels.1[i_level].volume);
        }

        let r_vwap_1: f64 = bids_mult.iter().sum();
        let r_vwap_2: f64 = asks_mult.iter().sum();
        let r_vwap_3: f64 = vol_sum.iter().sum();

        MetricResult::Value((r_vwap_1 + r_vwap_2) / r_vwap_3 as f64)
    }
}

// ----------------------------------------------------------------------------------- //
// ----------------------------------------------------------------------------------- //
/// OrderBook Volume Imbalance as a market metric.

pub struct VolumeImbalance;
impl OrderBookMetric<(&Vec<Level>, &Vec<Level>)> for VolumeImbalance {
    /// Computes the Order book Volume Imbalance from the given bids and asks.
    ///
    /// # Parameters
    /// - `levels`: A tupple with the vectors of bids and asks.
    /// - `depth`: The number of levels to compute results.
    ///
    /// # Returns
    /// Returns the MetricResult::Value(f64)
    ///

    fn compute(levels: &(&Vec<Level>, &Vec<Level>), depth: usize) -> MetricResult {
        let mut bids_mult: Vec<f64> = vec![];
        let mut asks_mult: Vec<f64> = vec![];

        for i_level in 0..=depth - 1 {
            bids_mult.push(levels.0[i_level].volume);
            asks_mult.push(levels.1[i_level].volume);
        }

        let r_obimb_1: f64 = bids_mult.iter().sum();
        let r_obimb_2: f64 = asks_mult.iter().sum();

        MetricResult::Value((r_obimb_1 - r_obimb_2) / (r_obimb_1 + r_obimb_2))
    }
}

// ----------------------------------------------------------------------------------- //
// ----------------------------------------------------------------------------------- //
/// Orderbook ticksize as the difference between price among levels

pub struct TickSize;
impl OrderBookMetric<Vec<Level>> for TickSize {
    /// Compute all the ticks in the provided Order Book, calculated as the
    /// difference between two adjacent price levels, for all levels, for boths sides.
    ///
    /// # Parameters
    /// - `levels`: A vector with all the Levels, agnostic of their side
    /// - `depth`: The number of levels to compute results.
    ///
    /// # Returns
    /// Returns the MetricResult::Values(f64)
    ///

    fn compute(levels: &Vec<Level>, depth: usize) -> MetricResult {
        let mut v_ticks = vec![];

        for i_level in 0..depth - 1 {
            v_ticks.push(levels[i_level].price - levels[i_level + 1].price)
        }

        MetricResult::Values(v_ticks)
    }
}

// ----------------------------------------------------------------------------------- //
// ----------------------------------------------------------------------------------- //
/// The Amount of Orders from TOB up to a Depth of the Orderbook

pub struct OrdersAmount;
impl OrderBookMetric<Vec<Level>> for OrdersAmount {
    /// Compute the amount of orders present in all the Levels, up to the
    /// defined depth
    ///
    /// # Parameters
    /// - `levels`: A Vector of Level objects
    /// - `depth`: The number of levels to compute results.
    ///
    /// # Returns
    /// Returns the MetricResult::Value(f64)
    ///

    fn compute(levels: &Vec<Level>, depth: usize) -> MetricResult {
        let mut v_orders = vec![];
        for i_level in 0..depth {
            v_orders.push(levels[i_level].orders.len() as f64);
        }
        MetricResult::Value(v_orders.into_iter().sum())
    }
}

// ----------------------------------------------------------------------------------- //
// ----------------------------------------------------------------------------------- //
/// Total Posted Volume from TOB up to a Depth of the Orderbook

pub struct OrdersVolume;
impl OrderBookMetric<Vec<Level>> for OrdersVolume {
    /// Compute the volume of all active orders in all levels up to the defined depth
    ///
    /// # Parameters
    /// - `levels`: A Vector of Level objects
    /// - `depth`: The number of Level objects to use for the calculation
    ///
    /// # Returns
    /// Returns the MetricResult::Values(f64)
    ///

    fn compute(levels: &Vec<Level>, depth: usize) -> MetricResult {
        let mut v_orders: Vec<f64> = vec![];

        for i_level in 0..depth {
            v_orders.push(
                levels[i_level]
                    .clone()
                    .orders
                    .into_iter()
                    .map(|i_order| i_order.amount.unwrap())
                    .sum(),
            );
        }
        MetricResult::Value(v_orders.into_iter().sum())
    }
}

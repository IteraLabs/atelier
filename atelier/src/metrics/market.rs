/// A trait for computing market metrics based on Bids and Asks levels
/// at a specified depth.
///
/// Any implementation of this trait does not consume the inputs.
///
/// # Parameters
/// - `bids`: A collection of bid prices and quantities.
/// - `asks`: A collection of ask prices and quantities.
/// - `depth`: The depth level from which to compute the metric.
///
/// # Returns
/// Returns the computed market metric as a `f64`.
pub trait MarketMetric<V> {
    fn compute(bids: &V, asks: &V, depth: usize) -> f64;
}

/// A struct to represent the Spread as a market metric.
pub struct Spread;
impl MarketMetric<f64> for Spread {
    /// Compute the Spread from the given bids and asks.
    /// # Parameters
    /// - `bids` : A f64 with the Bid price.
    /// - `asks` : A f64 with the Ask price.
    ///
    /// # Returns
    /// Returns the Spread as a `f64`.
    fn compute(bids: &f64, asks: &f64, _depth: usize) -> f64 {
        asks - bids
    }
}

/// A struct to represent the midprice as a market metric.
pub struct Midprice;
impl MarketMetric<f64> for Midprice {
    /// Computes the Midprice from the given bids and asks at the specified depth.
    /// # Parameters
    /// - `bids` : A vector of tupples where each contains the price and volume for the bids.
    /// - `asks` : A vector of tupples where each contains the price and volume for the asks.
    /// - `depth`: The index specifying up to which level of the order book to compute the midprice from.
    ///
    /// # Returns
    /// Returns the Midprice as a `f64`.
    fn compute(bids: &f64, asks: &f64, _depth: usize) -> f64 {
        (bids + asks) / 2.0
    }
}

/// A struct to represent the Volume-Weighted Average Price as a market metric.
pub struct VWAP;
impl MarketMetric<Vec<Vec<f64>>> for VWAP {
    /// Computes the Volume-Weighted Average Price (VWAP) from the given
    /// bids and asks.
    /// # Parameters
    /// - `bids`: A vector of tupples, where each contains the price and volume for the bids.
    /// - `asks`: A vector of tupples, where each contains the price and volume for the asks.
    /// - `depth`: The index specifying up to which level of the orderbook to compute the VWAP.
    ///
    /// # Returns
    /// Returns the VWAP as a `f64`.
    fn compute(
        bids: &Vec<Vec<f64>>,
        asks: &Vec<Vec<f64>>,
        depth: usize,
    ) -> f64 {
        let mut bids_mult: Vec<f64> = vec![];
        let mut asks_mult: Vec<f64> = vec![];
        let mut vol_sum: Vec<f64> = vec![];

        for i_level in 0..=depth {
            bids_mult.push(bids[i_level][0] * bids[i_level][1]);
            asks_mult.push(asks[i_level][0] * asks[i_level][1]);
            vol_sum.push(bids[i_level][1] + asks[i_level][1]);
        }

        let r_vwap_1: f64 = bids_mult.iter().sum();
        let r_vwap_2: f64 = asks_mult.iter().sum();
        let r_vwap_3: f64 = vol_sum.iter().sum();
        (r_vwap_1 + r_vwap_2) / r_vwap_3
    }
}

/// A struct to represent the OrderBook Volume Imbalance as a market metric.
pub struct VolumeImbalance;
impl MarketMetric<Vec<f64>> for VolumeImbalance {
    /// Computes the Order book Volume Imbalance from the given bids and asks.
    ///
    /// # Parameters
    /// - `bids`: A vector of values, where each contains only the volume for the bids levels.
    /// - `asks`: A vector of values, where each contains only the volume for the asks levels.
    /// - `depth`: The index specifying up to which level of the orderbook to compute the Volume Imbalance.
    ///
    /// # Returns
    /// Returns the Volume Imbalance as a `f64`.

    fn compute(bids: &Vec<f64>, asks: &Vec<f64>, depth: usize) -> f64 {
        let mut bids_mult: Vec<f64> = vec![];
        let mut asks_mult: Vec<f64> = vec![];

        for i_level in 0..=depth {
            bids_mult.push(bids[i_level]);
            asks_mult.push(asks[i_level]);
        }

        let r_obimb_1: f64 = bids_mult.iter().sum();
        let r_obimb_2: f64 = asks_mult.iter().sum();
        (r_obimb_1 - r_obimb_2) / (r_obimb_1 + r_obimb_2)
    }
}

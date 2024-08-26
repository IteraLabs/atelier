// The trait Metric
// a Value will be considered as a Metric if implements:
// 1. Define: A method to express the definition of the calculation.
// 2. Compute: A method to perform the calculation.

pub trait PriceVolumeMetric<V> {
    fn compute(bids: V, asks: V, depth: u32) -> f64;
}

pub struct Midprice;

impl PriceVolumeMetric<Vec<f64>> for Midprice {
    fn compute(bids: Vec<f64>, asks: Vec<f64>, depth: u32) -> f64 {
        (bids[0] + asks[0]) / 2.0
    }
}

pub struct WeightedMidPrice;

impl PriceVolumeMetric<Vec<f64>> for WeightedMidPrice {
    fn compute(bids: Vec<f64>, asks: Vec<f64>, depth: u32) -> f64 {
        (bids[0] * bids[1] + asks[0] * asks[1]) / (bids[1] + asks[1])
    }
}

pub struct VWAP;

impl PriceVolumeMetric<Vec<Vec<f64>>> for VWAP {
    fn compute(bids: Vec<Vec<f64>>, asks: Vec<Vec<f64>>) -> f64 {
        1.0
    }
}

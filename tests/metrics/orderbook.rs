
#[cfg(test)]

// -- ----------------------------------------------------------------- TESTS UTILS -- //
// -- ----------------------------------------------------------------- ----------- -- //

mod test_orderbook_utils {

    use atelier::data::market::Orderbook;

    pub const DEFAULT_BID_PRICE: f64 = 50_000.00;
    pub const DEFAULT_ASK_PRICE: f64 = 50_100.00;
    pub const DEFAULT_TICK_SIZE: f64 = 10.00;
    pub const DEFAULT_N_LEVELS: u32 = 4;
    pub const DEFAULT_N_ORDERS: u32 = 2;

    pub fn create_test_deep_orderbook(
        bid_price: f64,
        ask_price: f64,
        tick_size: f64,
        n_levels: u32,
        n_orders: u32
    ) -> Orderbook {
        Orderbook::synthetize(
            bid_price,
            ask_price,
            tick_size,
            n_levels,
            n_orders)
    }
    
    // ---------------------------------------------------------- DEFAULT ORDERBOOK -- //
    
    pub fn default_test_orderbook() -> Orderbook {
        create_test_deep_orderbook(
            DEFAULT_BID_PRICE,
            DEFAULT_ASK_PRICE,
            DEFAULT_TICK_SIZE,
            DEFAULT_N_LEVELS,
            DEFAULT_N_ORDERS
        )
    }
}

// -- ----------------------------------------------------------------------- TESTS -- //
// -- ----------------------------------------------------------------------- ----- -- //

mod tests {

    use approx::assert_abs_diff_eq;
    use atelier::metrics::orderbook;
    use atelier::metrics::orderbook::{OrderBookMetric, MetricResult};

    // -- ---------------------------------------------------------------- TickSize -- //
    // -- ---------------------------------------------------------------- -------- -- //
    
    #[test] 
    // -- ---------------------------------------------------------------------------- //
    fn test_ticksize_output_type() {
        
        use super::*;
        use test_orderbook_utils::*;
        let testable_ob = default_test_orderbook();

        let t_levels_bids = &testable_ob.bids;
        let test_result = orderbook::TickSize::compute(t_levels_bids, 4);

        assert!(matches!(test_result, MetricResult::Values(_)));

    }
    
    #[test] 
    // -- ---------------------------------------------------------------------------- //
    fn test_ticksize_output_value() {
        
        use super::*;
        use test_orderbook_utils::*;
        let testable_ob = default_test_orderbook();

        let t_levels_bids = &testable_ob.bids;
        let metric_result = orderbook::TickSize::compute(t_levels_bids, 4);

        let test_result: Vec<f64> = match metric_result {
            MetricResult::Values(value) => value,
            MetricResult::Value(_f64) => panic!("Unexpected variant of the MetricResult"),
        };

        let result:f64 = test_result.iter().sum();
        let after = f64::trunc(result * 1_000_000.0) / 1_000_000.0;
         
        assert_eq!(after, 30.0);

    }
    
    // -- --------------------------------------------------------- VolumeImbalance -- //
    // -- -------------------------------------------------------------------- ---- -- //

    #[test] 
    // -- ---------------------------------------------------------------------------- //
    fn test_volumeimbalance_output_type() {
        
        use super::*;
        use test_orderbook_utils::*;
        let testable_ob = default_test_orderbook();
        let t_levels = &(&testable_ob.bids, &testable_ob.asks);
        let test_result = orderbook::VolumeImbalance::compute(t_levels, 4);
        
        assert!(matches!(test_result, MetricResult::Value(_)));

    }
    
    // -- ------------------------------------------------------------ OrdersAmount -- //
    // -- -------------------------------------------------------------------- ---- -- //
    
    #[test] 
    // -- ---------------------------------------------------------------------------- //
    fn test_ordersamount_output_type() {
        
        use super::*;
        use test_orderbook_utils::*;
        let testable_ob = default_test_orderbook();

        let t_levels_bids = &testable_ob.bids;
        let test_result = orderbook::OrdersAmount::compute(t_levels_bids, 4);
        
        assert!(matches!(test_result, MetricResult::Value(_)));

    }

    // -- ------------------------------------------------------------ OrdersVolume -- //
    // -- -------------------------------------------------------------------- ---- -- //
    
    #[test] 
    // -- ---------------------------------------------------------------------------- //
    fn test_ordersvolume_output_type() {
        
        use super::*;
        use test_orderbook_utils::*;
        let testable_ob = default_test_orderbook();

        let t_levels_bids = &testable_ob.bids;
        let test_result = orderbook::OrdersVolume::compute(t_levels_bids, 4);
        
        assert!(matches!(test_result, MetricResult::Value(_)));

    }

    // -- -------------------------------------------------------------------- VWAP -- //
    // -- -------------------------------------------------------------------- ---- -- //
    
    #[test]
    // -- ---------------------------------------------------------------------------- //
    fn test_vwap_output_type() {
        use super::*;
        use test_orderbook_utils::*;
        let testable_ob = default_test_orderbook();
    
        let t_levels = &(&testable_ob.bids, &testable_ob.asks);
        // Get the result for this test
        let test_result = orderbook::VWAP::compute(t_levels, 4);
        assert!(matches!(test_result, MetricResult::Value(_f64)));
    }
    
    #[test]
    // -- ---------------------------------------------------------------------------- //
    fn test_vwap_expectation() {
        use super::*;
        use test_orderbook_utils::*;
        let testable_ob = default_test_orderbook();
        
        let t_levels = &(&testable_ob.bids, &testable_ob.asks);
        let metric_result = orderbook::VWAP::compute(t_levels, 4);
        let expected_result = 55_000.00;
        
        let test_result = match metric_result {
            MetricResult::Value(value) => value,
            MetricResult::Values(_) => panic!("Unexpected variant of the MetricResult"),
        };
        assert_abs_diff_eq!(test_result, expected_result, epsilon = 1.0);
    }

    // -- ---------------------------------------------------------------- Midprice -- //
    // -- ---------------------------------------------------------------- -------- -- //
    
    #[test]
    // -- ---------------------------------------------------------------------------- //
    fn test_midprice_output_type() {
        // Parameters for this test
        let v_prices = vec![70_000.00, 70_100.00];
    
        // Get the result for this test
        let test_result = orderbook::Midprice::compute(&v_prices, 0);

        // assert_matches!(MetricResult::Value);
        assert!(matches!(test_result, MetricResult::Value(_f64)));
    }
    
    #[test]
    // -- ---------------------------------------------------------------------------- //
    fn test_midprice_expectation() {
        // Parameters for this test
        let v_prices = vec![70_000.00, 70_100.00];
    
        // Get the result for this test
        let metric_result = orderbook::Midprice::compute(&v_prices, 0);

        let test_result = match metric_result {
            MetricResult::Value(value) => value,
            MetricResult::Values(_) => panic!("Unexpected variant of the MetricResult"),
        };
        
        // assert_matches!(MetricResult::Value);
        assert_eq!(test_result, 70_050.00);
    }
    
    // -- ------------------------------------------------------------------ Spread -- //
    // -- ------------------------------------------------------------------ ------ -- //
    
    #[test]
    // -- ---------------------------------------------------------------------------- //
    fn test_spread_output_type() {
        // Parameters for this test
        let v_prices = vec![70_000.00, 70_100.00];
    
        // Get the result for this test
        let test_result = orderbook::Spread::compute(&v_prices, 0);

        // assert_matches!(MetricResult::Value);
        assert!(matches!(test_result, MetricResult::Value(_f64)));
    }
    
    #[test]
    // -- ---------------------------------------------------------------------------- //
    fn test_spread_expectation() {
        // Parameters for this test
        let v_prices = vec![70_000.00, 70_100.00];
        let metric_result = orderbook::Spread::compute(&v_prices, 0);

        let spread = match metric_result {
            MetricResult::Value(value) => value,
            MetricResult::Values(_) => panic!("Unexpected variant of the MetricResult"),
        };

        assert_eq!(spread, 300.0);
    }
}

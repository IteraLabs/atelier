#[cfg(test)]

// -- ----------------------------------------------------------------- TESTS UTILS -- //
// -- ----------------------------------------------------------------- ----------- -- //

mod test_synthetizer_utils {

    use atelier::data::market::Orderbook;

    // ---------------------------------------------------------- DEFAULT ORDERBOOK -- //

    pub fn create_init_orderbook(
    ) -> Orderbook {
        Orderbook::random()
    }

    pub fn default_test_orderbook() -> Orderbook {
        create_init_orderbook()
    }
}

mod tests {

    use atelier::generators::synthetizer;

    #[test]
    // -- ---------------------------------------------------------------------------- //
    fn test_generator_synthetizer_output_type() {
        
        // Parameters for this test
        use super::*;
        use test_synthetizer_utils::*;
        let test_init_ob = default_test_orderbook();

        // Create the synthetic progressions
        let test_n_progressions: u16 = 5;
        let test_tick_size = 10.01;
        let test_n_levels = 4;
        let test_n_orders = 3;

        let test_synthetizer = synthetizer::Synthetizer::new(test_init_ob.clone());
        
        let synthetic_ob = test_synthetizer.brownian(
            test_n_progressions,
            test_tick_size,
            test_n_levels,
            test_n_orders,
        );

        assert!(matches!(synthetic_ob, _test_init_ob));
    }

    #[test]
    // -- ---------------------------------------------------------------------------- //
    fn test_generator_synthetizer() {
        use atelier::generators::synthetizer::Synthetizer;

        // Create the initial_ob
        use super::*;
        use test_synthetizer_utils::*;
        let test_init_ob = default_test_orderbook();

        // Create the synthetic progressions
        let test_n_progressions: u16 = 5;
        let test_tick_size = 10.01;
        let test_n_levels = 4;
        let test_n_orders = 3;

        let test_synthetizer = Synthetizer::new(test_init_ob.clone());
        
        
        let synthetic_ob = test_synthetizer.brownian(
            test_n_progressions,
            test_tick_size,
            test_n_levels,
            test_n_orders,
        );

        assert_eq!(synthetic_ob.len(), 1 + test_n_progressions as usize);
     
    }
}

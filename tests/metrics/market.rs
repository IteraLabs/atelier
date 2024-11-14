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

    // -- ---------------------------------------------------------------- -------- -- //
    // -- ---------------------------------------------------------------- -------- -- //
    
    #[test]
    // -- ---------------------------------------------------------------------------- //
    fn test_type() {
   
    }

}


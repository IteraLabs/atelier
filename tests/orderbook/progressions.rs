#[cfg(test)]
mod tests {
    use atelier::data::market::Orderbook;

    #[test]
    fn symmetric_sides() {
        let bid_price = 50_000.00;
        let ask_price = 50_100.00;
        let tick_size = 100.0;
        let n_levels = 10;
        let n_orders = 2;

        let i_ob = Orderbook::synthetize(bid_price, ask_price, tick_size, n_levels, n_orders);

        assert_eq!(i_ob.bids.len(), n_levels as usize);
        assert_eq!(i_ob.asks.len(), n_levels as usize);
        assert_eq!(i_ob.bids.len(), i_ob.asks.len());
    }
}

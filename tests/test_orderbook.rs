#[cfg(test)]
mod tests {
    use atelier::data::market::Orderbook;
    use atelier::simulation::randomizer;

    #[test]
    fn symmetric_sides() {
        let bid_price = 50_000.00;
        let ask_price = 50_100.00;
        let tick_size = 100.0;
        let n_levels = 10;
        let n_orders = 2;

        let i_ob = Orderbook::synthetize(
            bid_price, ask_price, tick_size, n_levels, n_orders,
        );

        assert_eq!(i_ob.bids.len(), n_levels as usize);
        assert_eq!(i_ob.asks.len(), n_levels as usize);
        assert_eq!(i_ob.bids.len(), i_ob.asks.len());
    }

    #[test]
    fn naive_progression() {
        // A correct progression of an orderbook should produce:
        // different orders at the same side -> level -> order queue

        // Parameters for the orderbook creation
        let bid_price = 50_000.00;
        let ask_price = 50_100.00;
        let tick_size = 50.0;
        let n_levels = 20;
        let n_orders = 5;

        // Parameters for the midprice progression model
        let mu = 0.0001;
        let sigma = 0.0025;

        let orderbook = Orderbook::synthetize(
            bid_price, ask_price, tick_size, n_levels, n_orders,
        );
        let mut n_orderbooks: Vec<Orderbook> = vec![];
        n_orderbooks.push(orderbook);

        for i in 0..=3 {
            let i_bid_price = n_orderbooks[i].bids[0].price;
            let i_ret_gbm_bids: f64 =
                randomizer::gbm_return(i_bid_price, mu, sigma, 1.0);

            let i_ask_price = n_orderbooks[i].asks[0].price;
            let i_ret_gbm_asks: f64 =
                randomizer::gbm_return(i_ask_price, mu, sigma, 1.0);

            let i_orderbook = Orderbook::synthetize(
                i_bid_price - i_ret_gbm_bids,
                i_ask_price + i_ret_gbm_asks,
                tick_size,
                n_levels,
                n_orders,
            );

            n_orderbooks.push(i_orderbook);
        }

        // for the first and second orderbook, compare the first order of the first level

        // for the bid side
        let t0_bids0_order0 = n_orderbooks[0].bids[0].orders[0];
        let t1_bids0_order0 = n_orderbooks[1].bids[0].orders[0];
        assert_ne!(t0_bids0_order0, t1_bids0_order0);

        // for ask side
        let t0_asks0_order0 = n_orderbooks[0].asks[0].orders[0];
        let t1_asks0_order0 = n_orderbooks[1].asks[0].orders[0];
        assert_ne!(t0_asks0_order0, t1_asks0_order0);
    }
}

use atelier::data::market::Orderbook;
use atelier::generators::brownian;
use atelier::metrics::market;
use atelier::metrics::market::MarketMetric;

fn main() {
    // Parameters for the first Order Book
    let bid_price = 50_000.00;
    let ask_price = 50_100.00;
    let tick_size = 100.0;
    let n_levels = 200;
    let n_orders = 10;

    // Orderbook data structure
    let orderbook = Orderbook::synthetize(bid_price, ask_price, tick_size, n_levels, n_orders);
    let mut n_orderbooks: Vec<Orderbook> = vec![];
    n_orderbooks.push(orderbook);

    // Progression model parameters
    let n_progressions: usize = 50;
    let mu = 0.001;
    let sigma = 0.0025;

    // ---------------------------------------------------- Progressions Generation -- //
    // ---------------------------------------------------- ----------------------- -- //

    for i in 0..=n_progressions {
        let i_bid_price = n_orderbooks[i].bids[0].price;
        let i_ask_price = n_orderbooks[i].asks[0].price;

        // Generate a single progression for the price change
        let i_ret_gbm_bids: f64 = brownian::gbm_return(i_bid_price, mu, sigma, 0.1, 1).unwrap()[0];
        let i_ret_gbm_asks: f64 = brownian::gbm_return(i_ask_price, mu, sigma, 0.1, 1).unwrap()[0];

        let i_orderbook = Orderbook::synthetize(
            i_bid_price - i_ret_gbm_bids,
            i_ask_price + i_ret_gbm_asks,
            tick_size,
            n_levels,
            n_orders,
        );
        n_orderbooks.push(i_orderbook);
    }

    // ------------------------------------------------ Market Metrics Calculations -- //
    // ------------------------------------------------ --------------------------- -- //

    let mut v_mids: Vec<f64> = vec![];
    let mut v_vwap: Vec<f64> = vec![];
    let mut v_spreads: Vec<f64> = vec![];

    //Compute the Spread
    for i in 0..=n_progressions {
        // Compute the spreads
        v_spreads.push(market::Spread::compute(
            &n_orderbooks[i].bids[0].price,
            &n_orderbooks[i].asks[0].price,
            0,
        ));

        // Compute the mids
        v_mids.push(market::Midprice::compute(
            &n_orderbooks[i].bids[0].price,
            &n_orderbooks[i].asks[0].price,
            0,
        ));

        // Compute the Volume-Weighted Average Price
        let iter_bids: Vec<_> = n_orderbooks[i]
            .bids
            .clone()
            .into_iter()
            .map(|x| vec![x.price, x.volume])
            .collect();

        let iter_asks: Vec<_> = n_orderbooks[i]
            .asks
            .clone()
            .into_iter()
            .map(|x| vec![x.price, x.volume])
            .collect();

        // Compute the VWAP
        v_vwap.push(market::VWAP::compute(
            &iter_bids.clone(),
            &iter_asks.clone(),
            4,
        ));
    }

    // Generate output data
    println!("v_spreads: {:?}", v_spreads);
    println!("v_mids: {:?}", v_mids);
    println!("v_vwap: {:?}", v_vwap);
}

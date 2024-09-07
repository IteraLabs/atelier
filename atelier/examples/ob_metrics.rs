use atelier::data::market::Orderbook;
use atelier::metrics::market::{MarketMetric, VWAP, Midprice};

fn main() {
    let bid_price = 50_000.00;
    let ask_price = 50_100.00;
    let tick_size = 100.0;
    let n_levels = 5;
    let n_orders = 2;

    // generate a synthetic orderbook for testing
    let i_ob = Orderbook::synthetize(
        bid_price, ask_price, tick_size, n_levels, n_orders,
    );

    // extract tob values
    let tob_bid: Vec<(f64, f64)> =
        vec![(i_ob.bids[0].price, i_ob.bids[0].volume)];
    let tob_ask: Vec<(f64, f64)> =
        vec![(i_ob.asks[0].price, i_ob.asks[0].volume)];

    // compute the midprice using the Midprice struct
    let midprice_value = Midprice::compute(tob_bid, tob_ask, 0);
    println!("Computed Midprice: {}", midprice_value);

    let iter_bids = i_ob.bids.into_iter().map(|x| (x.price, x.volume)).collect();
    let iter_asks = i_ob.asks.into_iter().map(|x| (x.price, x.volume)).collect(); 
    let vwap_value = VWAP::compute(iter_bids, iter_asks, 0);
    
    println!("VWAP: {:?}", vwap_value);
}

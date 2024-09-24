use atelier::data::market::{Orderbook, Side};
use atelier::metrics::market::{MarketMetric, Midprice, Spread, VolumeImbalance, VWAP};

fn main() {
    // Parameters for synthetic orderbook generation
    let bid_price = 50_000.00;
    let ask_price = 50_100.00;
    let tick_size = 100.0;
    let n_levels = 2;
    let n_orders = 1;

    // Generate a synthetic orderbook for testing
    let i_ob = Orderbook::synthetize(bid_price, ask_price, tick_size, n_levels, n_orders);

    // Generated Orderbook
    println!("{:?}", i_ob);

    // let find_ob_level = i_ob.find_level(50_200.0);
    // println!("{:?}", new_ob_level);

    let content_ob_level = i_ob.retrieve_level(50_200.0);
    println!("found:\n {:?}", content_ob_level);

    // get the TOB
    // let tob_data = i_ob.get_tob();
    // println!("TOB: {:?}", tob_data);

    // extract tob values
    // let tob_bid: f64 = i_ob.bids[0].price;
    // let tob_ask: f64 = i_ob.asks[0].price;

    // Compute the Spread
    // let spread_value = Spread::compute(&tob_bid, &tob_ask, 0);
    // println!("Spread: {:?}", spread_value);

    // Compute the Midprice
    // let midprice_value = Midprice::compute(&tob_bid, &tob_ask, 0);
    // println!("Midprice: {}", midprice_value);

    // Compute the Volume Imbalance
    // let iter_bids: Vec<f64> = i_ob.bids.clone().into_iter().map(|x| x.volume).collect();
    // let iter_asks: Vec<f64> = i_ob.asks.clone().into_iter().map(|x| x.volume).collect();

    // let obimb_value = VolumeImbalance::compute(&iter_bids, &iter_asks, 1);
    // println!("Volume Imbalance: {:?}", obimb_value);

    // Compute the Volume-Weighted Average Price
    //let iter_bids: Vec<_> = i_ob
    //     .bids
    //    .into_iter()
    //    .map(|x| vec![x.price, x.volume])
    //   .collect();
    // let iter_asks: Vec<_> = i_ob
    //    .asks
    //    .into_iter()
    //    .map(|x| vec![x.price, x.volume])
    //    .collect();

    // Compute the VWAP
    // let vwap_value = VWAP::compute(&iter_bids.clone(), &iter_asks.clone(), 1);
    // println!("VWAP: {:?}", vwap_value);
}

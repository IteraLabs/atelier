use atelier::data::market;
use atelier::data::market::Orderbook;
use atelier::generators::hawkes;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Parameters for the Initial State for the Order Book
    let bid_price = 70_000.00;
    let ask_price = 70_100.00;
    let tick_size = 100.0;
    let n_levels = 4;
    let n_orders = 2;

    let n_orderbooks = Orderbook::synthetize(bid_price, ask_price, tick_size, n_levels, n_orders);

    // println!("n_orderbooks: {:?}", n_orderbooks);

    // Define the step window (seconds)
    let step_window: f64 = 1.0;

    // Within the step window, all the events need to take place
    let market_orders_times = hawkes::HawkesProcess {
        mu: 0.85,
        alpha: 0.80,
        beta: 1.0,
    };

    let cancel_orders_times = hawkes::HawkesProcess {
        mu: 0.50,
        alpha: 0.40,
        beta: 1.1,
    };

    let limit_orders_times = hawkes::HawkesProcess {
        mu: 0.90,
        alpha: 0.81,
        beta: 0.95,
    };

    let current_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;

    // -- Independent from each other but all start at the same time point (step by step)

    // -- Generate: Time of arrival, Side, Amount.
    let _mo_times = market_orders_times.generate_values(current_ts, 2);
    let _mo_sides: Vec<market::Side> = vec![];
    let _mo_amounts: Vec<f64> = vec![];

    // -- Generate: Time of arrival, Order Selection.
    // everal empirical studies covering a wide range of different markets have
    // concluded that the vast majority of active orders ended in cancellation
    // rather than matching. between 70% and 80% according to "Limit Order Books, Gould"
    let _co_times = cancel_orders_times.generate_values(current_ts, 3);
    let _co_orders: Vec<market::Order> = vec![];

    // -- Generate: Time of arrival, Side, Amount, Price.
    let _lo_times = limit_orders_times.generate_values(current_ts, 4);
    let _lo_sides: Vec<market::Side> = vec![];
    let _lo_amounts: Vec<f64> = vec![];
    let _lo_price: Vec<f64> = vec![];

    // Generate a Market Order {Buy|Sell}

    // Generate a New limit Order {Buy|Sell}
    // a) Within the spread (from 0 to TOB)
    // b) Outside the spread (Above Spread Below HOB)
    // c) Outside the spread (Above HOB Below LOB)

    // Generate a Cancelation of a Limit Order {Buy|Sell}
    // a) Within the spread (from 0 to TOB)
    // b) Outside the spread (Above Spread Below HOB)
    // c) Outside the spread (Above HOB Below LOB)
}

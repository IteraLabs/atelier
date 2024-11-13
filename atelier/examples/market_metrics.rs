use atelier::data::market::Orderbook;
use atelier::metrics::market;
use atelier::metrics::market::OrderBookMetric;

fn main() {
    // Parameters for the Initial State for the Order Book
    let bid_price = 70_000.00;
    let ask_price = 70_100.00;
    let tick_size = 200.0;
    let n_levels = 4;
    let n_orders = 2;

    // Create synthetic progression of orderbooks
    let n_orderbooks = Orderbook::synthetize(bid_price, ask_price, tick_size, n_levels, n_orders);

    let v_bids = n_orderbooks.bids;
    let v_asks = n_orderbooks.asks;

    // ---------------------------------------------------------------------- TICKS -- //
    // ---------------------------------------------------------------------- ----- -- //

    let bid_ticks = market::TickSize::compute(
        &v_bids
            .clone()
            .into_iter()
            .map(|level| level.price)
            .collect(),
        2,
    );

    let ask_ticks = market::TickSize::compute(
        &v_asks
            .clone()
            .into_iter()
            .map(|level| level.price)
            .collect(),
        2,
    );

    println!("computed bid_ticks: {:?}", bid_ticks);
    println!("computed ask_ticks: {:?}", ask_ticks);

    // -------------------------------------------------------------- ORDERS AMOUNT -- //
    // -------------------------------------------------------------- ------------- -- //

    let book_depth = 4;

    let bids_orders_amount =
        market::OrdersAmount::compute(&v_bids.clone().into_iter().collect(), book_depth);

    let asks_orders_amount =
        market::OrdersAmount::compute(&v_bids.clone().into_iter().collect(), book_depth);

    println!("orders amount in the Bid side: {:?}", bids_orders_amount);
    println!("orders amount in the Ask side: {:?}", asks_orders_amount);
}

use atelier::data::market::Orderbook;
use atelier::metrics::orderbook;
use atelier::metrics::orderbook::OrderBookMetric;

fn main() {
    // Parameters for the Initial State for the Order Book
    let bid_price = 70_000.00;
    let ask_price = 70_100.00;
    let tick_size = 200.0;
    let n_levels = 4;
    let n_orders = 2;

    // Create synthetic progression of orderbooks
    let n_orderbooks = Orderbook::synthetize(
        bid_price,
        ask_price,
        tick_size,
        n_levels,
        n_orders);

    // sides of the order book Vec<Level>
    let v_bids = n_orderbooks.bids;
    let v_asks = n_orderbooks.asks;
    let book_depth = 4;

    // --------------------------------------------------------------------- SPREAD -- //
    // --------------------------------------------------------------------- ------ -- //

    let spread = orderbook::Spread::compute(&vec![v_bids[0].price, v_asks[0].price], 0);
    println!("The computed spread is: {:?}", spread);

    // ------------------------------------------------------------------- MIDPRICE -- //
    // ------------------------------------------------------------------- -------- -- //

    let midprice = orderbook::Midprice::compute(&vec![v_bids[0].price, v_asks[0].price], 0);
    println!("The computed midprice is: {:?}", midprice);

    // ----------------------------------------------------------------------- VWAP -- //
    // ----------------------------------------------------------------------- ---- -- //

    let vwap = orderbook::VWAP::compute(&(&v_bids, &v_asks), book_depth);
    println!("With a depth of: {:?}, the VWAP is: {:?}", book_depth, vwap);

    // ------------------------------------------------------------ VOLUME_IMBALANCE-- //
    // ------------------------------------------------------------ --------------- -- //

    let volume_imb = orderbook::VolumeImbalance::compute(&(&v_bids, &v_asks), book_depth);
    println!(
        "With a depth of: {:?}, the Volume Imbalance is: {:?}",
        book_depth, volume_imb
    );

    // ---------------------------------------------------------------------- TICKS -- //
    // ---------------------------------------------------------------------- ----- -- //

    let bid_ticks = orderbook::TickSize::compute(&v_bids, 4);
    println!("bid_ticks: {:?}", bid_ticks);

    let ask_ticks = orderbook::TickSize::compute(&v_asks, 4);
    println!("ask_ticks: {:?}", ask_ticks);

    // -------------------------------------------------------------- ORDERS AMOUNT -- //
    // -------------------------------------------------------------- ------------- -- //

    let bids_orders_amount = orderbook::OrdersAmount::compute(&v_bids, book_depth);
    println!("orders amount in the Bid side: {:?}", bids_orders_amount);

    let asks_orders_amount = orderbook::OrdersAmount::compute(&v_bids, book_depth);
    println!("orders amount in the Ask side: {:?}", asks_orders_amount);

    // -------------------------------------------------------------- ORDERS VOLUME -- //
    // -------------------------------------------------------------- ------------- -- //

    let bids_orders_volume = orderbook::OrdersVolume::compute(&v_bids, book_depth);
    let asks_orders_volume = orderbook::OrdersVolume::compute(&v_asks, book_depth);
    println!(
        "The Volume, at the Bid Side, of each level up to: {:?} depth is: {:?}",
        book_depth, bids_orders_volume
    );
    
    println!(
        "The Volume, at the Ask Side, of each level up to: {:?} depth is: {:?}",
        book_depth, asks_orders_volume
    );
}

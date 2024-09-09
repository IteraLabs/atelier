use atelier::data::market::Orderbook;
use atelier::simulation::randomizer;

fn main() {
    let bid_price = 50_000.00;
    let ask_price = 50_100.00;

    let tick_size = 100.0;
    let n_levels = 200;
    let n_orders = 10;
    let mu = 0.0001;
    let sigma = 0.0025;

    let orderbook = Orderbook::synthetize(
        bid_price, ask_price, tick_size, n_levels, n_orders,
    );

    let mut n_orderbooks: Vec<Orderbook> = vec![];
    n_orderbooks.push(orderbook);

    println!("pre-bid_price {}", n_orderbooks[0].bids[0].price);
    println!("pre-ask_price {}", n_orderbooks[0].asks[0].price);
    println!(
        "pre-mid_price {}",
        (n_orderbooks[0].bids[0].price + n_orderbooks[0].asks[0].price) / 2.0
    );

    for i in 0..=3 {
        let i_bid_price = n_orderbooks[i].bids[0].price;
        println!("\n{}-bid_price: {}", i, i_bid_price);
        let i_ask_price = n_orderbooks[i].asks[0].price;
        println!("{}-ask_price: {}", i, i_ask_price);
        let i_mid_price = (i_bid_price + i_ask_price) / 2.0;
        println!("{}-mid_price: {}", i, i_mid_price);
        println!("{}-spread: {}", i, i_ask_price - i_bid_price);

        let i_ret_gbm_bids: f64 =
            randomizer::gbm_return(i_bid_price, mu, sigma, 1.0);
        println!("{}-ret_gbm_bids: {}", i, i_ret_gbm_bids);

        let i_ret_gbm_asks: f64 =
            randomizer::gbm_return(i_ask_price, mu, sigma, 1.0);
        println!("{}-ret_gbm_asks: {}", i, i_ret_gbm_asks);

        let i_orderbook = Orderbook::synthetize(
            i_bid_price - i_ret_gbm_bids,
            i_ask_price + i_ret_gbm_asks,
            tick_size,
            n_levels,
            n_orders,
        );

        n_orderbooks.push(i_orderbook);
    }

    println!(
        "orderbooks[0].bids[0].orders[0] (t=0) {:?}",
        n_orderbooks[0].bids[0].orders[1]
    );
    println!(
        "orderbooks[0].bids[0].orders[1] (t=0) {:?}",
        n_orderbooks[0].bids[0].orders[2]
    );

    println!(
        "\norderbooks[1].bids[0].orders[0] (t=1) {:?}",
        n_orderbooks[1].bids[0].orders[0]
    );
    println!(
        "orderbooks[1].bids[0].orders[1] (t=1) {:?}",
        n_orderbooks[1].bids[0].orders[1]
    );
}

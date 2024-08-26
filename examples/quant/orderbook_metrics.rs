use atelier::data::market::Orderbook;

fn main() {
    let bid_price = 50_000.00;
    let ask_price = 50_100.00;
    let tick_size = 100.0;
    let n_levels = 200;
    let n_orders = 300;

    let i_ob = Orderbook::synthetize(
        bid_price, ask_price, tick_size, n_levels, n_orders,
    );

    let mid_price: f64 = i_ob.mid_price();
    println!("mid_price: {}", mid_price);

    let weighted_mid_price: f64 = i_ob.weighted_mid_price();
    println!(
        "weighted_mid_price: {}",
        weighted_mid_price
    );
}

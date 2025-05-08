//! Generation of Synthetic Orderbook
//!
//! This code is to generate 3 different markets (of the same pair)
//! with slightly different values in order to explore
//! distributed convex optimization theory
//!

use rand::Rng;
use rand_distr::{Bernoulli, Distribution, Uniform};

use atelier_data::orderbooks::Orderbook;

fn main() {
    // -- Asia Orderbook -- //

    let bid_price_0 = 100_000.00;
    let bid_levels_0 = Some((1, 2));
    let bid_orders_0 = Some((1, 10));
    let ask_price_0 = 100_001.00;
    let ask_levels_0 = Some((1, 2));
    let ask_orders_0 = Some((1, 10));
    let ticksize_0 = Some((0.1, 1.1));

    // -- Europe Orderbook -- //

    let bid_price_1 = 100_000.00;
    let bid_levels_1 = Some((1, 2));
    let bid_orders_1 = Some((1, 10));
    let ask_price_1 = 100_001.00;
    let ask_levels_1 = Some((1, 2));
    let ask_orders_1 = Some((1, 10));
    let ticksize_1 = Some((0.1, 1.1));

    // -- Americas Orderbook -- //

    let bid_price_1 = 100_000.00;
    let bid_levels_1 = Some((1, 2));
    let bid_orders_1 = Some((1, 10));
    let ask_price_1 = 100_001.00;
    let ask_levels_1 = Some((1, 2));
    let ask_orders_1 = Some((1, 10));
    let ticksize_1 = Some((0.1, 1.1));

    let mut v_orderbook = vec![];
    let n_progressions = 50;

    // -- Probabilistic parameters -- //

    let uni_params = vec![-0.001, 0.005];
    let ber_params = vec![0.5];

    let mut rng = rand::rng();

    // ---------------------------------------------------------------------------------------- //

    for _ in 0..n_progressions {
        let uni_rand = Uniform::new(uni_params[0], uni_params[1])
            .expect("Failed to create Uniform distribution sampler");
        let r_amount_ret = rng.sample(uni_rand);

        println!("uni_rand: {:?}", r_amount_ret);

        let bernoulli = Bernoulli::new(ber_params[0]).unwrap();
        let r_sign_ret = if bernoulli.sample(&mut rng) {
            1.0
        } else {
            -1.0
        };

        let v_bid_price = bid_price_0 + bid_price_0 * r_amount_ret * 1.0;
        let v_ask_price = ask_price_0 + ask_price_0 * r_amount_ret * 1.0;

        let r_ob = Orderbook::random(
            v_bid_price,
            bid_levels_0,
            bid_orders_0,
            ticksize_0,
            v_ask_price,
            ask_levels_0,
            ask_orders_0,
        );

        v_orderbook.push(r_ob);
    }

    println!(
        "\nbid prices: {:?}, {:?}, {:?}",
        v_orderbook[0].bids[0].price, v_orderbook[1].bids[0].price, v_orderbook[2].bids[0].price
    );

    println!(
        "\nask prices: {:?}, {:?}, {:?}",
        v_orderbook[0].asks[0].price, v_orderbook[1].asks[0].price, v_orderbook[2].asks[0].price
    );
}

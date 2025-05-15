//! Generation of Synthetic Orderbook
//!
//! This code is to generate 3 different markets (of the same pair)
//! with slightly different values in order to explore
//! distributed convex optimization theory
//!

use atelier_data::{configs, orderbooks::Orderbook};
use rand::Rng;
use rand_distr::Uniform;
use serde_json;
use std::{fs::File, io::Write};

fn main() {

    // --- Experiment Parameters -- //
    
    // Number of progressions for all Orderbooks
    let n_progressions = 1000;

    // Random numbers
    let uni_params = vec![-0.01, 0.01];
    let mut rng = rand::rng();

    // Output
    println!("\nNo. of progressions: {:?}", n_progressions);

    let asia_config = configs::OrderbookConfig;
    let i_c = asia_config::loader("Config.toml");
    println!("asia_config: {:?}", i_c);

    // --- Asia Orderbook -- //

    // --- Generation Parameters
    let bid_price_0 = 100_000.00;
    let bid_levels_0 = Some((25, 50));
    let bid_orders_0 = Some((5, 10));
    let ask_price_0 = 100_001.00;
    let ask_levels_0 = Some((25, 50));
    let ask_orders_0 = Some((5, 10));
    let ticksize_0 = Some((0.1, 1.1));

    let mut v_orderbook_0 = vec![];

    // --- Generation progressions
    for _ in 0..n_progressions {
        let uni_rand = Uniform::new(uni_params[0], uni_params[1])
            .expect("Failed to create Uniform distribution sampler");

        let r_amount_ret = rng.sample(uni_rand);

        let v_bid_price = bid_price_0 + bid_price_0 * r_amount_ret;
        let v_ask_price = ask_price_0 + ask_price_0 * r_amount_ret;

        let r_ob = Orderbook::random(
            v_bid_price,
            bid_levels_0,
            bid_orders_0,
            ticksize_0,
            v_ask_price,
            ask_levels_0,
            ask_orders_0,
        );

        v_orderbook_0.push(r_ob);
    }

    // --- Write into JSON
    let asia_ob_json = serde_json::to_string(&v_orderbook_0).unwrap();
    let mut file = File::create("asia_orderbook.json").unwrap();
    file.write_all(asia_ob_json.as_bytes()).unwrap();

    // --- Presenting results
    println!("\n---- Asia Orderbook ----");

    println!("\nTOB:\n");
    println!(
        "  bid[0].volume {:.4}, bid[0].price {:.4}, ask[0].price {:.4}, ask[0].volume {:.4}",
        v_orderbook_0[0].bids[0].volume,
        v_orderbook_0[0].bids[0].price,
        v_orderbook_0[0].asks[0].price,
        v_orderbook_0[0].asks[0].volume
    );

    let level_bids: &Vec<f32> = &v_orderbook_0.iter().map(|x| x.bids.len() as f32).collect();
    let (level_bids_min, level_bids_max) = v_orderbook_0
        .iter()
        .map(|x| x.bids.len() as f32)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), val| {
            (min.min(val), max.max(val))
        });

    let stats_bids = vec![
        level_bids_min,
        level_bids_max,
        level_bids.into_iter().sum::<f32>() / level_bids.into_iter().len() as f32,
    ];

    let level_asks: &Vec<f32> = &v_orderbook_0.iter().map(|x| x.asks.len() as f32).collect();
    let (level_asks_min, level_asks_max) = v_orderbook_0
        .iter()
        .map(|x| x.asks.len() as f32)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), val| {
            (min.min(val), max.max(val))
        });

    let stats_asks = vec![
        level_asks_min,
        level_asks_max,
        level_asks.into_iter().sum::<f32>() / level_asks.into_iter().len() as f32,
    ];

    println!("\nLEVEL STATS: ");
    println!(
        "\n  No. of Bids levels stats (min, max, mean) {:?}
        \n  No. of Asks levels stats (min, max, mean) {:?}",
        stats_bids, stats_asks
    );

    // -------------------------------------------------------------------- Europe Orderbook -- //
    // -------------------------------------------------------------------- ---------------- -- //

    let bid_price_1 = 100_000.00;
    let bid_levels_1 = Some((25, 50));
    let bid_orders_1 = Some((5, 10));

    let ask_price_1 = 100_001.00;
    let ask_levels_1 = Some((25, 50));
    let ask_orders_1 = Some((5, 10));

    let ticksize_1 = Some((0.1, 1.1));

    let mut v_orderbook_1 = vec![];

    for _ in 0..n_progressions {
        let uni_rand = Uniform::new(uni_params[0], uni_params[1])
            .expect("Failed to create Uniform distribution sampler");

        let r_amount_ret = rng.sample(uni_rand);

        let v_bid_price = bid_price_1 + bid_price_1 * r_amount_ret;
        let v_ask_price = ask_price_1 + ask_price_1 * r_amount_ret;

        let r_ob = Orderbook::random(
            v_bid_price,
            bid_levels_1,
            bid_orders_1,
            ticksize_1,
            v_ask_price,
            ask_levels_1,
            ask_orders_1,
        );

        v_orderbook_1.push(r_ob);
    }

    // --- Write into JSON
    let euro_ob_json = serde_json::to_string(&v_orderbook_1).unwrap();
    let mut file = File::create("euro_orderbook.json").unwrap();
    file.write_all(euro_ob_json.as_bytes()).unwrap();

    println!("\n---- Euro Orderbook ----");

    println!("\nTOB:\n");
    println!(
        "  bid[0].volume {:.4}, bid[0].price {:.4}, ask[0].price {:.4}, ask[0].volume {:.4}",
        v_orderbook_1[0].bids[0].volume,
        v_orderbook_1[0].bids[0].price,
        v_orderbook_1[0].asks[0].price,
        v_orderbook_1[0].asks[0].volume
    );

    let level_bids: &Vec<f32> = &v_orderbook_1.iter().map(|x| x.bids.len() as f32).collect();
    let (level_bids_min, level_bids_max) = v_orderbook_1
        .iter()
        .map(|x| x.bids.len() as f32)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), val| {
            (min.min(val), max.max(val))
        });

    let stats_bids = vec![
        level_bids_min,
        level_bids_max,
        level_bids.into_iter().sum::<f32>() / level_bids.into_iter().len() as f32,
    ];

    let level_asks: &Vec<f32> = &v_orderbook_1.iter().map(|x| x.asks.len() as f32).collect();
    let (level_asks_min, level_asks_max) = v_orderbook_1
        .iter()
        .map(|x| x.asks.len() as f32)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), val| {
            (min.min(val), max.max(val))
        });

    let stats_asks = vec![
        level_asks_min,
        level_asks_max,
        level_asks.into_iter().sum::<f32>() / level_asks.into_iter().len() as f32,
    ];

    println!("\nLEVEL STATS: ");
    println!(
        "\n  No. of Bids levels stats (min, max, mean) {:?}
        \n  No. of Asks levels stats (min, max, mean) {:?}",
        stats_bids, stats_asks
    );

    // ------------------------------------------------------------------ Americas Orderbook -- //
    // ------------------------------------------------------------------ ------------------ -- //

    let bid_price_2 = 100_000.00;
    let bid_levels_2 = Some((25, 50));
    let bid_orders_2 = Some((5, 10));
    let ask_price_2 = 100_001.00;
    let ask_levels_2 = Some((25, 50));
    let ask_orders_2 = Some((5, 10));
    let ticksize_2 = Some((0.1, 1.1));

    let mut v_orderbook_2 = vec![];

    for _ in 0..n_progressions {
        let uni_rand = Uniform::new(uni_params[0], uni_params[1])
            .expect("Failed to create Uniform distribution sampler");

        let r_amount_ret = rng.sample(uni_rand);

        let v_bid_price = bid_price_2 + bid_price_2 * r_amount_ret;
        let v_ask_price = ask_price_2 + ask_price_2 * r_amount_ret;

        let r_ob = Orderbook::random(
            v_bid_price,
            bid_levels_2,
            bid_orders_2,
            ticksize_2,
            v_ask_price,
            ask_levels_2,
            ask_orders_2,
        );

        v_orderbook_2.push(r_ob);
    }
    // --- Write into JSON
    let americas_ob_json = serde_json::to_string(&v_orderbook_2).unwrap();
    let mut file = File::create("americas_orderbook.json").unwrap();
    file.write_all(americas_ob_json.as_bytes()).unwrap();

    println!("\n---- Americas Orderbook ----");

    println!("\nTOB:\n");
    println!(
        "  bid[0].volume {:.4}, bid[0].price {:.4}, ask[0].price {:.4}, ask[0].volume {:.4}",
        v_orderbook_2[0].bids[0].volume,
        v_orderbook_2[0].bids[0].price,
        v_orderbook_2[0].asks[0].price,
        v_orderbook_2[0].asks[0].volume
    );

    let level_bids: &Vec<f32> = &v_orderbook_2.iter().map(|x| x.bids.len() as f32).collect();
    let (level_bids_min, level_bids_max) = v_orderbook_2
        .iter()
        .map(|x| x.bids.len() as f32)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), val| {
            (min.min(val), max.max(val))
        });

    let stats_bids = vec![
        level_bids_min,
        level_bids_max,
        level_bids.into_iter().sum::<f32>() / level_bids.into_iter().len() as f32,
    ];

    let level_asks: &Vec<f32> = &v_orderbook_2.iter().map(|x| x.asks.len() as f32).collect();
    let (level_asks_min, level_asks_max) = v_orderbook_2
        .iter()
        .map(|x| x.asks.len() as f32)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), val| {
            (min.min(val), max.max(val))
        });

    let stats_asks = vec![
        level_asks_min,
        level_asks_max,
        level_asks.into_iter().sum::<f32>() / level_asks.into_iter().len() as f32,
    ];

    println!("\nLEVEL STATS: ");
    println!(
        "\n  No. of Bids levels stats (min, max, mean) {:?}
        \n  No. of Asks levels stats (min, max, mean) {:?}",
        stats_bids, stats_asks
    );
}

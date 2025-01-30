// Tutorial for Limit Order Book interaction

use atelier_data::orderbooks::Orderbook;
use rand::distributions::{Bernoulli, Uniform, Distribution};
use rand::Rng;

fn main() {

    let ini_bid_price = 100_000.00;
    let ini_bid_levels = 2;
    let ini_bid_orders = 3;

    let ini_ask_price = 100_001.00;
    let ini_ask_levels = 2;
    let ini_ask_orders = 3;

    let ini_ticksize = 1.0;

    let r_ob = Orderbook::random(
        ini_bid_price,
        ini_bid_levels,
        ini_bid_orders,
        ini_ticksize,
        ini_ask_price,
        ini_ask_levels,
        ini_ask_orders,
    );
    
    let mut v_orderbook: Vec<Orderbook> = vec![];

    for _ in 0..3 {
    
        let mut uni_rand = rand::thread_rng();
        let r_amount_ret = uni_rand.sample(Uniform::new(0.001, 0.005));
    
        let mut rng = rand::thread_rng();
        let bernoulli = Bernoulli::new(0.3).unwrap();
        let r_sign_ret = if bernoulli.sample(&mut rng) { 1.0 } else { -1.0 };
        
        let v_bid_price = ini_bid_price + ini_bid_price * r_amount_ret * r_sign_ret;
        let v_ask_price = ini_ask_price + ini_ask_price * r_amount_ret * r_sign_ret;
    
        let r_ob = Orderbook::random(
            v_bid_price,
            ini_bid_levels,
            ini_bid_orders,
            ini_ticksize,
            v_ask_price,
            ini_ask_levels,
            ini_ask_orders,
        );
        v_orderbook.push(r_ob);
    }

    // Orderbook progressions
    println!("\nbid prices: {:?}, {:?}, {:?}",
        v_orderbook[0].bids[0].price,
        v_orderbook[1].bids[0].price,
        v_orderbook[2].bids[0].price);
    
    println!("\nask prices: {:?}, {:?}, {:?}",
        v_orderbook[0].asks[0].price,
        v_orderbook[1].asks[0].price,
        v_orderbook[2].asks[0].price);
    
    // number of levels per side
    let n_bids = r_ob.bids.len();
    let n_asks = r_ob.asks.len();

    // mid price and total volume calculations
    let mid_price = (r_ob.asks[0].price + r_ob.bids[0].price) / 2.0;
    let volume_bids: f64 = r_ob.bids.clone().into_iter().map(|x| x.volume).sum();
    let volume_asks: f64 = r_ob.asks.clone().into_iter().map(|x| x.volume).sum();

    // random orders created at particular levels
    let n_orders_b0 = r_ob.bids[0].orders.len();
    let n_orders_a2 = r_ob.asks[1].orders.len();

    println!("\n-- Random Prices/Amounts generated --\n");
    println!("- Midprice: {:?}", mid_price);
    println!("- total volume in bids: {:?}", volume_bids);
    println!("- total volume in asks: {:?}", volume_asks);

    println!("\n-- Random Levels generated --\n");
    println!("- No. of levels in bids: {:?}", n_bids);
    println!("- No. of levels in asks: {:?}", n_asks);

    println!("\n-- Random Orders generated --\n");
    println!(
        "- No. of Orders in the First level, at the Bids side: {:?}",
        n_orders_b0
    );
    println!(
        "- No. of Orders in the Second level, at the Asks side: {:?}",
        n_orders_a2
    );
}

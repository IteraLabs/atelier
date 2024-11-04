//! # Misc Random Generation

use crate::data::market::{Order, OrderType, Side};
use rand::distributions::Uniform;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

// Create a random Order according to the provided parameters
// it uses structs::marketdata::order::Order

pub fn randomize_order(side: Side, price: f64, order_type: OrderType) -> Order {
    let mut uni_rand = rand::thread_rng();

    // Randomize order_ts
    let now_ts = SystemTime::now();
    // println!("now_ts: {:?}", now_ts);

    let since_epoch_ts = now_ts
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();
    // println!("since_epoch_ts: {:?}", since_epoch_ts);

    // Random millis between orders ~Uniform(1,30)
    let ms_offset = uni_rand.sample(Uniform::new(1, 30));
    // println!("ms_offset: {:?}", ms_offset);

    let order_ts = since_epoch_ts + ms_offset as u128;
    // println!("order_ts: {}", order_ts);

    // Randomize amount
    let amount = uni_rand.sample(Uniform::new(0.01, 10.0));

    // Randomize order_id
    let order_id: u32 = 123;

    // Parse Order Type
    let order_type = order_type;

    Order {
        order_id,
        order_ts,
        order_type,
        side,
        price,
        amount,
    }
}

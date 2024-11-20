/// Market event generator module

use rand::thread_rng;
use rand::seq::SliceRandom;
use atelier::events::templates::enum_create;
use atelier::events::templates;

fn main() {

    // -------------------------------------------------------------- new limit order -- //
    // -------------------------------------------------------------- --------------- -- //

    // Use the macro to create the enum
    enum_create!(
        EventType,
        CancelLimitOrder,
        NewMarketOrder,
        ModifyLimitOrder,
        NewLimitOrder);

    // Now you can use the enum
    let _variants = EventType::variants();
    let _r_variants = EventType::random_variants(2);

    // -- Thread for Event Queue : New Market Order
    let mut new_mo_queue:Vec<templates::MarketEvent> = vec![];
    for _ in 1..4 {
        new_mo_queue.push(
            templates::random_new_mo(
            ).unwrap()
        )
    }

    // -- Thread for Event Queue : Cancel Limit Order
    let mut cancel_lo_queue:Vec<templates::MarketEvent> = vec![];
    for _ in 1..4 {
        cancel_lo_queue.push(
            templates::random_cancel_lo(
            ).unwrap()
        )
    }

    println!("new_mo_queue: {:?}", new_mo_queue);
    println!("cancel_lo_queue: {:?}", cancel_lo_queue);

}


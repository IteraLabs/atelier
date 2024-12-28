use atelier::data::market::Orderbook;
use atelier::events::message;
use atelier::events::message::{MarketEvent, MarketEventType};
use atelier::events::templates;

fn main() {
    // -- Base OrderBook -- //

    // Parameters for synthetic orderbook generation
    let bid_price = 70_000.00;
    let ask_price = 70_100.00;
    let tick_size = 100.0;
    let n_levels = 20;
    let n_orders = 10;

    let _i_ob = Orderbook::synthetize(bid_price, ask_price, tick_size, n_levels, n_orders);

    // -------------------------------------------------------------- new limit order -- //
    // -------------------------------------------------------------- --------------- -- //

    // Use the macro to create the enum

    // Now you can use the enum
    // let _variants = EventType::variants();
    // let _r_variants = EventType::random_variants(2);

    // -- Thread for Event Queue : New Market Order
    // let mut new_mo_queue: Vec<templates::MarketEvent> = vec![];
    // for _ in 1..4 {
    //    new_mo_queue.push(templates::random_new_mo().unwrap())
    // }

    // -- Thread for Event Queue : Cancel Limit Order
    // let mut cancel_lo_queue: Vec<templates::MarketEvent> = vec![];
    // for _ in 1..4 {
    //    cancel_lo_queue.push(templates::random_cancel_lo().unwrap())
    // }

    // println!("new_mo_queue: {:?}", new_mo_queue);
    // println!("cancel_lo_queue: {:?}", cancel_lo_queue);

    // ------------------------------------------------ Event generation simulation -- //
    // ------------------------------------------------ --------------------------- -- //

    let mut vec_variants: Vec<MarketEventType> = vec![];

    for _ in 1..4 {
        vec_variants.push(MarketEventType::random_variants(1)[0].clone());
    }

    // let iter_variants = &vec_variants;

    println!("\ngenerated variants were: {:?}\n", &vec_variants);

    let mut single_queue: Vec<MarketEvent> = vec![];

    for i_variant in vec_variants {
        match i_variant {
            MarketEventType::NewMarketOrder => {
                single_queue.push(templates::random_new_mo_template().unwrap());
            }

            MarketEventType::CancelLimitOrder => {
                single_queue.push(templates::random_cancel_lo_template().unwrap());
            }

            MarketEventType::NewLimitOrder => {
                single_queue.push(templates::random_new_lo_template().unwrap());
            }

            MarketEventType::ModifyLimitOrder => {
                single_queue.push(templates::random_modify_lo_template().unwrap());
            }
        }
    }

    println!("\nThe single_queue has: {:?}\n", single_queue);

    // ------------------------------------------------------- Execution simulation -- //
    // ------------------------------------------------------- -------------------- -- //

    for i in 1..=single_queue.len() {
        println!(
            "\n This is the {:?} of {:?} total events in the queue\n",
            i,
            single_queue.len()
        );

        let i_event = single_queue.pop().unwrap();

        match i_event.event_info.event_type {
            message::MarketEventType::CancelLimitOrder => {
                let i_event_content = i_event.event_content;
                // println!("\nOrder to be inserted: {:?}\n", &i_event_order);

                if let message::EventContent::OrderCancellation(i_order_id) = i_event_content {
                    println!("Order created: {:?}", &i_order_id);
                }
            }

            _ => {}
        }
    }
}

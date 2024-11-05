use atelier::data::market::{Level, Order, OrderType, Orderbook, Side};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Parameters for synthetic orderbook generation
    let bid_price = 50_000.00;
    let ask_price = 50_100.00;
    let tick_size = 100.0;
    let n_levels = 2;
    let n_orders = 2;

    // Generate a synthetic orderbook for testing
    // this determines the order of the Level objects within
    // the bids and asks vectors, and also, the order of each
    // order within each level.

    let mut i_ob = Orderbook::synthetize(bid_price, ask_price, tick_size, n_levels, n_orders);

    // Generated Orderbook
    // println!("{:?}", i_ob);

    // ------------------------------------------------------- Find Level -- //
    // ------------------------------------------------------- ---------- -- //

    let price_level = 50_200.0;
    let find_level_ob = i_ob.find_level(&price_level);

    println!("\n -- Find Level --");

    match find_level_ob {
        Ok(n) if n < 0 => {
            let bid_found = find_level_ob.unwrap().abs() as usize - 1;

            println!(
                "
                The search price: {:?} is in the bid side, 
                with an index: {:?},
                with a price level: {:?},
                with {:?} orders.",
                &price_level,
                i_ob.bids[bid_found].level_id,
                i_ob.bids[bid_found].price,
                i_ob.bids[bid_found].orders.len()
            )
        }

        Ok(n) if n > 0 => {
            let ask_found = find_level_ob.unwrap() as usize - 1;

            println!(
                "
                The price: {:?} is in the ask side,
                with an index: {:?},
                with a price level: {:?},
                with {:?} orders",
                &price_level,
                i_ob.asks[ask_found].level_id,
                i_ob.asks[ask_found].price,
                i_ob.asks[ask_found].orders.len()
            )
        }
        Err(e) => println!("Error encountered : {:?}", e),
        Ok(_) => println!("Error not mapped"),
    }

    // --------------------------------------------------- Retrieve Level -- //
    // --------------------------------------------------- -------------- -- //

    let find_this: f64 = 50_200.0;
    let content_ob_level = i_ob.retrieve_level(&find_this).unwrap();

    println!("\n -- Retrieve Level --");
    println!(
        "
        level to be retrieved: {:?},
        retrieved Level index: {:?},
        retrieved Level price: {:?}, 
        retrieved Level orders: {:?}",
        find_this,
        content_ob_level.level_id,
        content_ob_level.price,
        content_ob_level.orders.len(),
    );

    // ----------------------------------------------------- Delete Level -- //
    // ----------------------------------------------------- ------------ -- //

    println!("\n -- Delete Level --");
    let delete_this: f64 = 50_200.0;
    println!(
        " 
        Delete the level with this price: {:?}",
        &delete_this
    );
    i_ob.delete_level(&delete_this).unwrap();
    println!("\nNew state of the OB.bids: \n{:?}", i_ob.bids);
    println!("\nNew state of the OB.asks: \n{:?}", i_ob.asks);

    // ----------------------------------------------------- Insert Level -- //
    // ----------------------------------------------------- ------------ -- //

    println!("\n -- Insert Level --");
    println!("");

    let new_order = Order {
        order_id: 123,
        order_ts: 456,
        order_type: OrderType::Limit,
        side: Side::Asks,
        price: 50_200.0,
        amount: 0.123,
    };

    let new_level = Level {
        level_id: 123,
        side: Side::Asks,
        price: 50_200.0,
        volume: 0.987,
        orders: vec![new_order],
    };

    let insert_this: &Level = &new_level;
    println!(
        "
        Level to be inserted: {:?}",
        &insert_this
    );

    println!("\nResult of insertion: {:?}", i_ob.insert_level(new_level));

    // let find_this: &f64 = &new_level.price;
    println!("i_ob content: {:?}", i_ob);

    println!("\n -- Retrieve Level --");
    println!(
        "
        NEW level to be retrieved: {:?},
        NEW retrieved Level index: {:?},
        NEW retrieved Level price: {:?}, 
        NEW retrieved Level orders: {:?}",
        find_this,
        content_ob_level.level_id,
        content_ob_level.price,
        content_ob_level.orders.len()
    );

    // ------------------------------------------------------- Find Order -- //
    // ------------------------------------------------------- ---------- -- //

    println!("\n -- Find Order --");
    let i_order = &i_ob.bids[0].orders[0];
    let found_order = i_ob.find_order(i_order.side, i_order.price, i_order.order_ts);

    println!(
        "
        Order to be found,
        side: {:?},
        price: {:?},
        order_ts: {:?}",
        i_order.side, i_order.price, i_order.order_ts,
    );

    println!("\nOrder found: {:?}", found_order.unwrap());

    // --------------------------------------------------- Retrieve Order -- //
    // --------------------------------------------------- -------------- -- //

    println!("\n -- Retrieve Order --");
    let i_order = &i_ob.bids[0].orders[0];

    let retrieved_order = i_ob.retrieve_order(i_order.side, i_order.price, i_order.order_ts);

    println!(
        "
        Order to be found,
        side: {:?},
        price: {:?},
        order_ts: {:?}",
        i_order.side, i_order.price, i_order.order_ts,
    );

    println!("Retrieved Order: {:?}", retrieved_order);

    // ----------------------------------------------------- Delete Order -- //
    // ----------------------------------------------------- ------------ -- //

    println!("\n -- Delete Order --");
    let to_delete_order = i_ob.bids[0].orders[0].clone();

    println!(
        "
        Order to be deleted,
        side: {:?},
        price: {:?},
        amount: {:?},
        order_ts {:?}",
        to_delete_order.side,
        to_delete_order.price,
        to_delete_order.amount,
        to_delete_order.order_ts,
    );

    println!("\nPrevious orderbook.bids has :\n{:?}", i_ob.bids);

    let _deleted = i_ob.delete_order(
        to_delete_order.side,
        to_delete_order.price,
        to_delete_order.order_ts,
    );

    println!("\nNow orderbook.bids has :\n{:?}", i_ob.bids);

    // ----------------------------------------------------- Insert Order -- //
    // ----------------------------------------------------- ------------ -- //

    println!("\n -- Insert Order --");
    let inserted_order = i_ob.insert_order(Side::Bids, 50_200.0, 0.1986);
    match inserted_order {
        Ok(result) => {
            println!("This is the result: {:?}", result);
        }
        Err(result) => {
            eprintln!("This was the error: {:?}", result);
        }
    }

    // ----------------------------------------------------- Modify Order -- //
    // ----------------------------------------------------- ------------ -- //

    println!("\n -- Modify Order --");
    let to_modify_order = i_ob.asks[0].orders[0].clone();

    let moded = i_ob.modify_order(
        to_modify_order.order_ts,
        to_modify_order.side,
        to_modify_order.price,
        999.999,
    );

    println!("moded_order: {:?}", moded.unwrap());
}

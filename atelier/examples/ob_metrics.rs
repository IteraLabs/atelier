use atelier::data::market::{Level, Order, OrderType, Orderbook, Side};

fn main() {

    // Parameters for synthetic orderbook generation
    let bid_price = 50_000.00;
    let ask_price = 50_100.00;
    let tick_size = 100.0;
    let n_levels = 2;
    let n_orders = 1;

    // Generate a synthetic orderbook for testing
    // this determines the order of the Level objects within 
    // the bids and asks vectors, and also, the order of each
    // order within each level.
    
    let mut i_ob = Orderbook::synthetize(
        bid_price, 
        ask_price, 
        tick_size, 
        n_levels, 
        n_orders);

    // Generated Orderbook
    // println!("{:?}", i_ob);
    
    // ------------------------------------------------------- Find Level -- //
    // ------------------------------------------------------- ---------- -- //
    
    let price_level = 50_200.0;
    let find_level_ob = i_ob.find_level(&price_level);

    println!("\n -- Find Level --");

    match find_level_ob {

        Ok(n) if n < 0 => {
    
            let bid_found = find_level_ob.unwrap().abs() as usize -1;

            println!("
                The search price: {:?} is in the bid side, 
                with an index: {:?},
                with a price level: {:?},
                with {:?} orders.",
                &price_level,
                i_ob.bids[bid_found].level_id,
                i_ob.bids[bid_found].price,
                i_ob.bids[bid_found].orders.len())
        },
        
        Ok(n) if n > 0 => {
        
            let ask_found = find_level_ob.unwrap() as usize -1 ;
            
            println!("
                The price: {:?} is in the ask side,
                with an index: {:?},
                with a price level: {:?},
                with {:?} orders", 
                &price_level,
                i_ob.asks[ask_found].level_id,
                i_ob.asks[ask_found].price,
                i_ob.asks[ask_found].orders.len())
        },
        Err(e) => println!("Error encountered : {:?}", e),
        Ok(_) => println!("Error not mapped"),
    }
    
    // --------------------------------------------------- Retrieve Level -- //
    // --------------------------------------------------- -------------- -- //

    let find_this: f64 = 50_200.0;
    let content_ob_level = i_ob.retrieve_level(&find_this).unwrap();

    println!("\n -- Retrieve Level --");
    println!("
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
    println!(" 
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
    println!("
        Level to be inserted: {:?}",
        &insert_this
    );

    println!("\nResult of insertion: {:?}", i_ob.insert_level(new_level));
    
    // let find_this: &f64 = &new_level.price;
    println!("i_ob content: {:?}", i_ob);

    println!("\n -- Retrieve Level --");
    println!("
        NEW level to be retrieved: {:?},
        NEW retrieved Level index: {:?},
        NEW retrieved Level price: {:?}, 
        NEW retrieved Level orders: {:?}",
        find_this,
        content_ob_level.level_id,
        content_ob_level.price,
        content_ob_level.orders.len());
    
    /*
    let new_order: Order = Order::new(123, 123, OrderType::Limit,
        Side::Bids, 50_000.00, 123.123);
    let insert_new = i_ob.insert_level(Side::Bids, 49_900.0, 123.123,
        vec![new_order]);
    */

    // let r_result = i_ob.delete_level(50_000.01);
    // println!("{:?}", r_result);

    // println!("{:?}", insert_new);

    // get the TOB
    // let tob_data = i_ob.get_tob();
    // println!("TOB: {:?}", tob_data);

    // extract tob values
    // let tob_bid: f64 = i_ob.bids[0].price;
    // let tob_ask: f64 = i_ob.asks[0].price;

    // Compute the Spread
    // let spread_value = Spread::compute(&tob_bid, &tob_ask, 0);
    // println!("Spread: {:?}", spread_value);

    // Compute the Midprice
    // let midprice_value = Midprice::compute(&tob_bid, &tob_ask, 0);
    // println!("Midprice: {}", midprice_value);

    // Compute the Volume Imbalance
    // let iter_bids: Vec<f64> = i_ob.bids.clone().into_iter().map(|x| x.volume).collect();
    // let iter_asks: Vec<f64> = i_ob.asks.clone().into_iter().map(|x| x.volume).collect();

    // let obimb_value = VolumeImbalance::compute(&iter_bids, &iter_asks, 1);
    // println!("Volume Imbalance: {:?}", obimb_value);

    // Compute the Volume-Weighted Average Price
    //let iter_bids: Vec<_> = i_ob
    //     .bids
    //    .into_iter()
    //    .map(|x| vec![x.price, x.volume])
    //   .collect();
    // let iter_asks: Vec<_> = i_ob
    //    .asks
    //    .into_iter()
    //    .map(|x| vec![x.price, x.volume])
    //    .collect();

    // Compute the VWAP
    // let vwap_value = VWAP::compute(&iter_bids.clone(), &iter_asks.clone(), 1);
    // println!("VWAP: {:?}", vwap_value);
}


use atelier_data::orders::{OrderType, OrderSide, Order};

// -------------------------------------------------------------------------------------------- //

/*

Steps to create an Order-events streaming client

event generation : random orders with fixed parameters
event message build : 
connectivity preparations to channel : 
send message : 

*/

pub fn main() {

    let _my_user_id = 777;
    // let i_market_event_type = MarketEventType::NewLimitOrder; 
    /*
    let i_event = event_info {
        event_id: 
        event_received_ts: 
        event_type: 
        user_id: 
    }
    let event_content = 
    MarketEvent { 
        event_info: 
        event_content: 
    } 
    */
    

    loop {

        let r_order_type = OrderType::random();
        let r_order_side = OrderSide::random();
        let r_order_prices = Some((100_000.01, 100_001.00));
        let r_order_amounts = Some((0.01, 0.05));

        let r_order = Order::random(
            r_order_type,
            r_order_side,
            r_order_prices,
            r_order_amounts
        );
        
        println!("Generated event: {:?}", r_order);
    
    }

}


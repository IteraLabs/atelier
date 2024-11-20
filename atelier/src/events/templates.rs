/// Market event generator module

use crate::data::market;
use crate::generators;
use crate::messages::errors;

use std::time::{SystemTime, UNIX_EPOCH};
use rand::seq::SliceRandom;
use rand::thread_rng;

// ---------------------------------------------------------------------- EventType -- //
// ---------------------------------------------------------------------- --------- -- //

#[macro_export]
macro_rules! enum_create {
    ($enum_name:ident, $($variant:ident),+) => {
        
        #[derive(Debug, Clone, PartialEq, PartialOrd)]

        pub enum $enum_name {
            $($variant),+
        }

        impl $enum_name {
            
            fn variants() -> Vec<Self> {
                vec![$(Self::$variant),+]
            }
            
            fn random_variants(n_choice:usize) -> Vec<Self> {
                let mut rng = thread_rng();
                Self::variants()
                .choose_multiple(&mut rng, n_choice)
                .cloned()
                .collect()
            }
        }
    }
}

// -- Instantiate a MarketEvent Type Enum -- //

enum_create!(MarketEventType,
    CancelLimitOrder,
    NewMarketOrder,
    ModifyLimitOrder,
    NewLimitOrder);

// ------------------------------------------------------- Market Event Data Struct -- //
// ------------------------------------------------------- ------------------------ -- //

#[derive(Debug)]
pub struct EventData {
    pub event_created_ts: u128,
    pub event_executed_ts: u128,
    pub event_type: MarketEventType,
    pub user_id:u32,
}

impl EventData {
    
    pub fn new(
        event_created_ts: u128,
        event_executed_ts: u128,
        event_type: MarketEventType,
        user_id: u32) -> Self {
    
        EventData {
            event_created_ts,
            event_executed_ts,
            event_type,
            user_id,
        }
    }
}

// ---------------------------------------------------- Market Event Content Struct -- //
// ---------------------------------------------------- --------------------------- -- //

#[derive(Debug)]
pub struct EventContent {
    pub event_object: market::Order
}

impl EventContent {

    pub fn new(
        event_object: market::Order
    ) -> Self {
        EventContent {
            event_object
        }
    }
}

#[derive(Debug)]
pub struct MarketEvent {
    pub event_data: EventData,
    pub event_content: EventContent,
}

// ------------------------------------------------------------ Market Event Struct -- //
// ------------------------------------------------------------ ------------------- -- //

impl MarketEvent {
    pub fn new(
        event_data: EventData,
        event_content: EventContent,
    ) -> Self {
        MarketEvent {
            event_data,
            event_content,
        }
    }
}

// ------------------------------------------------ Template for Cancel Limit Order -- //
// ------------------------------------------------ ------------------------------- -- //

/// To create a pseudo-random Cancel Limit Order event
pub fn random_cancel_lo() -> Result<MarketEvent, errors::EventError> {

    let current_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u128;
    
    // -- random event data -- //
    
    let i_event_created_ts = current_ts;
    let i_event_executed_ts = current_ts + 1; 
    let i_event_type = MarketEventType::CancelLimitOrder;
    let i_user_id = 321;
    
    let i_event_data = EventData::new(
        i_event_created_ts,
        i_event_executed_ts,
        i_event_type,
        i_user_id,
    );

    // -- random event content -- //
    
    let i_order_id = 345;
    let i_order_ts = current_ts;
    let i_order_type = market::OrderType::Limit;
    let i_order_side = market::Side::random();
    let i_order_price = 70_000.00;
    let i_order_amount = 10.01;

    let i_order = market::Order::new(
        i_order_id,
        i_order_ts,
        i_order_type,
        i_order_side,
        i_order_price,
        i_order_amount);

    let i_event_content = EventContent::new(i_order);

    let r_market_event = MarketEvent::new(
        i_event_data,
        i_event_content
    );
    
    // returns the message {event data, event content}
    Ok(r_market_event)

}

// -------------------------------------------------- Template for New Market Order -- //
// -------------------------------------------------- ----------------------------- -- //

pub fn random_new_mo() -> Result<MarketEvent, errors::EventError> {

    let current_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u128;
    
    // -- random event data -- //
    
    let i_event_created_ts = current_ts;
    let i_event_executed_ts = current_ts + 1; 
    let i_event_type = MarketEventType::NewMarketOrder;
    let i_user_id = 654;
    
    let i_event_data = EventData::new(
        i_event_created_ts,
        i_event_executed_ts,
        i_event_type,
        i_user_id,
    );

    // -- random event content -- //
    
    let i_order_id = 012;
    let i_order_ts = current_ts;
    let i_order_type = market::OrderType::Market;
    let i_order_side = market::Side::random();
    let i_order_price = 70_100.00;
    let i_order_amount = 01.01;

    let i_order = market::Order::new(
        i_order_id,
        i_order_ts,
        i_order_type,
        i_order_side,
        i_order_price,
        i_order_amount);

    let i_event_content = EventContent::new(i_order);

    let r_market_event = MarketEvent::new(
        i_event_data,
        i_event_content
    );
    
    // returns the message {event data, event content}
    Ok(r_market_event)

}

// ------------------------------------------------ Template for Modify Limit Order -- //
// ------------------------------------------------ ------------------------------- -- //

// TODO: implement random_modify_mo

// --------------------------------------------------- Template for New Limit Order -- //
// --------------------------------------------------- ---------------------------- -- //

// TODO: implement random_new_lo

// -- re-export macro 
pub use crate::enum_create as enum_create;


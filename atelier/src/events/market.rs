/// Market event generator module

use crate::data::market;
use crate::generators;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum EventType {
    NewLimitOrder(market::Order),
    NewMarketOrder(market::Order),
    ModifyLimitOrder {
        order_id: u32,
        amount: f64,
    },
    CancelLimitOrder{
        order_id: u32,
    },
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct MarketEvent {
    event_timestamp: u128,
    event_type: EventType,
    event_id: u32,
}

pub struct EventGenerator;

impl EventGenerator {

    fn new() -> Self {
    }

    // ------------------------------------------------------------ New Limit Order -- //
    // ------------------------------------------------------------ --------------- -- //
    
    gen_new_limit_order(&mut self) -> Self {

    }

    // --------------------------------------------------------- Modify Limit Order -- //
    // --------------------------------------------------------- ------------------ -- //
    
    gen_new_market_order() -> Self {

    }

    // --------------------------------------------------------- Cancel Limit Order -- //
    // --------------------------------------------------------- ------------------ -- //
    
    gen_modify_order() -> Self {
    
    }

    // ----------------------------------------------------------- New Market Order -- //
    // ----------------------------------------------------------- ---------------- -- //
    
    gen_cancel_limit_order() -> Self {

    }

}


/// Market event generator module
use crate::results::errors;
use atelier_data::orders::Order;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::{SystemTime, UNIX_EPOCH};

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

            pub fn variants() -> Vec<Self> {
                vec![$(Self::$variant),+]
            }

            pub fn random_variants(n_choice:usize) -> Vec<Self> {
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

enum_create!(
    MarketEventType,
    CancelLimitOrder,
    NewMarketOrder,
    ModifyLimitOrder,
    NewLimitOrder
);

// ------------------------------------------------------- Market Event Info Struct -- //
// ------------------------------------------------------- ------------------------ -- //

#[derive(Debug, PartialEq, PartialOrd)]
pub struct EventInfo {
    pub event_id: u32,
    pub event_received_ts: u128,
    pub event_type: MarketEventType,
    pub user_id: u32,
}

impl EventInfo {
    pub fn new(
        event_id: u32,
        event_received_ts: u128,
        event_type: MarketEventType,
        user_id: u32,
    ) -> Self {
        EventInfo {
            event_id,
            event_received_ts,
            event_type,
            user_id,
        }
    }
}

// ---------------------------------------------------- Market Event Content Struct -- //
// ---------------------------------------------------- --------------------------- -- //

/// OrderEvent
///
/// The necessary contents are dependent of the type of the order event,
/// there is a one to one requirement for every type of order event:
/// CancelLimitOrder requires only u32
/// NewMarketOrder requires the actual market order,
/// NewLimitOrder requires the actual limit order,
/// ModifyLimitOrder requires a tupple of order_id and the order_amount.

#[derive(Debug, PartialEq, PartialOrd)]
pub enum EventContent {
    CancelLimitOrder(u32),
    NewMarketOrder(Order),
    ModifyLimitOrder(Order),
    NewLimitOrder(Order),
}

// ------------------------------------------------------------ Market Event Struct -- //
// ------------------------------------------------------------ ------------------- -- //

#[derive(Debug)]
pub struct MarketEvent {
    pub event_info: EventInfo,
    pub event_content: EventContent,
}

impl MarketEvent {

    pub fn new(event_info: EventInfo, event_content: EventContent) -> Self {
        MarketEvent {
            event_info,
            event_content,
        }
    }

}

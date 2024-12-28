use crate::data::market;
use crate::events::message;
use crate::generators;
use crate::results::errors;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::{SystemTime, UNIX_EPOCH};

// ------------------------------------------------ Template for Cancel Limit Order -- //
// ------------------------------------------------ ------------------------------- -- //

/// To create a pseudo-random Cancel Limit Order event
pub fn random_cancel_lo_template() -> Result<message::MarketEvent, errors::EventError> {
    // -- random event info -- //
    let random_received_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u128;
    let random_event_type = message::MarketEventType::CancelLimitOrder;
    let random_user_id = 123;
    let random_event_id = 987;

    // -- random event content -- //
    let random_order_id: u32 = 123;
    let i_event_info = message::EventInfo::new(
        random_event_id,
        random_received_ts,
        random_event_type,
        random_user_id,
    );

    let i_event_content = message::EventContent::OrderCancellation(random_order_id);

    // -- market event formation -- //
    let r_market_event = message::MarketEvent::new(i_event_info, i_event_content);

    // returns the message {event data, event content}
    Ok(r_market_event)
}

// -------------------------------------------------- Template for New Market Order -- //
// -------------------------------------------------- ----------------------------- -- //

pub fn random_new_mo_template() -> Result<message::MarketEvent, errors::EventError> {
    let current_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u128;

    // -- random event data -- //

    let i_event_created_ts = current_ts;
    let i_event_executed_ts = current_ts + 1;
    let i_event_type = message::MarketEventType::NewMarketOrder;
    let i_user_id = 654;
    let i_event_id = 987;

    let i_event_data =
        message::EventInfo::new(i_event_id, i_event_created_ts, i_event_type, i_user_id);

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
        i_order_amount,
    );

    let i_event_content = message::EventContent::OrderCreation(i_order);

    let r_market_event = message::MarketEvent::new(i_event_data, i_event_content);

    // returns the message {event data, event content}
    Ok(r_market_event)
}

// ------------------------------------------------ Template for Modify Limit Order -- //
// ------------------------------------------------ ------------------------------- -- //

pub fn random_modify_lo_template() -> Result<message::MarketEvent, errors::EventError> {
    let current_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u128;

    // -- random event data -- //

    let i_event_created_ts = current_ts;
    let i_event_type = message::MarketEventType::ModifyLimitOrder;
    let i_user_id = 654;
    let i_event_id = 987;

    let i_event_data =
        message::EventInfo::new(i_event_id, i_event_created_ts, i_event_type, i_user_id);

    // -- random event content -- //

    let i_order_id = 012;
    let i_order_amount = 01.01;

    let i_event_content = message::EventContent::OrderModification(i_order_id, i_order_amount);
    let r_market_event = message::MarketEvent::new(i_event_data, i_event_content);

    // returns the message {event data, event content}
    Ok(r_market_event)
}

// --------------------------------------------------- Template for New Limit Order -- //
// --------------------------------------------------- ---------------------------- -- //

pub fn random_new_lo_template() -> Result<message::MarketEvent, errors::EventError> {
    let current_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u128;

    // -- random event data -- //

    let i_event_created_ts = current_ts;
    let i_event_type = message::MarketEventType::NewLimitOrder;

    // TODO: Hash value for user_id + Create a list of users
    let i_user_id = 654;
    let i_event_id = 987;

    let i_event_data =
        message::EventInfo::new(i_event_id, i_event_created_ts, i_event_type, i_user_id);

    // -- random event content -- //

    // TODO: Hash value for order_id
    let i_order_id = 012;

    let i_order_ts = current_ts;
    let i_order_type = market::OrderType::Limit;
    let i_order_side = market::Side::random();

    // perhaps pass these two
    let i_order_price = 70_300.00;
    let i_order_amount = 01.666;

    let i_order = market::Order::new(
        i_order_id,
        i_order_ts,
        i_order_type,
        i_order_side,
        i_order_price,
        i_order_amount,
    );

    let i_event_content = message::EventContent::OrderCreation(i_order);
    let r_market_event = message::MarketEvent::new(i_event_data, i_event_content);
    Ok(r_market_event)
}

// -- re-export macro
pub use crate::enum_create;

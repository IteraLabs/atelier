use std::time::{SystemTime, UNIX_EPOCH};

use atelier_data::{
    orders::{Order, OrderSide, OrderType},
    trades::{Trade, TradeSide, TradeType},
    };

pub struct InventoryMetrics {
    trades_inventory: Vec<Trade>,
    base_inventory: f64,
    quote_inventory: f64,
    gas_inventory: f64,
}

pub struct FinancialMetrics {
    cumulative_fees: f64,
    cumulative_trades: u32,
    cumulative_traded_quote: f64,
    cumulative_traded_base: f64
}

pub struct DynamicSpread {}
pub struct ActiveTrades {}
pub struct RefPrice {}


// ------------------------------------------------------------------------- ORDER FORMATION -- //
// ------------------------------------------------------------------------- --------------- -- //

/// Store an Order's values. Supports hybrid logic for DEX-CEX
pub fn order_formation(
    order_price: f64,
    order_amount: f64,
    order_side: OrderSide,
    order_type: OrderType
) -> Order {

    // TODO: let order_fee = fee_model(priority_gas_fee)

    let order_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros() as u64;
    
    let i_order: Order = Order::builder()
        .side(order_side)
        .order_type(order_type)
        .order_ts(order_ts)
        .price(order_price)
        .amount(order_amount)
        .build()
        .expect("Failed new order creation in order_formation");

    i_order
}

// ------------------------------------------------------------------------------- ORDER FILL -- //
// ------------------------------------------------------------------------------- ---------- -- //

/// Simulated perfect order execution
pub fn order_fill(i_order: Order) -> Trade {

    let trade_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros() as u64;

   let trade_side = match i_order.side {
        OrderSide::Bids => TradeSide::Buy,
        OrderSide::Asks => TradeSide::Sell,
    }; 

    let trade_type = TradeType::Dex;

    let i_trade: Trade = Trade::builder()
        .side(trade_side)
        .trade_type(trade_type)
        .trade_ts(trade_ts)
        .price(i_order.price.unwrap())
        .amount(i_order.amount.unwrap())
        .build()
        .expect("Failed new trade from order_fill");

    i_trade

}

/// Data structure to expand with calculated values
pub fn inventory_metrics_update(i_trade: Trade) -> InventoryMetrics {

    // trades_inventory += (i_trade)
    // base_inventory += i_trade[order_amount]
    // quote_inventory -= i_trade[order_price] * i_trade[order_amount]
    // gas_inventory -= trade_fee

    InventoryMetrics{}
}

/// Data structure to expand with calculated values
pub fn financial_metrics_update(i_trade: Trade) -> FinancialMetrics {

    // cumulative_fees += i_trade[trade_fee]
    // cumulative_trades += 1
    // cumulative_traded_quote += i_trade[order_amount] * i_trade[order_price]
    // cumulative_traded_base += i_trade[order_amount]
    
    FinancialMetrics{}
}

/// Data structure with a collection of active trades (will empty after a reset)
pub fn active_trades_update(i_trade: Trade) -> ActiveTrades {

    // - time_since_reset += 1 time units
    // - active_trades += (i_trade[order_side] * i_trade[order_amount], i_trade[order_price])

    ActiveTrades {}
}

/// Data point of a single value as price reference
pub fn ref_price_update(i_activetrades: ActiveTrades) -> RefPrice {
    // - ref_price = ref_price_model(active_trades[ref_history])
    RefPrice {}
}


// -- cycle -- //

fn main() {

let market_price: f64 = 100_000.0;
let n_progressions: u32 = 10;
let ref_price = market_price;
let ask_price = ref_price * (1 - ask_spread_model())

for i in 1..n_progressions {
        
    }

}

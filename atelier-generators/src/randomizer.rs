//! # Misc Random Generation

use atelier_data::orders::{Order, OrderType, OrderSide};

pub fn randomize_order(order_side: OrderSide, order_type: OrderType) -> Order {
    Order::random(order_type, order_side, None, None, None)
}

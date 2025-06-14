/// Order Examples
use atelier_data::orders::{Order, OrderSide, OrderType};

fn main() {
    let det_order: Order = Order::builder()
        .side(OrderSide::Bids)
        .order_type(OrderType::Limit)
        .price(10_000.0)
        .amount(0.1)
        .build()
        .expect("Failed new order creation");

    println!(
        "
       \ncall to: Order::builder():\n
       order_id: {:?}
       order_side: {:?}
       order_type: {:?}
       order_ts: {:?}
       order_price: {:?}
       order_amount: {:?}",
        det_order.order_id,
        det_order.side,
        det_order.order_type,
        det_order.order_ts,
        det_order.price,
        det_order.amount
    );

    let order_types = vec![OrderType::random(), OrderType::Limit, OrderType::Market];
    let order_sides = vec![OrderSide::random(), OrderSide::Bids, OrderSide::Asks];

    let i_order = Order::random(
        order_types[1],
        order_sides[1],
        (10_000.00, 11_000.00),
        (0.0, 0.1),
    );

    println!(
        "
        \ncall to: Order::random()\n
        i_order.side: {:?}
        i_order.order_type: {:?}
        i_order.order_ts: {:?}
        i_order.price: {:?}
        i_order.amount: {:?}\n
        generated order_id: {:?}
        decoded i_order with Order::decode_order_id: {:?}\n",
        i_order.unwrap().side,
        i_order.unwrap().order_type,
        i_order.unwrap().order_ts,
        i_order.unwrap().price,
        i_order.unwrap().amount,
        i_order.unwrap().order_id,
        Order::decode_order_id(i_order.unwrap().order_id),
    );
}

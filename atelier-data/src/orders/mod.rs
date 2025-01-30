
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

/// OrderSide
///
/// Enum for identification of either a buy or sell side
/// used to describe the Order side.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum OrderSide {
    Bids,
    Asks,
}

impl OrderSide {
    ///
    /// Creates a random choice of the Side enum variants, which currently
    /// has implemented: {Bids, Asks}
    ///
    pub fn random() -> Self {
        
        let now_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if now_ts % 2 == 0 {
            OrderSide::Bids
        } else {
            OrderSide::Asks
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum OrderType {
    Market,
    Limit,
}

impl OrderType {
    ///
    /// Creates a random choice of the OrderType enum variants, which currently
    /// has implemented: {Limit, Market} as variants.
    ///

    pub fn random() -> Self {
        let now_ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        if now_ts.as_secs() % 2 == 0 {
            OrderType::Limit
        } else {
            OrderType::Market
        }
    }
}

// ------------------------------------------------------------------------------------ ORDER -- //
// ------------------------------------------------------------------------------------ -------- //

/// Represents a single order in the Orderbook.
///
/// The `Order` struct contains details about an individual order, including
/// its unique identifier, timestamp, type, side (buy/sell), price, and amount.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Order {
    pub order_id: Option<u32>,
    pub order_ts: Option<u128>,
    pub order_type: Option<OrderType>,
    pub side: Option<OrderSide>,
    pub price: Option<f64>,
    pub amount: Option<f64>,
}

impl Order {
    /// Creates a new _empty_ instance of an `Order`.
    ///
    /// # Parameters
    ///
    /// - `order_id`: The unique identifier for the order.
    /// - `order_ts`: The timestamp for when the order was created.
    /// - `order_type`: The type of the order (e.g., `OrderType::Limit`).
    /// - `side`: The side of the order, either `Side::Bids` or `Side::Asks`.
    /// - `price`: The price at which the order is placed.
    /// - `amount`: The amount of the asset being ordered.
    ///

    pub fn new() -> Order {
        Order {
            order_id: None,
            order_ts: None,
            order_type: None,
            side: None,
            price: None,
            amount: None,
        }
    }

    pub fn order_id(mut self, order_id: u32) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn order_ts(mut self, order_ts: u128) -> Self {
        let default_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        self.order_ts = match Some(order_ts) {
            Some(order_ts) => Some(order_ts),
            None => Some(default_ts),
        };

        self
    }

    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }

    pub fn side(mut self, side: OrderSide) -> Self {
        self.side = Some(side);
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn random(
        order_type: OrderType,
        order_side: OrderSide,
        mo_amounts: Option<(f64, f64)>,
        lo_prices: Option<(f64, f64)>,
        lo_amounts: Option<(f64, f64)>,
    ) -> Self {
        let mut rng = rand::thread_rng();

        let i_order = Order::new()
            .order_id(rng.gen())
            .order_ts(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            )
            .side(order_side)
            .order_type(order_type);

        match i_order.order_type {
            Some(OrderType::Limit) => {
                if let Some(lo_prices) = lo_prices {
                    i_order.price(rng.gen_range(lo_prices.0..lo_prices.1));
                } else {
                    i_order.price(rng.gen_range(0.001..100_000.00));
                }

                if let Some(lo_amounts) = lo_amounts {
                    i_order.amount(rng.gen_range(lo_amounts.0..lo_amounts.1));
                } else {
                    i_order.amount(rng.gen_range(0.00001..1.0));
                }
            }

            Some(OrderType::Market) => {
                if let Some(mo_amounts) = mo_amounts {
                    i_order.amount(rng.gen_range(mo_amounts.0..mo_amounts.1));
                } else {
                    i_order.amount(rng.gen_range(0.00001..1.0));
                }
            }
            _ => {}
        }
        i_order.amount(rng.gen_range(0.1..100.0))
    }
}

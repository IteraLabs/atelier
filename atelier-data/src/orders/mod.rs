use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// OrderSide
///
/// Enum for identification of either a buy or sell side
/// used to describe the Order side.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum OrderSide {
    Bids,
    Asks,
}

impl OrderSide {
    ///
    /// Creates a random choice of the Side enum variants, which currently
    /// has implemented: {Bids, Asks}
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

/// OrderType
///
/// Enum for identification of the supported Order Types
/// currently Market and Limit.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
}

impl OrderType {
    ///
    /// Creates a random choice of the OrderType enum variants, which currently
    /// has implemented: {Limit, Market} as variants.
    pub fn random() -> Self {
        let now_ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        if now_ts.as_secs() % 2 == 0 {
            OrderType::Limit
        } else {
            OrderType::Market
        }
    }
}

/// OrderID
///
/// Method to generate unique Order ID values for individual orders.
/// currently taking the timestamp, the Order Type and the Order Side to deliver
/// a hashed u64 value.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct OrderId(u64);

impl OrderId {
    const TIME_MASK: u64 = 0x0FFF_FFFF_FFFF_FFFF;

    pub fn new(ts: u64, order_side: OrderSide, order_type: OrderType) -> Self {
        let mut val = ts & Self::TIME_MASK;
        val |= (order_side as u64) << 63;
        val |= (order_type as u64) << 62;
        OrderId(val)
    }

    pub fn timestamp(&self) -> u64 {
        self.0 & Self::TIME_MASK
    }

    pub fn side(&self) -> OrderSide {
        if self.0 >> 62 & 1 == 0 {
            OrderSide::Bids
        } else {
            OrderSide::Asks
        }
    }

    pub fn order(&self) -> OrderType {
        if self.0 >> 63 & 1 == 0 {
            OrderType::Market
        } else {
            OrderType::Limit
        }
    }
}

// -------------------------------------------------------------------------- ORDER -- //
// -------------------------------------------------------------------------- -------- //

#[derive(Debug, Copy, Clone)]
pub struct OrderBuilder {
    order_ts: Option<u64>,
    order_type: Option<OrderType>,
    side: Option<OrderSide>,
    price: Option<f64>,
    amount: Option<f64>,
}

impl OrderBuilder {
    pub fn new() -> Self {
        OrderBuilder {
            side: None,
            order_type: None,
            order_ts: None,
            price: None,
            amount: None,
        }
    }

    ///
    /// Builder pattern to create a randomly new instance of an `Order`
    ///
    /// definition is inherited from `Order::random()`
    pub fn random_new(
        r_order_type: OrderType,
        r_order_side: OrderSide,
        r_order_prices: (f64, f64),
        r_order_amounts: (f64, f64),
    ) -> Order {
        let mut rng = rand::rng();

        let r_order_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_micros() as u64;

        let r_order_price = rng.random_range(r_order_prices.0..r_order_prices.1);
        let r_order_amount = rng.random_range(r_order_amounts.0..r_order_amounts.1);

        let r_order_id = Order::encode_order_id(r_order_side, r_order_type, r_order_ts);

        Order {
            order_id: r_order_id,
            side: r_order_side,
            order_type: r_order_type,
            order_ts: r_order_ts,
            price: Some(r_order_price),
            amount: Some(r_order_amount),
        }
    }

    pub fn side(mut self, side: OrderSide) -> Self {
        self.side = Some(side);
        self
    }

    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }

    pub fn order_ts(mut self, order_ts: u64) -> Self {
        self.order_ts = Some(order_ts);
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

    ///
    /// Builder pattern to create a new instance of an `Order`.
    /// it requires to have the fields defined in the following order
    ///
    /// side: OrderSide
    /// order_type: OrderType
    /// order_ts: u64
    ///
    /// then it forms the `order_id` by calling the `encode_order_id`
    /// which will be a u64 formed with side, order_type, order_ts.
    pub fn build(self) -> Result<Order, &'static str> {
        let order_side = self.side.ok_or("Missing side")?;
        let order_type = self.order_type.ok_or("Missing order_type")?;
        let order_ts = self.order_ts.unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_micros() as u64
        });

        let order_id = Order::encode_order_id(order_side, order_type, order_ts);

        Ok(Order {
            order_id,
            order_ts,
            order_type,
            side: order_side,
            price: self.price,
            amount: self.amount,
        })
    }
}

/// Represents a single order in the Orderbook.
///
/// The `Order` struct contains details about an individual order, including
/// its unique identifier, timestamp, type, side (buy/sell), price, and amount.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Order {
    pub order_id: u64,
    pub order_ts: u64,
    pub order_type: OrderType,
    pub side: OrderSide,
    pub price: Option<f64>,
    pub amount: Option<f64>,
}

impl Order {
    pub fn builder() -> OrderBuilder {
        OrderBuilder::new()
    }

    /// A simple random `Order` generator.
    ///
    /// Uses a very naive approach. It requires:
    ///
    /// r_order_type: OrderType
    /// r_order_side: OrderSide
    /// r_order_prices: (f64, f64) with lower and upper boundary for Uniform
    /// random distribution r_order_amounts: (f64, f64) with lower and upper
    /// boundaries for Uniform random distribution
    pub fn random(
        order_type: OrderType,
        order_side: OrderSide,
        order_prices: (f64, f64),
        order_amounts: (f64, f64),
    ) -> Result<Order, &'static str> {
        Ok(OrderBuilder::random_new(
            order_type,
            order_side,
            order_prices,
            order_amounts,
        ))
    }

    /// Encoded Order ID formation
    ///
    /// The order_id field is an u64 containing encoded info about: side, type,
    /// timestamp. The Bit allocation is the following:
    ///
    /// 00TTSSSS SSSSSSSS SSSSSSSS SSSSSSSS SSSSSSSS SSSSSSSS SSSSSSSS SSSSSSSS
    /// ||└ 60 bits for timestamp (valid until ~2079)
    /// |└─ 1 bit for type (0=Market, 1=Limit)
    /// └── 1 bit for side (0=Bid, 1=Ask)
    pub fn encode_order_id(
        order_side: OrderSide,
        order_type: OrderType,
        order_ts: u64,
    ) -> u64 {
        // Highest bit
        let side_bit = match order_side {
            OrderSide::Bids => 0,
            OrderSide::Asks => 1,
        } << 63;
        // Second highest bit
        let type_bit = match order_type {
            OrderType::Market => 0,
            OrderType::Limit => 1,
        } << 62;
        // 60 bits starting at position 2
        let timestamp_bits = (order_ts & ((1 << 60) - 1)) << 2;

        side_bit | type_bit | timestamp_bits
    }

    /// Decode Order ID encoded formation. check `encoded_order_id` for more
    /// details.
    pub fn decode_order_id(order_id: u64) -> (OrderSide, OrderType, u64) {
        // Highest bit
        let order_side = if (order_id >> 63) & 1 == 0 {
            OrderSide::Bids
        } else {
            OrderSide::Asks
        };
        // Second highest bit
        let order_type = if (order_id >> 62) & 1 == 0 {
            OrderType::Market
        } else {
            OrderType::Limit
        };
        // 60 bits starting at position 2
        let order_ts = (order_id >> 2) & ((1 << 60) - 1);

        (order_side, order_type, order_ts)
    }
}

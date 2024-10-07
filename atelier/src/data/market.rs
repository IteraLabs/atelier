<<<<<<< HEAD

use crate::simulation::randomizer::randomize_order;
=======
use crate::generators::randomizer::randomize_order;
use rand::distributions::Uniform;
use std::time::{SystemTime, UNIX_EPOCH};
>>>>>>> origin/main

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Side {
    Bids,
    Asks,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum OrderType {
    Market,
    Limit,
}

// ---------------------------------------------------------------- ORDER -- //
// ------------------------------------------------------------------------- //

/// Represents a single order in the Orderbook.
///
/// The `Order` struct contains details about an individual order, including
/// its unique identifier, timestamp, type, side (buy/sell), price, and amount.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Order {
    pub order_id: u32,
    pub order_ts: u64,
    pub order_type: OrderType,
    pub side: Side,
    pub price: f64,
    pub amount: f64,
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
    pub fn new(
        order_id: u32,
        order_ts: u64,
        order_type: OrderType,
        side: Side,
        price: f64,
        amount: f64,
    ) -> Self {
        match side {
            Side::Bids => Order {
                order_id,
                order_ts,
                order_type,
                side: Side::Bids,
                price,
                amount,
            },
            Side::Asks => Order {
                order_id,
                order_ts,
                order_type,
                side: Side::Asks,
                price,
                amount,
            },
        };

        Order {
            order_id,
            order_ts,
            order_type,
            side,
            price,
            amount,
        }
    }

    /// Creates a new _random_ instance of an `Order`.
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
    pub fn randomize(side: Side, price: f64, order_type: OrderType) -> Self {
        // Randomize order_ts
        let now_ts = SystemTime::now();

        let since_epoch_ts = now_ts
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let order_ts_gen = since_epoch_ts as u64;

        // Randomize amount
        let order_amount_gen = 123.456;

        // Randomize order_id
        let order_id_gen: u32 = 123;

        // Parse Order Type
        let order_type_gen = order_type;

        Order {
            order_id: order_id_gen,
            order_ts: order_ts_gen,
            order_type: order_type_gen,
            side,
            price,
            amount: order_amount_gen,
        }
    }
}

// ---------------------------------------------------------------- LEVEL -- //
// ------------------------------------------------------------------------- //

/// Represents a price level in an order book.
///
/// The `Level` struct contains details about a specific price level, including
/// its unique identifier, side (buy/sell), price, total volume at that price,
/// and a vector of orders associated with that level.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Level {
    pub level_id: u32,
    pub side: Side,
    pub price: f64,
    pub volume: f64,
    pub orders: Vec<Order>,
}

impl Level {
    /// Creates a new instance of `Level`.
    ///
    /// # Parameters
    ///
    /// - `level_id`: The unique identifier for the price level.
    /// - `side`: The side of the order book, either `Side::Bids` or `Side::Asks`.
    /// - `price`: The price at which orders are placed at this level.
    /// - `volume`: The total volume of orders at this price level.
    /// - `orders`: A vector of `Order` representing the orders at this level.
    ///
    /// # Returns
    ///
    /// Returns a new `Level` instance with the specified parameters.
    pub fn new(level_id: u32, side: Side, price: f64, volume: f64, orders: Vec<Order>) -> Self {
        match side {
            Side::Bids => Level {
                level_id,
                side: Side::Bids,
                price,
                volume,
                orders: orders.clone(),
            },
            Side::Asks => Level {
                level_id,
                side: Side::Asks,
                price,
                volume,
                orders: orders.clone(),
            },
        };

        Level {
            level_id,
            side,
            price,
            volume,
            orders,
        }
    }
}

// ------------------------------------------------------------ ORDERBOOK -- //
// ------------------------------------------------------------------------- //

/// Represents a Limit Order Book for a specific market.
///
/// This `Orderbook` structure is different than 99.99% of other structs
/// within other Rust pojects, and that is the 3rd level of composition.
/// 1) Has both bids and asks sides (aham....)
/// 2) for each side, another Level struct with price, volume, etc (hemm ...)
/// 3) and for each Level, a queue (vector) of Order structs, (now we are talking)
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Orderbook {
    pub orderbook_id: u32,
    pub orderbook_ts: u64,
    pub symbol: String,
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
}

impl Orderbook {
    // ---------------------------------------------------- New Orderbook -- //
    // ------------------------------------------------------------------ -- //

    /// Creates a new instance of `Orderbook`.
    ///
    /// # Parameters
    ///
    /// - `orderbook_id`: The unique identifier for the order book.
    /// - `orderbook_ts`: The timestamp for the order book.
    /// - `symbol`: The trading symbol for the order book.
    /// - `bids`: A vector of `Level` representing the buy orders.
    /// - `asks`: A vector of `Level` representing the sell orders.
    ///
    /// # Returns
    ///
    /// Returns a new `Orderbook` instance.
    pub fn new(
        orderbook_id: u32,
        orderbook_ts: u64,
        symbol: String,
        bids: Vec<Level>,
        asks: Vec<Level>,
    ) -> Self {
        Orderbook {
            orderbook_id,
            orderbook_ts,
            symbol,
            bids,
            asks,
        }
    }

    // ------------------------------------------------------ Get the TOB -- //
    // ------------------------------------------------------ ----------- -- //

    pub fn get_tob(&self) -> Vec<&f64> {
        let bid_volume = self.bids.get(0).map(|bid| &bid.volume).unwrap_or(&0.0);
        let bid_price = self.bids.get(0).map(|bid| &bid.price).unwrap_or(&0.0);
        let ask_price = self.asks.get(0).map(|ask| &ask.price).unwrap_or(&0.0);
        let ask_volume = self.asks.get(0).map(|ask| &ask.volume).unwrap_or(&0.0);
        vec![&bid_volume, &bid_price, &ask_price, &ask_volume]
    }

    // ----------------------------------------------------- Find a Level -- //
    // ----------------------------------------------------- ------------ -- //

    pub fn find_level(&self, level_price: f64) -> Result<i32, i32> {
        // -1 : -n for indexes within the bid side
        // +1 : +n for indexes within the ask side
        // 0  : level does not exist

        if level_price <= self.bids[0].price {
            let mut i_level: i32 = 0;

            for i_bid in &self.bids {
                i_level -= 1;
                if level_price == i_bid.price {
                    return Ok(i_level);
                }
            }
        }

        if level_price >= self.asks[0].price {
            let mut i_level: i32 = 0;

            for i_ask in &self.asks {
                i_level += 1;
                if level_price == i_ask.price {
                    return Ok(i_level);
                }
            }
        }
        Ok(0)
    }

    // --------------------------------------- Retrieve an Existing Level -- //
    // --------------------------------------- -------------------------- -- //

    pub fn retrieve_level(&self, level_price: f64) -> Result<Level, i32> {
        // Validate the existence of the level.
        let i_level = self.find_level(level_price).unwrap();

        // On the Bid side
        if i_level < 0 {
            let i_level = i_level.abs() - 1;
            return Ok(self.bids[i_level as usize].clone());
        }

        // On the Ask side
        if i_level > 0 {
            let i_level = i_level - 1;
            return Ok(self.asks[i_level as usize].clone());
        } else {
            Err(0)
        }
    }

    // ----------------------------------------- Delete an Existing Level -- //
    // ----------------------------------------- ------------------------ -- //

    // ----------------------------------------------- Insert a New Level -- //
    // --------------------------------------------------- -------------- -- //

    // ---------------------------------------------------- Find an Order -- //
    // ---------------------------------------------------- ------------- -- //

    // --------------------------------------- Retrieve an Existing Order -- //
    // --------------------------------------- -------------------------- -- //

    // ----------------------------------------- Delete an Existing Order -- //
    // ----------------------------------------- ------------------------ -- //

    // ----------------------------------------------- Insert a New Order -- //
    // ----------------------------------------------- ------------------ -- //

    // ---------------------------------------------- Synthetic Orderbook -- //
    // ------------------------------------------------------------------ -- //

    /// Generates a synthetic order book with specified parameters.
    ///
    /// This method is useful for benchmarking and simulation purposes.
    ///
    /// # Parameters
    ///
    /// - `bid_price`: The starting price for the bids.
    /// - `ask_price`: The starting price for the asks.
    /// - `tick_size`: The minimum price increment between levels.
    /// - `n_levels`: Number of price levels to generate for both bids and asks.
    /// - `n_orders`: Number of individual orders to create at each price level.
    ///
    /// # Returns
    ///
    /// Returns a new `Orderbook` instance populated with synthetic bid and 
    /// ask levels.
    pub fn synthetize(
        bid_price: f64,
        ask_price: f64,
        tick_size: f64,
        n_levels: u32,
        n_orders: u32,
    ) -> Self {
        let mut i_bids = Vec::new();
        let mut i_asks = Vec::new();

        for i in 1..=n_levels {
            let i_bid_price = bid_price - (&tick_size * i as f64);
            let i_bid_side = Side::Bids;
            let i_order_type = OrderType::Limit;

            let mut v_bid_orders: Vec<Order> = (0..n_orders)
                .map(|_| randomize_order(i_bid_side, i_bid_price, i_order_type))
                .collect();

            v_bid_orders.sort_by_key(|order| order.order_ts);

            let i_bid_volume: f64 = v_bid_orders.iter().map(|order| order.amount).sum();

            i_bids.push(Level {
                level_id: i,
                side: i_bid_side,
                price: i_bid_price,
                volume: i_bid_volume,
                orders: v_bid_orders,
            });

            let i_ask_price = ask_price + (&tick_size * i as f64);
            let i_ask_side = Side::Asks;

            let mut v_ask_orders: Vec<Order> = (0..n_orders)
                .map(|_| randomize_order(i_ask_side, i_ask_price, i_order_type))
                .collect();

            v_ask_orders.sort_by_key(|order| order.order_ts);

            let i_ask_volume: f64 = v_ask_orders.iter().map(|order| order.amount).sum();

            i_asks.push(Level {
                level_id: i,
                side: i_ask_side,
                price: i_ask_price,
                volume: i_ask_volume,
                orders: v_ask_orders,
            });
        }

        Orderbook {
            orderbook_id: 123,
            orderbook_ts: 321,
            symbol: String::from("BTCUSDT"),
            bids: i_bids,
            asks: i_asks,
        }
    }
}

use crate::generators::randomizer::randomize_order;
use rand::distributions::Uniform;
use rand::Rng;

use crate::results::errors::{LevelError, OrderError};
use core::f64;
use std::task::Wake;
use std::time::{SystemTime, UNIX_EPOCH};

/// Side
///
/// Enum for identification of either a buy or sell side
/// used to describe the Order side.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Side {
    Bids,
    Asks,
}

impl Side {
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
            Side::Bids
        } else {
            Side::Asks
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
    pub side: Option<Side>,
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

    pub fn side(mut self, side: Side) -> Self {
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
            .side(Side::random())
            .order_type(OrderType::random());

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

// ------------------------------------------------------------------------------------ LEVEL -- //
// ------------------------------------------------------------------------------------ ----- -- //

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
    // ---------------------------------------------------------------------------- New Level -- //
    // ---------------------------------------------------------------------------- --------- -- //

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
// -------------------------------------------------------------------------------- ORDERBOOK -- //
// -------------------------------------------------------------------------------- --------- -- //

/// Represents a Limit Order Book for a specific market.
///
/// This `Orderbook` structure is different than 99.99% of other structs
/// within other Rust pojects, and that is the 3rd level of composition.
/// 1) Has both bids and asks sides (aham....)
/// 2) for each side, another Level struct with price, volume, etc (hemm ...)
/// 3) and for each Level, a queue (vector) of Order structs, (now we are talking)

/// ## Defaults
/// non-empty vectors : The default decision is to have empty vectors even
/// if there is no further to fill them with. e.g. for the bids/asks

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Orderbook {
    pub orderbook_id: u32,
    pub orderbook_ts: u64,
    pub symbol: String,
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
}

impl Orderbook {
    // ------------------------------------------------------------------------ New Orderbook -- //
    // ------------------------------------------------------------------------ ------------- -- //

    // Creates a new instance of `Orderbook`.
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
    ///
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

    // ------------------------------------------------------------------------- Find a Level -- //
    // ------------------------------------------------------------------------- ------------ -- //

    /// If a level exists, either within the Bids, or, the Asks,
    /// it will return the index of it, positive for asks, negative for bids.
    ///
    /// ## Parameters
    /// level_price: f64 = The Level's price to be found.
    ///
    /// ## Returns
    /// Ok(i32): Index of the Level found (if it exists),
    /// the sign encodes the side, negative (bids) and positive (asks)
    /// Err(LevelError): with LevelNotFound
    ///
    pub fn find_level(&self, level_price: &f64) -> Result<i32, LevelError> {
        let mut i_level: i32 = 0;

        if level_price <= &self.bids[0].price {
            for i_bid in &self.bids {
                i_level -= 1;
                if level_price == &i_bid.price {
                    return Ok(i_level);
                }
            }
        }

        if level_price >= &self.asks[0].price {
            for i_ask in &self.asks {
                i_level += 1;
                if level_price == &i_ask.price {
                    return Ok(i_level);
                }
            }
        }

        Err(LevelError::LevelNotFound)
    }

    // ----------------------------------------------------------- Retrieve an Existing Level -- //
    // ----------------------------------------------------------- -------------------------- -- //

    /// If a Level exists, either within the Bids, or, the Asks, it will
    /// return a _cloned()_ version of it.
    ///
    /// ## Parameters
    /// level_price: f64 : The level's price to be used as index.
    ///
    /// ## Returns
    /// Ok(Level) : A cloned version of the founded Level. \
    /// Err(LevelError): A custom error type as LevelError:LevelNotFound
    ///
    pub fn retrieve_level(&self, level_price: &f64) -> Result<Level, LevelError> {
        // return the level_price if it exists, or, LevelError::LevelNotFound
        if let Ok(i_level) = self.find_level(level_price) {
            // Level is on the Bid side
            if i_level < 0 {
                let i_level = i_level.abs() + 1;
                return Ok(self.bids[i_level as usize].clone());

            // Level is on the Ask side
            } else if i_level > 0 {
                let i_level = i_level - 1;
                return Ok(self.asks[i_level as usize].clone());

            // level is not present
            } else {
                return Err(LevelError::LevelInfoNotAvailable);
            }

        // find_level returned err in the first place
        } else {
            return Err(LevelError::LevelNotFound);
        }
    }

    // ------------------------------------------------------------- Delete an Existing Level -- //
    // ------------------------------------------------------------- ------------------------ -- //

    /// Deletes an existing level
    ///
    /// ## Parameters
    /// level_price: &f64

    pub fn delete_level(&mut self, level_price: &f64) -> Result<(), LevelError> {
        // see if level exists
        let find_level_ob = self.find_level(level_price);

        match find_level_ob {
            Ok(n) if n < 0 => {
                let bid_found = find_level_ob.unwrap().abs() as usize - 1;
                self.bids.remove(bid_found);
                Ok(())
            }

            Ok(n) if n > 0 => {
                let ask_found = find_level_ob.unwrap() as usize - 1;
                self.asks.remove(ask_found);
                Ok(())
            }

            Err(e) => Err(LevelError::LevelDeletionFailed),

            Ok(_) => Err(LevelError::LevelInfoNotAvailable),
        }
    }

    // ------------------------------------------------------------------- Insert a New Level -- //
    // ------------------------------------------------------------------- ------------------ -- //

    /// Inserts a new level. If the level already exists, the new level over
    /// rides the existing one, if it does not exists, the new level is inserted
    /// in its corresponding slot within the Vec<Level> for the corresponding
    /// side.
    ///
    /// ## Parameters
    /// level: With a Level::new()
    ///
    /// ## Returns
    /// Ok(Level)
    /// Err(LevelError): Custom Error Type of LevelInsertionFailed.
    ///
    pub fn insert_level(&mut self, level: Level) -> Result<(), LevelError> {
        // return the level_price if it exists, or, LevelError::LevelNotFound
        if let Ok(i_level) = self.find_level(&level.price) {
            println!("i_level found: {:?}", &i_level);

            // -- Level exist on the Bid side (to be replaced)
            if i_level < 0 {
                // updated counter to this side
                let i_level = i_level.abs() - 1;
                // use the same id for the level
                let same_level_id = &self.bids[i_level as usize].level_id;
                // override existing level with the new one
                self.bids[i_level as usize] = Level::new(
                    *same_level_id,
                    Side::Bids,
                    level.price,
                    level.volume,
                    level.orders,
                );

                return Ok(());

            // -- Level exist on the Ask side (to be replaced)
            } else if i_level > 0 {
                // update counter to this side
                let i_level = i_level - 1;
                // use the same id for the level
                let same_level_id = &self.asks[i_level as usize].level_id;
                // override existing level with new one
                self.asks[i_level as usize] = Level::new(
                    *same_level_id,
                    level.side,
                    level.price,
                    level.volume,
                    level.orders,
                );

                return Ok(());

            // A response was produced but with an error on level index
            } else {
                // find the right position within the vector
                // and insert new Level there

                println!("i_level found: {:?}", &i_level);

                return Err(LevelError::LevelNotFound);
            }
        // Level not found.
        // Localize index, insert into vector.
        } else {
            match level.side {
                Side::Bids => {
                    // Get the level_price for all levels in the vector
                    let mut v_bids = self.bids.clone().into_iter();

                    // Find position : Start with the upper most position
                    // (given the ordering in orders)
                    let index_level = v_bids
                        .position(|existing_level| level.price > existing_level.price)
                        .unwrap_or(v_bids.len());

                    // Insert new Level into the existing vector of levels.
                    self.bids.insert(
                        index_level,
                        Level::new(
                            index_level as u32,
                            level.side,
                            level.price,
                            level.volume,
                            level.orders,
                        ),
                    );
                    return Ok(());
                }

                Side::Asks => {
                    // get the level_price for all levels in the vector
                    let mut v_asks = self.asks.clone().into_iter();

                    // Find position : Start with the lower and top most position
                    // (given the ordering in orders)
                    let index_level = v_asks
                        .position(|existing_level| level.price < existing_level.price)
                        .unwrap_or(v_asks.len());

                    // Insert the new level into the existing vector of levels.
                    self.asks.insert(
                        index_level,
                        Level::new(
                            index_level as u32,
                            level.side,
                            level.price,
                            level.volume,
                            level.orders,
                        ),
                    );
                    return Ok(());
                }
            }
        }

        return Err(LevelError::LevelNotFound);
    }

    // ------------------------------------------------------------------------ Find an Order -- //
    // ------------------------------------------------------------------------ ------------- -- //

    /// To find if a given `Order` exists within the current Level.
    ///
    /// ## Parameters
    /// side: Side = {Side::Bids, Side::Asks}
    /// price: f64 = the order's price, which is the same as the Level's price
    /// order_ts: u64 = Order's timestamp
    ///
    /// ## Results
    /// Ok: (level_index: usize, order_index: usize)
    /// Err: OrderError
    ///

    pub fn find_order(
        &self,
        side: Side,
        price: f64,
        order_ts: u128,
    ) -> Result<(i32, usize), OrderError> {
        // see if level exists
        let find_level_ob = self.find_level(&price);
        match find_level_ob {
            Ok(n) if n < 0 => {
                let level_found = find_level_ob.unwrap().abs() as usize - 1;
                let level_orders = &self.bids[level_found].orders;

                // Level has orders
                if level_orders.len() > 0 {
                    let r_level = level_orders
                        .binary_search_by(|order| order.order_ts.unwrap().cmp(&order_ts))
                        .unwrap();

                    Ok((n, r_level))

                // Level is empty
                } else {
                    Err(OrderError::OrderNotFound)
                }
            }

            Ok(n) if n > 0 => {
                let level_found = find_level_ob.unwrap().abs() as usize - 1;
                let level_orders = &self.asks[level_found].orders;

                // Level has orders
                if level_orders.len() > 0 {
                    let r_level = level_orders
                        .binary_search_by(|order| order.order_ts.unwrap().cmp(&order_ts))
                        .unwrap();

                    Ok((n, r_level))

                // Level is empty
                } else {
                    Err(OrderError::OrderNotFound)
                }
            }

            Err(e) => Err(OrderError::OrderNotFound),
            Ok(_) => Err(OrderError::OrderInfoNotAvailable),
        }
    }

    // ----------------------------------------------------------- Retrieve an Existing Order -- //
    // ----------------------------------------------------------- -------------------------- -- //

    /// To retrieve info about an existing `Order`.
    ///
    /// ## Parameters
    /// Order
    ///
    /// ## Results
    ///  

    pub fn retrieve_order(
        &self,
        side: Side,
        price: f64,
        order_ts: u128,
    ) -> Result<(Order), OrderError> {
        if let Ok((found_level, found_order)) = self.find_order(side, price, order_ts) {
            // Get the curren timestamp
            let bid_ts = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();

            if found_level > 0 {
                Ok(self.asks[found_level.abs() as usize].orders[found_order])
            } else {
                Ok(self.bids[found_level.abs() as usize].orders[found_order])
            }
        } else {
            Err(OrderError::OrderNotFound)
        }
    }

    // ------------------------------------------------------------- Delete an Existing Order -- //
    // ------------------------------------------------------------- ------------------------ -- //

    /// To delete an existing `Order`.
    ///
    /// ## Parameters
    ///
    /// ## Results
    ///

    pub fn delete_order(
        &mut self,
        side: Side,
        price: f64,
        order_ts: u128,
    ) -> Result<(), OrderError> {
        if let Ok((found_level, found_order)) = self.find_order(side, price, order_ts) {
            if found_level > 0 {
                self.asks[found_level.abs() as usize - 1]
                    .orders
                    .remove(found_order);

                Ok(())
            } else {
                self.bids[found_level.abs() as usize - 1]
                    .orders
                    .remove(found_order);
                Ok(())
            }
        } else {
            Err(OrderError::OrderNotFound)
        }
    }

    // ------------------------------------------------------------------- Insert a New Order -- //
    // ------------------------------------------------------------------- ------------------ -- //

    /// To insert a new `Order`.
    ///
    /// The first process is to find whether the necessary Level in the
    /// Orderbook exists. Then, depending on which side is it (or has to be
    /// created) all the parameters for Order::new() are created and parsed.
    ///
    /// ## Parameters
    ///
    ///
    /// ## Results
    ///

    pub fn insert_order(&mut self, side: Side, price: f64, amount: f64) -> Result<(), OrderError> {
        // see if level exists
        let find_level_ob = self.find_level(&price);

        match find_level_ob {
            Ok(n) if n < 0 => {
                // Get the curren timestamp
                let bid_ts = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos();

                let i_order = Order::new()
                    .order_id(123)
                    .order_ts(bid_ts)
                    .order_type(OrderType::Limit)
                    .side(Side::Bids)
                    .price(self.bids[n as usize].price)
                    .amount(amount);

                self.bids[find_level_ob.unwrap() as usize]
                    .orders
                    .push(i_order);

                Ok(())
            }

            Ok(n) if n > 0 => {
                // Get the curren timestamp
                let ask_ts = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos();

                let i_order = Order::new()
                    .order_id(123)
                    .order_ts(ask_ts)
                    .order_type(OrderType::Limit)
                    .side(Side::Asks)
                    .price(self.asks[n as usize].price)
                    .amount(amount);

                self.asks[find_level_ob.unwrap() as usize]
                    .orders
                    .push(i_order);

                Ok(())
            }

            Err(e) => Err(OrderError::OrderNotFound),
            Ok(_) => Err(OrderError::OrderInfoNotAvailable),
        }
    }

    // ---------------------------------------------------------------------- Modify an Order -- //
    // ---------------------------------------------------------------------- --------------- -- //

    /// To modify an existing `Order`.
    ///
    /// ## Parameters
    ///
    /// ## Results
    ///

    pub fn modify_order(
        &mut self,
        order_ts: u128,
        side: Side,
        price: f64,
        amount: f64,
    ) -> Result<Order, OrderError> {
        match self.find_order(side, price, order_ts) {
            Ok((found_level, found_order)) => {
                if found_level < 0 {
                    println!("\nfounded level: {:?}", found_level.abs() as usize - 1);

                    let founded_order =
                        self.bids[found_level.abs() as usize].orders[found_order].clone();

                    println!("\nfounded order: {:?}", founded_order);

                    let moded_ts = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_nanos();

                    let founded_ts = founded_order.order_ts;

                    let to_moded_order = Order::new()
                        .order_id(founded_order.order_id.unwrap())
                        .order_ts(founded_ts.unwrap())
                        .order_type(founded_order.order_type.unwrap())
                        .side(founded_order.side.unwrap())
                        .price(founded_order.price.unwrap())
                        .amount(amount);

                    let moded_order = self.bids[found_level.abs() as usize - 1].orders[found_order];
                    let moded_order = to_moded_order;
                    Ok(moded_order.clone())
                } else if found_level > 0 {
                    println!("\nfounded_level: {:?}", found_level.abs() as usize - 1);

                    let founded_order =
                        self.asks[found_level.abs() as usize].orders[found_order].clone();

                    println!("founded order: {:?}", founded_order);

                    let moded_ts = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_nanos();

                    let founded_ts = founded_order.order_ts;

                    let to_moded_order = Order::new()
                        .order_id(founded_order.order_id.unwrap())
                        .order_ts(founded_ts.unwrap())
                        .order_type(founded_order.order_type.unwrap())
                        .side(founded_order.side.unwrap())
                        .price(founded_order.price.unwrap())
                        .amount(amount);

                    let moded_order = self.asks[found_level.abs() as usize - 1].orders[found_order];

                    let moded_order = to_moded_order;

                    Ok(moded_order.clone())
                } else {
                    println!("else");

                    Err(OrderError::OrderNotFound)
                }
            }
            Err(e) => Err(OrderError::OrderNotFound),
        }
    }

    // ------------------------------------------------------------------ Synthetic Orderbook -- //
    // ------------------------------------------------------------------ ------------------- -- //

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
    ///
    /// TODO: update this to be done with builder method.
    ///

    //
    // left and right interval values for:
    // tick sizes
    // no. of levels
    // no. of orders
    // 
    
    pub fn random(
        
        bids_price: f64,
        bids_orders: u32,
        bids_levels: u32,
        tick_size: f64,
        asks_price: f64,
        asks_orders: u32,
        asks_levels: u32,
    
    ) -> Self {
       
        // -- Default values -- //
        let mut i_bids = Vec::new();
        let mut i_asks = Vec::new();

        let r_orderbook_ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let r_orderbook_id = 1234;

        // -- Bid side level formation -- //
        for i in 1..=bids_levels {

            let mut v_bid_orders: Vec<Order> = (0..bids_orders)
                .map(|_| randomize_order(
                    Side::Bids,
                    bids_price - (&tick_size * i as f64),
                    OrderType::Limit))
                .collect();

            v_bid_orders.sort_by_key(|order| order.order_ts);

            let i_bid_volume: f64 = v_bid_orders
                .iter()
                .map(|order| order.amount.unwrap_or(0.0))
                .sum();

            i_bids.push(Level {
                level_id: i,
                side: Side::Bids,
                price: bids_price - (&tick_size * i as f64),
                volume: i_bid_volume,
                orders: v_bid_orders,
            });
        }

        // -- Ask side level formation -- //
        for i in 1..=asks_levels {

            let mut v_ask_orders: Vec<Order> = (0..asks_orders)
                .map(|_| randomize_order(
                    Side::Asks,
                    asks_price + (&tick_size * i as f64),
                    OrderType::Limit))
                .collect();

            v_ask_orders.sort_by_key(|order| order.order_ts);

            let i_ask_volume: f64 = v_ask_orders.iter().map(|order| order.amount.unwrap()).sum();

            i_asks.push(Level {
                level_id: i,
                side: Side::Asks,
                price: asks_price + (&tick_size * i as f64),
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

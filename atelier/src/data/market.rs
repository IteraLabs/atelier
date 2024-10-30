use crate::generators::randomizer::randomize_order;
use crate::messages::errors::{LevelError, OrderError};
use core::f64;
use std::time::{SystemTime, UNIX_EPOCH};

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

    // ----------------------------------------------------- Find a Level -- //
    // ----------------------------------------------------- ------------ -- //

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
    pub fn find_level(&self, level_price: f64) -> Result<i32, LevelError> {
        let mut i_level: i32 = 0;

        if level_price <= self.bids[0].price {
            for i_bid in &self.bids {
                i_level -= 1;
                if level_price == i_bid.price {
                    return Ok(i_level);
                }
            }
        }

        if level_price >= self.asks[0].price {
            for i_ask in &self.asks {
                i_level += 1;
                if level_price == i_ask.price {
                    return Ok(i_level);
                }
            }
        }

        Err(LevelError::LevelNotFound)
    }

    // --------------------------------------- Retrieve an Existing Level -- //
    // --------------------------------------- -------------------------- -- //

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
    pub fn retrieve_level(&self, level_price: f64) -> Result<Level, LevelError> {
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

    // ----------------------------------------- Delete an Existing Level -- //
    // ----------------------------------------- ------------------------ -- //

    /// To delete an existing level
    ///
    ///
    
    pub fn delete_level(&mut self, level_price: f64) -> Result<(), LevelError> {
        
        // see if level exists
        let find_level_ob = self.find_level(level_price);
        
        match find_level_ob {

            Ok(n) if n < 0 => {
    
                let bid_found = find_level_ob.unwrap().abs() as usize -1;
                self.bids.remove(bid_found);
                Ok(())
            },
        
            Ok(n) if n > 0 => {
                let ask_found = find_level_ob.unwrap() as usize -1 ;
                self.asks.remove(ask_found);
                Ok(())
            },
            
            Err(e) => Err(LevelError::LevelDeletionFailed),
            
            Ok(_) => Err(LevelError::LevelInfoNotAvailable),
        }
    
    }

    // ----------------------------------------------- Insert a New Level -- //
    // --------------------------------------------------- -------------- -- //

    /// Inserts a new level. If the level already exists, the new level over
    /// rides the existing one, if it does not exists, the new level is inserted
    /// in its corresponding slot within the Vec<Level> for the corresponding
    /// side.
    ///
    /// ## Parameters
    /// side: Side = {Side::Bids, Side::Asks}
    /// level_price: f64 = The Level's price to be used as index.
    /// volume: f64 = a single quantity for an empty `Vec<Order>`, the sum of
    /// all the order's amount.
    /// orders: Vec<Order> = The vector of `Order` structs.
    ///
    /// ## Returns
    /// Ok(Level)
    /// Err(LevelError): Custom Error Type of LevelInsertionFailed.
    ///
    pub fn insert_level(
        mut self,
        side: Side,
        level_price: f64,
        volume: f64,
        orders: Vec<Order>,
    ) -> Result<(), LevelError> {
        // return the level_price if it exists, or, LevelError::LevelNotFound
        if let Ok(i_level) = self.find_level(level_price) {
            
            // -- Level exist on the Bid side (to be replaced)
            if i_level < 0 {
                // updated counter to this side
                let i_level = i_level.abs() - 1;
                // override existing level with the new one
                let same_level_id = &self.bids[i_level as usize].level_id;
                self.bids[i_level as usize] =
                    Level::new(*same_level_id, side, level_price, volume, orders);

                Ok(())

            // -- Level exist on the Ask side (to be replaced)
            } else if i_level > 0 {
                // update counter to this side
                let i_level = i_level - 1;
                // override existing level with new one
                let same_level_id = &self.asks[i_level as usize].level_id;
                self.asks[i_level as usize] =
                    Level::new(*same_level_id, side, level_price, volume, orders);

                Ok(())

            // A response was produced but with an error on level index
            } else {
                
                // find the right position within the vector
                // and insert new Level there
                
                return Err(LevelError::LevelNotFound);
            }

        // Level not found, so insert a new one
        } else {

            match side {
                
                Side::Bids => {

                },

                Side::Asks => {

                },

            }
            
            // insert into the vector
            self.asks.insert(1); 

            return Err(LevelError::LevelNotFound);
        
        }
    }

    // ---------------------------------------------------- Find an Order -- //
    // ---------------------------------------------------- ------------- -- //

    /// To find if a given `Order` exists.
    ///
    /// ## Parameters
    /// side: Side = {Side::Bids, Side::Asks}
    /// price: f64 = the order's price, which is the same as the Level's price
    /// order_ts: u64 = Order's timestamp
    /// order_id: u32 = Order's unique ID
    ///
    /// Take the following sequence for validation of existence
    ///
    /// 1) side:
    ///     use it as the Side::Bids or Side::Asks, to find whether the side
    ///     exists.
    ///
    /// 2) price:
    ///     use it as the level_price, to find whether a level already exists.
    ///     
    /// 3) timestamp:
    ///     use it as the order index to find whether the order exists.
    ///
    /// ## Results
    ///

    pub fn find_order(
        &self,
        side: Side,
        price: f64,
        order_id: u32,
        order_ts: u64,
    ) -> Result<(), OrderError> {
        // Find if side exists
        let order_search = match side {
            //
            Side::Bids => {
                //
                if self.bids.len() > 0 {
                    return Ok(());
                } else {
                    return Err(OrderError::OrderNotFound);
                }
            }

            Side::Asks => {
                if self.asks.len() > 0 {
                    return Ok(());
                } else {
                    return Err(OrderError::OrderNotFound);
                }
            }
        };
    }

    // --------------------------------------- Retrieve an Existing Order -- //
    // --------------------------------------- -------------------------- -- //

    /// To retrieve info about an existing `Order`.
    ///
    /// ## Parameters
    ///
    /// ## Results
    ///

    pub fn retrieve_order() -> Result<(), OrderError> {
        Ok(())
    }

    // ----------------------------------------- Delete an Existing Order -- //
    // ----------------------------------------- ------------------------ -- //

    /// To delete an existing `Order`.
    ///
    /// ## Parameters
    ///
    /// ## Results
    ///

    pub fn delete_order() -> Result<(), OrderError> {
        Ok(())
    }

    // ----------------------------------------------- Insert a New Order -- //
    // ----------------------------------------------- ------------------ -- //

    /// To insert a new `Order`.
    ///
    /// ## Parameters
    ///
    /// ## Results
    ///

    pub fn insert_order() -> Result<(), OrderError> {
        Ok(())
    }

    // -------------------------------------------------- Modify an Order -- //
    // -------------------------------------------------- --------------- -- //

    /// To modify an existing `Order`.
    ///
    /// ## Parameters
    ///
    /// ## Results
    ///

    pub fn modify_order() -> Result<(), OrderError> {
        Ok(())
    }

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

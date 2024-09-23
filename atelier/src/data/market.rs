use std::ops::{Deref, DerefMut};

use crate::simulation::randomizer::randomize_order;
use trolly::lob::{
    ops::{update_strategies::ReplaceOrRemove, PartitionPredicate, Update},
    price_and_quantity::{Price, Quantity},
};

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
    /// Creates a new instance of `Order`.
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
    /// # Returns
    ///
    /// Returns a new `Order` instance with the specified parameters.
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
    pub fn new(level_id: u32, price: f64, volume: f64, orders: Vec<Order>) -> Self {
        Level {
            level_id,
            price,
            volume,
            orders,
        }
    }
}

// This is how the updater knows the Ordering of the Bids
impl PartitionPredicate for Bids {
    fn partition_predicate<P: PartialOrd>(lhs: &P, rhs: &P) -> bool {
        lhs < rhs
    }
}

// This is how the updater knows the Ordering of the Asks
impl PartitionPredicate for Asks {
    fn partition_predicate<P: PartialOrd>(lhs: &P, rhs: &P) -> bool {
        lhs > rhs
    }
}

impl Update<ReplaceOrRemove> for Bids {
    type Level = Level;
    type Key = usize;

    fn entry(&mut self, level_update: &Self::Level) -> (Self::Key, Option<&Self::Level>)
    where
        <Self::Level as trolly::lob::price_and_quantity::Price>::P: PartialOrd,
    {
        let index = self.partition_point(|value| {
            Self::partition_predicate(Price::to_ref(value), Price::to_ref(level_update))
        });
        (index, self.get(index))
    }

    fn digest_operation(
        &mut self,
        operator: ReplaceOrRemove,
        key: &Self::Key,
        level_update: Self::Level,
    ) {
        match operator {
            ReplaceOrRemove::Replace => {
                self[*key] = level_update;
            }
            ReplaceOrRemove::Remove => {
                self.remove(*key);
            }
            ReplaceOrRemove::Displace => self.insert(*key, level_update),
            ReplaceOrRemove::Noop => {}
        }
    }
}

impl Update<ReplaceOrRemove> for Asks {
    type Level = Level;
    type Key = usize;

    fn entry(&mut self, level_update: &Self::Level) -> (Self::Key, Option<&Self::Level>)
    where
        <Self::Level as trolly::lob::price_and_quantity::Price>::P: PartialOrd,
    {
        let index = self.partition_point(|value| {
            Self::partition_predicate(Price::to_ref(value), Price::to_ref(level_update))
        });
        (index, self.get(index))
    }

    fn digest_operation(
        &mut self,
        operator: ReplaceOrRemove,
        key: &Self::Key,
        level_update: Self::Level,
    ) {
        match operator {
            ReplaceOrRemove::Replace => {
                self[*key] = level_update;
            }
            ReplaceOrRemove::Remove => {
                self.remove(*key);
            }
            ReplaceOrRemove::Displace => self.insert(*key, level_update),
            ReplaceOrRemove::Noop => {}
        }
    }
}

// ------------------------------------------------------------ ORDERBOOK -- //

/// Represents a Limit Order Book for a specific market.
///
/// This `Orderbook` structure is different than 99.99% of other structs
/// within other Rust pojects, and that is the 3rd level of composition.
/// 1) Has both bids and asks sides (aham....)
/// 2) for each side, another Level struct with price, volume, etc (hemm ...)
/// 3) and for each Level, a queue (vector) of Order structs, (now we are talking)
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Orderbook {
    pub orderbook_id: u64,
    pub orderbook_ts: u64,
    pub symbol: String,
    pub bids: Bids,
    pub asks: Asks,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Bids(pub Vec<Level>);
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Asks(pub Vec<Level>);

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
        orderbook_id: u64,
        orderbook_ts: u64,
        symbol: String,
        bids: Bids,
        asks: Asks,
    ) -> Self {
        Orderbook {
            orderbook_id,
            orderbook_ts,
            symbol,
            bids,
            asks,
        }
    }

    pub fn insert_bid(&mut self, level: Level) {
        // If an old level is found, replace it; if the Level is not found, insert the new one.
        Update::<ReplaceOrRemove>::process(&mut self.bids, level);
    }

    pub fn insert_ask(&mut self, level: Level) {
        // If an old level is found, replace it; if the Level is not found, insert the new one.
        Update::<ReplaceOrRemove>::process(&mut self.asks, level);
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
    /// Returns a new `Orderbook` instance populated with synthetic bid and ask levels.
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
                price: i_ask_price,
                volume: i_ask_volume,
                orders: v_ask_orders,
            });
        }

        Orderbook {
            orderbook_id: 123,
            orderbook_ts: 321,
            symbol: String::from("BTCUSDT"),
            bids: Bids(i_bids),
            asks: Asks(i_asks),
        }
    }
}

impl Deref for Bids {
    type Target = Vec<Level>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bids {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Asks {
    type Target = Vec<Level>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Asks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Quantity for Level {
    type Q = f64;

    fn to_ref(&self) -> &Self::Q {
        &self.volume
    }
}

impl Price for Level {
    type P = f64;

    fn to_ref(&self) -> &Self::P {
        &self.price
    }
}

impl Bids {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl Asks {
    pub fn new() -> Self {
        Self(vec![])
    }
}

#[cfg(test)]
pub mod test {
    use crate::simulation::randomizer;

    use super::*;

    #[test]
    fn symmetric_sides() {
        let bid_price = 50_000.00;
        let ask_price = 50_100.00;
        let tick_size = 100.0;
        let n_levels = 10;
        let n_orders = 2;

        let i_ob = Orderbook::synthetize(bid_price, ask_price, tick_size, n_levels, n_orders);

        assert_eq!(i_ob.bids.len(), n_levels as usize);
        assert_eq!(i_ob.asks.len(), n_levels as usize);
        assert_eq!(i_ob.bids.len(), i_ob.asks.len());
    }

    #[test]
    fn orderbook_insert_levels() {
        let bid_level = vec![Level {
            level_id: 1,
            price: 60_000.00,
            volume: 1.0,
            orders: vec![],
        }];

        let ask_level = vec![Level {
            level_id: 1,
            price: 60_001.00,
            volume: 1.1,
            orders: vec![],
        }];

        let i_ob = Orderbook::new(
            123,
            123,
            String::from("BTCUSDT"),
            Bids(bid_level),
            Asks(ask_level),
        );

        println!("pre-bid_price {}", i_ob.bids[0].price);
        println!("pre-ask_price {}", i_ob.asks[0].price);
    }

    #[test]
    fn naive_progression() {
        // A correct progression of an orderbook should produce:
        // different orders at the same side -> level -> order queue

        // Parameters for the orderbook creation
        let bid_price = 50_000.00;
        let ask_price = 50_100.00;
        let tick_size = 50.0;
        let n_levels = 20;
        let n_orders = 5;

        // Parameters for the midprice progression model
        let mu = 0.0001;
        let sigma = 0.0025;

        let orderbook = Orderbook::synthetize(bid_price, ask_price, tick_size, n_levels, n_orders);
        let mut n_orderbooks: Vec<Orderbook> = vec![];
        n_orderbooks.push(orderbook);

        for i in 0..=3 {
            let i_bid_price = n_orderbooks[i].bids[0].price;
            let i_ret_gbm_bids: f64 = randomizer::gbm_return(i_bid_price, mu, sigma, 1.0);

            let i_ask_price = n_orderbooks[i].asks[0].price;
            let i_ret_gbm_asks: f64 = randomizer::gbm_return(i_ask_price, mu, sigma, 1.0);

            let i_orderbook = Orderbook::synthetize(
                i_bid_price - i_ret_gbm_bids,
                i_ask_price + i_ret_gbm_asks,
                tick_size,
                n_levels,
                n_orders,
            );

            n_orderbooks.push(i_orderbook);
        }

        // for the first and second orderbook, compare the first order of the first level

        // for the bid side
        let t0_bids0_order0 = n_orderbooks[0].bids[0].orders.first().unwrap();
        let t1_bids0_order0 = n_orderbooks[1].bids[0].orders.first().unwrap();
        assert_ne!(t0_bids0_order0, t1_bids0_order0);

        // for ask side
        let t0_asks0_order0 = n_orderbooks[0].asks[0].orders.first().unwrap();
        let t1_asks0_order0 = n_orderbooks[1].asks[0].orders.first().unwrap();
        assert_ne!(t0_asks0_order0, t1_asks0_order0);
    }

    #[test]
    fn insert_bid_works() {
        let mut ob = Orderbook::new(0, 0, String::from("btc"), Bids::new(), Asks::new());
        let level = Level::new(0, 1., 1., vec![]);
        ob.insert_bid(level);
        assert_eq!(ob.bids.0, [Level::new(0, 1., 1., vec![])]);
        let level = Level::new(0, 0., 1., vec![]);
        ob.insert_bid(level);
        assert_eq!(
            ob.bids.0,
            [Level::new(0, 0., 1., vec![]), Level::new(0, 1., 1., vec![])]
        );
        let level = Level::new(0, 1., 2., vec![]);
        ob.insert_bid(level);
        assert_eq!(
            ob.bids.0,
            [Level::new(0, 0., 1., vec![]), Level::new(0, 1., 2., vec![])]
        );
        let level = Level::new(0, 1., 0., vec![]);
        ob.insert_bid(level);
        assert_eq!(ob.bids.0, [Level::new(0, 0., 1., vec![])]);
    }

    #[test]
    fn insert_ask_works() {
        let mut ob = Orderbook::new(0, 0, String::from("btc"), Bids::new(), Asks::new());
        let level = Level::new(0, 1., 1., vec![]);
        ob.insert_ask(level);
        assert_eq!(ob.asks.0, [Level::new(0, 1., 1., vec![])]);
        let level = Level::new(0, 0., 1., vec![]);
        ob.insert_ask(level);
        assert_eq!(
            ob.asks.0,
            [Level::new(0, 1., 1., vec![]), Level::new(0, 0., 1., vec![])]
        );
        let level = Level::new(0, 1., 2., vec![]);
        ob.insert_ask(level);
        assert_eq!(
            ob.asks.0,
            [Level::new(0, 1., 2., vec![]), Level::new(0, 0., 1., vec![])]
        );
        let level = Level::new(0, 1., 0., vec![]);
        ob.insert_ask(level);
        assert_eq!(ob.asks.0, [Level::new(0, 0., 1., vec![])]);
    }
}

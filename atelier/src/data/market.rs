use crate::simulation::randomizer::randomize_order;

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

        let i_ob = Orderbook::synthetize(
            bid_price, ask_price, tick_size, n_levels, n_orders,
        );

        assert_eq!(i_ob.bids.len(), n_levels as usize);
        assert_eq!(i_ob.asks.len(), n_levels as usize);
        assert_eq!(i_ob.bids.len(), i_ob.asks.len());
    }

    #[test]
    fn orderbook_insert_levels() {
        let bid_level = vec![Level {
            level_id: 1,
            side: Side::Bids,
            price: 60_000.00,
            volume: 1.0,
            orders: None,
        }];

        let ask_level = vec![Level {
            level_id: 1,
            side: Side::Asks,
            price: 60_001.00,
            volume: 1.1,
            orders: None,
        }];

        let i_ob = Orderbook::new(
            123,
            123,
            String::from("BTCUSDT"),
            bid_level,
            ask_level,
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

        let orderbook = Orderbook::synthetize(
            bid_price, ask_price, tick_size, n_levels, n_orders,
        );
        let mut n_orderbooks: Vec<Orderbook> = vec![];
        n_orderbooks.push(orderbook);

        for i in 0..=3 {
            let i_bid_price = n_orderbooks[i].bids[0].price;
            let i_ret_gbm_bids: f64 =
                randomizer::gbm_return(i_bid_price, mu, sigma, 1.0);

            let i_ask_price = n_orderbooks[i].asks[0].price;
            let i_ret_gbm_asks: f64 =
                randomizer::gbm_return(i_ask_price, mu, sigma, 1.0);

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
        let t0_bids0_order0 = n_orderbooks[0].bids[0]
            .orders
            .as_ref()
            .map(|o| o[0])
            .unwrap();
        let t1_bids0_order0 = n_orderbooks[1].bids[0]
            .orders
            .as_ref()
            .map(|o| o[0])
            .unwrap();
        assert_ne!(t0_bids0_order0, t1_bids0_order0);

        // for ask side
        let t0_asks0_order0 = n_orderbooks[0].asks[0]
            .orders
            .as_ref()
            .map(|o| o[0])
            .unwrap();
        let t1_asks0_order0 = n_orderbooks[1].asks[0]
            .orders
            .as_ref()
            .map(|o| o[0])
            .unwrap();
        assert_ne!(t0_asks0_order0, t1_asks0_order0);
    }
}

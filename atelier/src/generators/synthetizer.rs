//! # Synthetizer
//!
//! This module provides the functionality to generate synthetic orderbooks
//! using various quantitative methods, for now only including brownian motion.
//!
//! The main struct in this module is the `Synthetizer`, which allows for
//! the creation of progressions, each of which a synthetic orderbook, based
//! on an initial orderbook state.
//!

use crate::data::market::{Level, Order, OrderType, Orderbook, Side};
use crate::generators::brownian;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::time::{SystemTime, UNIX_EPOCH};

/// A struct to produce synthetic version of orderbooks
///
/// This struct holds an initial orderbook state, and, it provides methods to
/// generate progressions of the orderbook, based on implemented algorithms/models.
pub struct Synthetizer {
    /// The initial orderbook state used as a starting point for the progressions.
    pub initial_ob: Orderbook,
}

impl Synthetizer {
    /// Creates a new `Synthetizer` instance.
    ///
    /// # Parameters
    ///
    /// * `initial_ob` - The initial orderbook state to use as a starting point.
    ///
    /// # Returns
    ///
    /// A new `Synthetizer` instance.

    pub fn new(initial_ob: Orderbook) -> Self {
        Synthetizer { initial_ob }
    }

    /// Generates a series of synthetic orderbooks using Brownian motion.
    ///
    /// This method creates a sequence of orderbooks where the price movements
    /// are based on Geometric Brownian Motion (GBM).
    ///
    /// # Parameters
    ///
    /// * `n_progressions` - The number of orderbooks to generate.
    /// * `tick_size` - The minimum price movement.
    /// * `n_levels` - The number of price levels to generate for each side of the orderbook.
    /// * `n_orders` - The number of orders to generate for each price level.
    ///
    /// # Returns
    ///
    /// A vector of `Orderbook` instances representing the synthetic orderbook progression.
    ///

    pub fn brownian(
        self,
        n_progressions: u16,
        tick_size: f64,
        n_levels: u32,
        n_orders: u32,
    ) -> Vec<Orderbook> {
        let mut uni_rand = rand::thread_rng();
        let mut v_orderbooks: Vec<Orderbook> = Vec::new();

        // bid & ask prices to start the progression from
        let mut tob_bid_price = self.initial_ob.bids[0].price;
        let mut tob_ask_price = self.initial_ob.asks[0].price;

        let gbm_s0: f64 = (tob_bid_price + tob_ask_price) / 2.0;
        let gbm_mu: f64 = 0.20;
        let gbm_dt: f64 = 0.001;
        let gbm_sigma: f64 = 0.1;
        let gbm_n: usize = 1;

        for j in 0..=n_progressions {
            // Form an empty orderbook for the current Progression

            let j_ob_id = 1234;
            let j_ob_ts = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64;
            let j_ob_symbol = String::from("btcusd");
            let j_ob_bids = Vec::new();
            let j_ob_asks = Vec::new();
            let j_orderbook = Orderbook::new(j_ob_id, j_ob_ts, j_ob_symbol, j_ob_bids, j_ob_asks);

            v_orderbooks.push(j_orderbook);

            // Get bids change in price
            let gbm_ret = brownian::gbm_return(gbm_s0, gbm_mu, gbm_sigma, gbm_dt, gbm_n).unwrap();

            // Algebraic sum to incorporate price innovation for both sides
            let tob_bid_price = tob_bid_price + gbm_ret[0];
            let tob_ask_price = tob_ask_price + gbm_ret[0];

            for i in 0..=n_levels {
                // -- Bid Level Formation -- //

                let i_bid_price = tob_bid_price - (&tick_size * i as f64);
                let i_bid_side = Side::Bids;
                let i_order_type = OrderType::Limit;
                let i_bid_size = uni_rand.sample(Uniform::new(0.01, 0.3));

                let mut v_bid_orders: Vec<Order> = (0..n_orders).map(|_| Order::random()).collect();

                v_bid_orders.sort_by_key(|order| order.order_ts);

                let i_bid_volume: f64 =
                    v_bid_orders.iter().map(|order| order.amount.unwrap()).sum();

                v_orderbooks[j as usize].bids.push(Level {
                    level_id: i,
                    side: i_bid_side,
                    price: i_bid_price,
                    volume: i_bid_volume,
                    orders: v_bid_orders,
                });

                // -- Asks Levels Formation -- //

                let i_ask_price = tob_ask_price + (&tick_size * i as f64);
                let i_ask_side = Side::Asks;
                let i_order_type = OrderType::Limit;
                let i_ask_size = uni_rand.sample(Uniform::new(0.01, 0.3));

                let mut v_ask_orders: Vec<Order> = (0..n_orders).map(|_| Order::random()).collect();

                v_ask_orders.sort_by_key(|order| order.order_ts);

                let i_ask_volume: f64 =
                    v_ask_orders.iter().map(|order| order.amount.unwrap()).sum();

                v_orderbooks[j as usize].asks.push(Level {
                    level_id: i,
                    side: i_ask_side,
                    price: i_ask_price,
                    volume: i_ask_volume,
                    orders: v_ask_orders,
                });
            }
        }

        v_orderbooks
    }
}

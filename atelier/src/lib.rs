//! # Atelier
//! A Computational Framework/Engine for Market Microstructure High Frequency Modeling,
//! Synthetic Simulation and Historical Market Reconstruction/Replay:
//!
//! * **Quantitative Modeling**:
//! * **Synthetic Market Generation**:
//! * **Historical Market Replay**:
//!
//! In depth use-cases documentation is found on the [website]
//!
//! [website]: https://www.iteralabs.ai/atelier/docs
//!
//!
//! ### Order-level activity
//!
//! Financial Markets are Order-Driven markets, to generate a synthetic market
//! one should be able to, in priciple, generate order by order activity, because
//! every single price change is, in the vast majority of the cases, just 
//! the matching of two or more orders.
//!
//! ### Matching Engine
//!
//! Most common rule to match two orders is the FIFO (First-in, First-out). You can use
//! the base implementation for this.
//!
//! ### Synthetic Progressions
//!
//! One way to generate a synthetic dataset for development purposes is by 
//! generating progressions with a probabilistic approach.
//!
//! ```
//!
//! use atelier_data::orderbooks::Orderbook;
//! use rand::distributions::{Bernoulli, Uniform, Distribution};
//! use rand::Rng;
//! 
//! fn main() {
//!
//!    let ini_bid_price = 100_000.00;
//!    let ini_bid_levels = 2;
//!    let ini_bid_orders = 3;
//!
//!    let ini_ask_price = 100_001.00;
//!    let ini_ask_levels = 2;
//!    let ini_ask_orders = 3;
//!
//!    let ini_ticksize = 1.0;
//!
//!    let r_ob = Orderbook::random(
//!        ini_bid_price,
//!        ini_bid_levels,
//!        ini_bid_orders,
//!        ini_ticksize,
//!        ini_ask_price,
//!        ini_ask_levels,
//!        ini_ask_orders,
//!    );
//!    
//!    let mut v_orderbook: Vec<Orderbook> = vec![];
//!
//!   for _ in 0..3 {
//!    
//!       let mut uni_rand = rand::thread_rng();
//!        let r_amount_ret = uni_rand.sample(Uniform::new(0.001, 0.005));
//!    
//!        let mut rng = rand::thread_rng();
//!        let bernoulli = Bernoulli::new(0.3).unwrap();
//!        let r_sign_ret = if bernoulli.sample(&mut rng) { 1.0 } else { -1.0 };
//!        
//!        let v_bid_price = ini_bid_price + ini_bid_price * r_amount_ret * r_sign_ret;
//!        let v_ask_price = ini_ask_price + ini_ask_price * r_amount_ret * r_sign_ret;
//!    
//!        let r_ob = Orderbook::random(
//!            v_bid_price,
//!            ini_bid_levels,
//!            ini_bid_orders,
//!            ini_ticksize,
//!            v_ask_price,
//!            ini_ask_levels,
//!            ini_ask_orders,
//!        );
//!        v_orderbook.push(r_ob);
//!    }
//! }
//! ```
//! 

#![allow(dead_code)]
#![allow(warnings)]

/// Market event generator.
pub mod events;

/// Messages structures for: Errors, Success, Events, and Logs.
pub mod results;

/// Metrics calculation about various aspects and processes.
pub mod metrics;

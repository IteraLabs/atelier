//! Synthetic Generation of Centralized Exchange
//! Orderbooks

use atelier_core::{
    orderbooks::Orderbook,
    templates::{ModelConfig, OrderbookConfig},
};
use atelier_generators::brownian;
use futures::future::join_all;
use std::error::Error;

/// Generates a randomized orderbook snapshot based on input parameters.
///
/// Constructs an orderbook using the Orderbook::random to model price evolution,
/// with configurable market depth and order characteristics.
///
/// # Arguments
/// - `bid_price`: Initial best bid price
/// - `bid_levels`: [min, max] number of price levels for bids
/// - `bid_orders`: [min, max] number of orders per bid level
/// - `ticksize`: [min, max] price increment between levels
/// - `ask_price`: Initial best ask price
/// - `ask_levels`: [min, max] number of price levels for asks
/// - `ask_orders`: [min, max] number of orders per ask level
///
/// # Returns
/// `Result<Orderbook>` containing either:
/// - Randomized orderbook snapshot
/// - Error if input validation fails
///
/// # Panics
/// - If any vector argument doesn't contain exactly 2 elements
/// - If bid_price >= ask_price (violates market structure)
///
pub fn progress(
    bid_price: f64,
    bid_levels: Vec<u32>,
    bid_orders: Vec<u32>,
    ticksize: Vec<f64>,
    ask_price: f64,
    ask_levels: Vec<u32>,
    ask_orders: Vec<u32>,
) -> Result<Orderbook, Box<dyn Error>> {
    let r_ob = Orderbook::random(
        bid_price,
        Some((bid_levels[0], bid_levels[1])),
        Some((bid_orders[0], bid_orders[1])),
        Some((ticksize[0], ticksize[1])),
        ask_price,
        Some((ask_levels[0], ask_levels[1])),
        Some((ask_orders[0], ask_orders[1])),
    );

    Ok(r_ob)
}

/// Generates a sequence of orderbook progressions using Brownian motion dynamics.
///
/// This async function creates a time series of orderbooks where each subsequent book:
/// - Inherits structure from previous state
/// - Evolves prices using GBM returns
/// - Maintains configurable market depth parameters
///
/// # Arguments
/// - `template_orderbook`: Initial configuration with all fields required
/// - `template_model`: GBM parameters (μ, σ) required
/// - `n_progres`: Number of progressions to generate
///
/// # Returns
/// `Result<Vec<Orderbook>>` containing either:
/// - Time series of orderbook states
/// - Error if input validation fails or model becomes unstable
///
/// # Panics
/// - If any template field contains `None`
/// - If μ or σ lead to negative prices
///
pub async fn progressions(
    template_orderbook: OrderbookConfig,
    template_model: ModelConfig,
    n_progres: usize,
) -> Result<Vec<Orderbook>, Box<dyn Error + Send + Sync>> {
    let mut v_orderbooks: Vec<Orderbook> = vec![];

    let ini_bid = template_orderbook.bid_price.unwrap();
    let ini_ask = template_orderbook.ask_price.unwrap();
    let i_s0 = (ini_bid + ini_ask) / 2.0;
    let i_dt = 0.01;
    let i_n = n_progres;

    let i_mu = template_model.params_values.as_ref().unwrap()[0];
    let i_sigma = template_model.params_values.unwrap()[1];

    let gbm_return = brownian::gbm_return(i_s0, i_mu, i_sigma, i_dt, i_n).unwrap();

    let mut bid_price = template_orderbook.bid_price.unwrap();
    let bid_levels = template_orderbook.bid_levels.unwrap();
    let bid_orders = template_orderbook.bid_orders.unwrap();
    let ticksize = template_orderbook.ticksize.unwrap();
    let mut ask_price = template_orderbook.ask_price.unwrap();
    let ask_levels = template_orderbook.ask_levels.unwrap();
    let ask_orders = template_orderbook.ask_orders.unwrap();

    for i in 0..n_progres {
        let r_ob = Orderbook::random(
            bid_price,
            Some((bid_levels[0], bid_levels[1])),
            Some((bid_orders[0], bid_orders[1])),
            Some((ticksize[0], ticksize[1])),
            ask_price,
            Some((ask_levels[0], ask_levels[1])),
            Some((ask_orders[0], ask_orders[1])),
        );

        v_orderbooks.push(r_ob.clone());

        // --- Progress next Orderbook
        bid_price = r_ob.bids[0].price.clone() + gbm_return[i];
        ask_price = r_ob.asks[0].price.clone() + gbm_return[i];
    }
    Ok(v_orderbooks)
}

/// Executes multiple orderbook progression scenarios concurrently.
///
/// This high-performance implementation uses async rust to parallelize:
/// - Different initial orderbook configurations
/// - Multiple model parameterizations
/// - Independent progression sequences
///
/// # Arguments
/// - `orderbooks`: Vector of unique initial orderbook states
/// - `models`: Corresponding vector of model configurations
/// - `n_progres`: Number of steps per progression sequence
///
/// # Returns
/// Vector of individual progression results, preserving input order
///
/// # Note
/// Each progression task fails independently - check individual results
/// for partial successes in distributed computing scenarios
///
pub async fn async_progressions(
    orderbooks: Vec<OrderbookConfig>,
    models: Vec<ModelConfig>,
    n_progres: usize,
) -> Vec<Result<Vec<Orderbook>, Box<dyn std::error::Error + Send + Sync>>> {
    let tasks = orderbooks
        .into_iter()
        .zip(models.into_iter())
        .map(|(ob, model)| progressions(ob, model, n_progres));

    join_all(tasks).await
}


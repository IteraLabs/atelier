use atelier::generators::{brownian, hawkes};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // -------------------------------------------------------------- HAWKES PROCESS -- //
    // -------------------------------------------------------------- -------------- -- //

    // Average ocurrence rate of events.
    // units = counts over time units.
    // Total number of orders divided by the time period.
    let hawkes_mu: f64 = 0.85;

    // Market impact of an Order.
    // units = A-dimensional.
    // greater than 0 (there is an effect) but lower than 1 (final total impact)
    let hawkes_alpha: f64 = 0.8;

    // Market absortion of an impactful Order.
    // units = rate of decay (counts over time units).
    // same timescale of alpha. between 0 and N.
    let hawkes_beta: f64 = 1.0;

    // Initialize the struct
    let hawkes_events = hawkes::HawkesProcess {
        mu: hawkes_mu,
        alpha: hawkes_alpha,
        beta: hawkes_beta,
    };

    let current_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;

    let n_values: usize = 5;

    // include onlye the number of events to generate
    let order_times = hawkes_events.generate_values(current_ts, n_values);

    // Output the generated order times
    for time in order_times {
        println!("Order arrived at time: {:?}", time);
    }

    // ------------------------------------------------------------- BROWNIAN MOTION -- //
    // ------------------------------------------------------------- --------------- -- //

    let gbm_s0: f64 = 70_000.0;
    let gbm_mu: f64 = 0.0001;
    let gbm_sigma: f64 = 0.025;
    let gbm_dt: f64 = 0.1;
    let gbm_n: usize = 5;

    // Get the result for this test
    let _returns_progression = brownian::gbm_return(gbm_s0, gbm_mu, gbm_sigma, gbm_dt, gbm_n);

    // println!(
    //    "For the next {:?} steps,\nthe returns will be: {:?}",
    //    gbm_n,
    //    returns_progression.unwrap()
    // );
}

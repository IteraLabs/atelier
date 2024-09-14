use atelier::simulation::hawkes;

fn main() {
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

    // include onlye the number of events to generate
    let order_times = hawkes_events.generate_times(5);

    // Output the generated order times
    for time in order_times {
        println!("Order arrived at time: {:?}", time);
    }
}

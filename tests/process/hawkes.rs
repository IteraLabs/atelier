#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use atelier::generators::hawkes;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    // -- ---------------------------------------------------------------------------- //
    fn test_hawkes_single_process() {
        // Parameters for this test
        let hawkes_mu: f64 = 0.85;
        let hawkes_alpha: f64 = 0.8;
        let hawkes_beta: f64 = 1.0;

        let current_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as f64;

        let n_values: usize = 5;

        // Get the result for this test
        let hawkes_events = hawkes::HawkesProcess {
            mu: hawkes_mu,
            alpha: hawkes_alpha,
            beta: hawkes_beta,
        };

        // include only the number of events to generate
        let order_times = hawkes_events.generate_values(current_ts, n_values);

        assert_eq!(order_times.len(), 5);
    }

    #[test]
    // -- ---------------------------------------------------------------------------- //
    fn test_hawkes_approx_expectation() {
        // Parameters for this test
        let hawkes_mu: f64 = 0.85;
        let hawkes_alpha: f64 = 0.8;
        let hawkes_beta: f64 = 1.0;

        let n_values: usize = 5;

        // Get the result for this test
        let hawkes_events = hawkes::HawkesProcess {
            mu: hawkes_mu,
            alpha: hawkes_alpha,
            beta: hawkes_beta,
        };

        let current_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as f64;

        // include only the number of events to generate
        let generated_times = hawkes_events.generate_values(current_ts, n_values);
        let expected_times = current_ts;

        assert_abs_diff_eq!(generated_times[0], expected_times, epsilon = 2.0);
    }
}

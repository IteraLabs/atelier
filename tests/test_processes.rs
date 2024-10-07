#[cfg(test)]
mod tests {
    use atelier::generators::brownian;
    use atelier::generators::hawkes;

    #[test]
    fn hawkes_process() {
        let hawkes_mu: f64 = 0.85;
        let hawkes_alpha: f64 = 0.8;
        let hawkes_beta: f64 = 1.0;

        let hawkes_events = hawkes::HawkesProcess {
            mu: hawkes_mu,
            alpha: hawkes_alpha,
            beta: hawkes_beta,
        };

        // include only the number of events to generate
        let order_times = hawkes_events.generate_times(5);
        assert_eq!(order_times.len(), 5);
    }

    #[test]
    fn gmb_process() {
        let gbm_s0: f64 = 55_000.0;
        let gbm_mu: f64 = 0.1;
        let gbm_sigma: f64 = 0.2;
        let gbm_t: f64 = 2.0;

        // A naive test for the name and route
        let gbm_events = brownian::gbm_return(gbm_s0, gbm_mu, gbm_sigma, gbm_t);

        assert!(gbm_events.is_ok());
    }
}

#[cfg(test)]
mod tests {
    use atelier::generators::brownian;

    #[test]
    fn single_process() {
        let gbm_s0: f64 = 55_000.0;
        let gbm_mu: f64 = 0.1;
        let gbm_sigma: f64 = 0.2;
        let gbm_t: f64 = 2.0;

        // A naive test for the name and route
        let gbm_events = brownian::gbm_return(gbm_s0, gbm_mu, gbm_sigma, gbm_t);

        assert!(gbm_events.is_ok());
    }
}

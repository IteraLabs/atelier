#[cfg(test)]

mod tests {
    use approx::assert_abs_diff_eq;
    use atelier::generators::brownian;
    use atelier::generators::probabilistic;
    use atelier::generators::probabilistic::Sampling;

    #[test]
    // -- ---------------------------------------------------------------------------- //
    fn test_single_process() {
        // Parameters for this test
        let gbm_s0: f64 = 70_000.0;
        let gbm_mu: f64 = 0.1;
        let gbm_sigma: f64 = 0.2;
        let gbm_dt: f64 = 3.0;
        let gbm_n: usize = 1;

        // Get the result for this test
        let test_result = brownian::gbm_return(gbm_s0, gbm_mu, gbm_sigma, gbm_dt, gbm_n);

        assert!(test_result.is_ok());
    }

    #[test]
    // -- ---------------------------------------------------------------------------- //
    fn test_gbm_return_valid_inputs() {
        // Parameters for this test
        let gbm_s0: f64 = 70_000.00;
        let gbm_mu: f64 = 0.1;
        let gbm_sigma: f64 = 1.0;
        let gbm_dt: f64 = -1.0;
        let gbm_n: usize = 1;

        // Get the result for this tests
        let test_result = brownian::gbm_return(gbm_s0, gbm_mu, gbm_sigma, gbm_dt, gbm_n);
        assert!(test_result.is_err());
    }

    #[test]
    // -- ---------------------------------------------------------------------------- //
    fn test_gbm_return_approx_expectation() {
        // Parameters for this test
        let gbm_s0: f64 = 70_000.00;
        let gbm_mu: f64 = 0.1;
        let gbm_sigma: f64 = 0.01;
        let gbm_dt: f64 = 0.1;
        let gbm_n: usize = 1;
        let dis = probabilistic::NormalDistribution {
            mu: 0.0,
            sigma: gbm_dt.sqrt(),
        };
        let dwt = dis.sample(gbm_n)[0];

        // Stablish an expected value
        let exp_drift = ((gbm_mu - gbm_sigma * gbm_sigma) / 2.0) * gbm_dt;
        let exp_diffusion = gbm_sigma * dwt;
        let expected_value = gbm_s0 * (exp_drift + exp_diffusion).exp();

        // Get the generated value
        let brownian_result = brownian::gbm_return(gbm_s0, gbm_mu, gbm_sigma, gbm_dt, gbm_n);
        let generated_value = gbm_s0 + brownian_result.unwrap()[0];

        let result_tolerance = gbm_s0 * 0.1;
        assert_abs_diff_eq!(generated_value, expected_value, epsilon = result_tolerance);
    }
}

#[cfg(test)]
mod tests {
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

        // include onlye the number of events to generate
        let order_times = hawkes_events.generate_times(5);
        assert_eq!(order_times.len(), 5);
    }  
}


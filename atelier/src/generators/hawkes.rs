//! # Point process discrete simulation
//!
//! A Hawkes Process is a self-exciting point process where the intensity of events
//! increases following the occurrence of previous events.
//!
//! ## Simplest case (Linear regression on past events).
//!
//! In one dimension, this process is self-exciting, i.e. the arrival
//! of an event increases the likelihood of observing events in the
//! near future. It is also useful to consider the case when there is
//! more than one type of event, and there is mutual excitement
//! between the different events. For $d$ such events, we define a
//! $d$-dimensional Hawkes process:
//!
//! $$
//!  \lambda_{i}(t) = \mu_{i} + \sum_{j=1}^{d} \sum_{t_{j,r} \leq t} \phi_{ij}(t - t_{j,r})
//! $$
//!
//! Where:
//!
//! $\mu_{i} \in R_{+}$: Base line (exogenous) intensities. \
//! $\phi_{ij}$: The (exciting) Kernel functions. \
//! $t_{j,r}$ : the time of the $j^{th}$ event of type $r$
//! \
//! \
//! As for the $\phi_{ij}$ kernel definition, exponential kernels
//! are among the most commonly used in Hawkes
//! processes due to their simplicity and mathematical properties:
//!
//! $$
//!  \phi_{ij}(t) = \alpha_{ij} e^{-\beta_{ij}t}
//! $$
//!
//! Where:
//!
//! $\alpha$: Excitation factor (how much each event excites the
//! future events). \
//! $\beta$: Decay rate (how quickly the excitement diminishes).\
//!
//! ## Context usage
//! When used this approach arount the Orderbook, the arrival of an
//! order can increase the likelihood of subsequent orders.
use rand::Rng;
pub struct HawkesProcess {
    pub mu: f64,
    pub alpha: f64,
    pub beta: f64,
}

impl HawkesProcess {
    // Constructor to initialize the Hawkes process parameters
    pub fn new(mu: f64, alpha: f64, beta: f64) -> Self {
        Self { mu, alpha, beta }
    }

    // Method to generate N synthetic timestamps
    pub fn generate_times(&self, n: usize) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let mut event_times = Vec::new();
        let mut current_time = 1726356610031.0;

        for _ in 0..n {
            // Calculate the current intensity
            let intensity = self.lambda(current_time, &event_times);

            // Sample the waiting time until the next event
            let wait_time = rng.gen_range(0.0..1.0 / intensity);
            current_time += wait_time; // Update the current time

            // Store the new event time
            event_times.push(current_time);
        }
        event_times
    }

    // Method to compute the intensity at a given time based on past events
    fn lambda(&self, t: f64, event_times: &[f64]) -> f64 {
        let mut intensity = self.mu;
        for &event_time in event_times {
            if event_time < t {
                intensity += self.alpha * (-self.beta * (t - event_time)).exp();
            }
        }
        intensity
    }
}
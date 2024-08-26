use atelier::simulation::generators::{Distribution, Exponential, Poisson};

fn main() {
    let poisson = Poisson { lambda: 1.0 };
    let sample_poisson = poisson.sample(5);
    println!(
        "sample from poisson: {:?}",
        sample_poisson
    );

    let exponential = Exponential { lambda: 1.0 };
    let sample_exponential = exponential.sample(5);
    println!(
        "sample from exponential: {:?}",
        sample_exponential
    );
}

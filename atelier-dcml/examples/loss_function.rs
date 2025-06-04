/// Example: Loss Function 

use tch::{Tensor, Kind};
use atelier_dcml::functions;

pub fn main() {

    println!(" --- Loss Functions Example --- ");

    let y_true = Tensor::from_slice(&vec![1.0, 0.0, 0.0, 1.0]).to_kind(Kind::Int64);
    let y_hat = Tensor::from_slice(&vec![1.0, 1.0, 0.0, 0.0]).to_kind(Kind::Int64);
    let epsilon = 0.001;
    let weights = Tensor::from_slice(&vec![10.00, -5.0]);

    let loss = functions::CrossEntropy::builder()
        .weights(&weights)
        .y_true(&y_true)
        .y_hat(&y_hat)
        .epsilon(epsilon)
        .build();

    println!("loss : {:?}", loss);

    let computed_loss = loss.unwrap().compute_loss();

    println!("computed loss : {:?}", computed_loss);

}


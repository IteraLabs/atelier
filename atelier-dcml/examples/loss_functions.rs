/// Example: Loss Function

use atelier_dcml::functions;
use tch::{Kind, Tensor};

pub fn main() {
    println!(" --- Loss Functions Example --- ");

    let y_true = Tensor::from_slice(&vec![1.0, 0.0, 0.0, 1.0]).to_kind(Kind::Int64);
    let y_hat = Tensor::from_slice(&vec![1.0, 1.0, 0.0, 0.0]).to_kind(Kind::Int64);
    let loss = functions::CrossEntropy::new().build();

    println!("loss : {:?}", loss);
    let computed_loss = loss.unwrap().compute_loss(&y_hat, &y_true);
    println!("computed loss : {:?}", computed_loss);

}

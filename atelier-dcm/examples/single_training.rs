use atelier_dcm::{dataset, features, targets};
use tch::{
    nn::{Adam, OptimizerConfig, VarStore},
    Device, Kind, Reduction, Tensor, TrainableCModule,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- Files
    let model_file = String::from(
        "/Users/franciscome/git/iteralabs/atelier/atelier-torch/src/scripted_model.pt",
    );
    let data_file = String::from("americas_orderbook.json");

    // -- Variable store
    let vs = VarStore::new(Device::Cpu);

    // --- Load scripted model
    let mut model = TrainableCModule::load(model_file, vs.root())?;
    model.set_train();

    // --- Features --- //
    let v_orderbook_2 = dataset::read_json(&data_file)?;
    let wmid_price: Vec<f64> = features::ob_wmidprice(&v_orderbook_2)?;
    let vwap_price: Vec<f64> = features::ob_vwap(&v_orderbook_2, 2 as usize)?;
    let wmid_price_tensor = Tensor::from_slice(&wmid_price).unsqueeze(1);
    let vwap_price_tensor = Tensor::from_slice(&vwap_price).unsqueeze(1);

    // --- Input data conversion to Tensor
    let xs_vec = [wmid_price_tensor, vwap_price_tensor];
    let xs_pre = Tensor::cat(&xs_vec, 1).to_kind(Kind::Float);
    let xs = xs_pre.roll(&[1], &[0]); // shift=1 along dim=0 (rows)
    let xs_mean = xs.mean(Kind::Float);
    let xs_std = xs.std(true);
    let xs = (&xs - &xs_mean) / (&xs_std + 1e-8);

    // --- Target data conversion to Tensor
    let ys_vec = targets::ob_price_direction(&v_orderbook_2)?;
    let ys = Tensor::from_slice(&ys_vec)
        .unsqueeze(1)
        .to_kind(Kind::Float);

    // --- Distributed Specs --- //
    // m  := Number of agents (one per exchange), m = 3
    // G  := Undirected Communication Network Graph, G = ([m], edges)
    // F  := Global Minimization Problem
    // xi := Consensus update, xi = aij + sum (aij * xj)
    //

    // ----------------------------------------------------------------- Optimizer --- //
    // ----------------------------------------------------------------- --------- --- //

    let mut opt = Adam::default().build(&vs, 1e-3)?;

    // ------------------------------------------------------------------ Training --- //
    // ------------------------------------------------------------------ -------- --- //

    for epoch in 1..=5000 {
        // Forward pass
        let predictions = model.forward_ts(&[xs.shallow_clone()])?.sigmoid();

        // Loss
        let loss = predictions.binary_cross_entropy::<Tensor>(&ys, None, Reduction::Mean);

        // Backward pass
        opt.zero_grad();
        loss.backward();
        opt.step();

        // Log result
        if epoch % 10 == 0 {
            println!("Epoch: {:?}, Loss: {:?}", epoch, &loss);
        }
    }

    Ok(())
}

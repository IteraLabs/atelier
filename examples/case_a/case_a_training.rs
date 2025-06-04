/// Single Agent Training

use atelier_dcml::{models, metrics, functions, optimizers, processes};
use std::{path::Path, error::Error};

// use tch::{Tensor, Kind, IndexOp};
// use atelier_dcml::{
//    agents::DistributedAgent,
//    training,
//    features,
//    targets,
// };
//

use atelier_core::{
    data,
    templates,
};

pub fn main() -> Result<(), Box<dyn Error + 'static>> {

    // --- Setup working directory
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(manifest_dir)
        .parent()
        .expect("Failed to get workspace root");

    // --- Template file (toml)
    let template_file = workspace_root
        .join("examples")
        .join("case_a")
        .join("config_a.toml");

    let template = templates::Config::load_from_toml(template_file.to_str().unwrap())
        .unwrap()
        .clone();

    // --- Extract parameters from template
    let _exp_id = &template.experiments[0].id;
    let _n_progres = template.experiments[0].n_progressions as usize;
    let _template_model = template.models[0].clone();
   

    // --- Data Layer --- // 

    // File specification and read
    let data_file = workspace_root
        .join("examples")
        .join("case_a")
        .to_str()
        .unwrap().to_owned() + "/data_case_a.csv";
  
    let header = true;
    let column_types = None;
    let target_column = Some(7);

    let a_data = data::Dataset::from_csv(
        &data_file, header, column_types, target_column
    );

    let (features, target) = a_data.unwrap().from_csv_to_tensor();
    let n_features = features.size()[1];
    let n_target = target.size()[1];
    
    // --- Model Layer --- //
    //let a_model = models::LinearModel::builder()
    //   .id("model".to_string())
    //    .build()
    //    .unwrap();

    // --- Initialize Metrics
    let _a_metrics = metrics::Metrics::classification_suite();

    // --- Initialize Loss Function
    let _a_loss = functions::Classification::CrossEntropy;

    // --- Initialize Optimizer
    let _a_optimizer = optimizers::Gradient::GradientDescent;

    // --- Trainer Environment (Singular) --- //
    // data: Features, Target
    // model: Weight definition, compute gradient
    // metrics: Model performance
    // loss: Formula definition, regularization
    // optimizer: step 

    // process: Train

    // let epochs = 100;

    // let training_process = processes::Singular::builder()
    //   .data(a_data)
    //    .model(a_model)
    //    .loss(a_loss)
    //    .optimizer(a_optimizer)
    //    .metrics(a_metrics)
    //    .build();

    // training_process.unwrap().train(epochs);

    // process: Evaluate
    //

    Ok(())
}


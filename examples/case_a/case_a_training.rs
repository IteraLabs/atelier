/// Single Agent Training

use atelier_dcml::{models, metrics};
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
   
    // --- Features file (csv)
    let data_file = workspace_root
        .join("examples")
        .join("case_a")
        .to_str()
        .unwrap().to_owned() + "/features_case_a.csv";
  
    // --- Read Features File
    let data = data::load_from_csv(&data_file);
    
    // --- Initialize Model
    let _lc_model = models::LogisticClassifier::builder()
        .id("model".to_string())
        .input_features(5)
        .glorot_uniform()
        .expect("Failed to initialize with Glorot Uniform")
        .build()
        .expect("Failed to build the model");
   
    // --- Initialize Metrics

    let _lc_metrics = metrics::Metrics::classification_suite();

    // --- Trainer Environment --- //
    // model
    // loss
    // optimizer
    // data
    // metrics
    // topology (only for distributed)

    Ok(())
}


/// Single Agent Training

use tch::{Tensor, Kind, IndexOp};
use std::{path::Path, error::Error};

use atelier_dcml::{
    agents::DistributedAgent,
    training,
    features,
    targets,
};

use atelier_core::{
    data,
    templates,
};

pub fn main() -> Result<(), Box<dyn Error + 'static>> {

    // --- Initial Setup and Folders

    // Setup working directory
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(manifest_dir)
        .parent()
        .expect("Failed to get workspace root");

    // Template file (toml)
    let template_file = workspace_root
        .join("examples")
        .join("case_a")
        .join("config_a.toml");
    let template = templates::Config::load_from_toml(template_file.to_str().unwrap())
        .unwrap()
        .clone();
   
    // Data file (json)
    let data_file = workspace_root
        .join("examples")
        .join("case_a")
        .join("data_a.json");
    let data = data::load_from_json(&data_file.to_str().unwrap().to_owned())?;

    // Extract parameters from template
    let n_progres = template.experiments[0].n_progressions as usize;

    Ok(())
}


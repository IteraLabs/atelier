use atelier_core::{templates, training};
use atelier_dcm::{
    agents::DistributedAgent, dataset, features, targets, training::distributed_training,
};

use std::{env, path::Path};
use tch::{display, Kind, Tensor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    display::set_print_options_full();

    // --- Set workdir
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(manifest_dir)
        .parent()
        .expect("Failed to get workspace root");

    // --- experiment parameters
    let experiment_route = workspace_root
        .join("atelier-dcm")
        .join("experiments")
        .join("distributed_training_00.toml")
        .to_str()
        .unwrap()
        .to_owned();

    let config = templates::Config::load_from_toml(&experiment_route)
        .unwrap()
        .clone();

    println!("config {:?}", config);

    Ok(())
}

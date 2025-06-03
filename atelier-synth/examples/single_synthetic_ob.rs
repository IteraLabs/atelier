use atelier_core::templates;
use atelier_dcml::math::Stats;
use atelier_synth::synthbooks::progressions;
use std::{env, fs::File, io::Write, path::Path};

#[tokio::main]
pub async fn main() {

    // --- Setup working directory
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(manifest_dir)
        .parent()
        .expect("Failed to get workspace root");

    // --- Template file (toml)
    let template_file = workspace_root
        .join("atelier-synth")
        .join("templates")
        .join("single_orderbook.toml");
    let template = templates::Config::load_from_toml(template_file.to_str().unwrap())
        .unwrap()
        .clone();

    // --- Extract parameters from template
    let exp_id = &template.experiments[0].id;
    let n_progres = template.experiments[0].n_progressions as usize;
    let template_orderbook = template.exchanges[0].orderbook.clone().unwrap();
    let template_model = template.models[0].clone();

    // --- Create progressions
    let v_rand_ob = progressions(template_orderbook, template_model, n_progres).await;

    // --- Compute basic stats
    let level_bids: Vec<f32> = v_rand_ob
        .as_ref()
        .unwrap()
        .iter()
        .map(|x| x.bids.len() as f32)
        .collect();

    let level_asks: Vec<f32> = v_rand_ob
        .as_ref()
        .unwrap()
        .iter()
        .map(|x| x.asks.len() as f32)
        .collect();

    let bids_stats = Stats::new(&level_bids);
    let asks_stats = Stats::new(&level_asks);

    println!("bids_stats: {:?}, asks_stats {:?}", bids_stats, asks_stats);

    let mut folder_route = workspace_root
        .join("atelier-synth")
        .join("datasets")
        .to_str()
        .unwrap()
        .to_owned();

    folder_route.push_str("/");
    folder_route.push_str(&exp_id);
    folder_route.push_str(&"_asia.json");

    let i_ob = v_rand_ob.as_ref().unwrap();

    let ob_json = serde_json::to_string(&i_ob).unwrap();
    let mut file = File::create(&folder_route).unwrap();
    file.write_all(ob_json.as_bytes()).unwrap();
}

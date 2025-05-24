use atelier_core::{
    orderbooks::{stats::Stats, Orderbook},
    templates,
};
use atelier_synth::synthbooks::async_progressions;
use std::{error::Error, path::Path};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // --- Setup working directory
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(manifest_dir)
        .parent()
        .expect("Failed to get workspace root");

    // --- Template file (toml)
    let template_file = workspace_root
        .join("atelier-synth")
        .join("templates")
        .join("multi_orderbooks.toml");
    let template = templates::Config::load_from_toml(template_file.to_str().unwrap())
        .unwrap()
        .clone();

    // --- Extract parameters from template
    let n_progres = template.experiments[0].n_progressions as usize;
    let v_template_orderbook = template
        .exchanges
        .into_iter()
        .map(|exchange| exchange.orderbook.unwrap())
        .collect();

    let v_template_model = template.models;

    let v_rand_ob =
        async_progressions(v_template_orderbook, v_template_model, n_progres).await;

    let result_obs: Result<
        Vec<Vec<Orderbook>>,
        Box<dyn std::error::Error + Send + Sync>,
        > = v_rand_ob.into_iter().collect();

        match result_obs {

            Ok(all_orderbooks) => {
                
                println!("all {:?} of orderbooks successfully generated",
                    &all_orderbooks.len());
                
                let v_rand_ob = &all_orderbooks[1];
                
                let level_bids: &Vec<f32> = &v_rand_ob
                    .iter()
                    .map(|x| x.bids.len() as f32)
                    .collect();
                let level_asks: &Vec<f32> = &v_rand_ob
                    .iter()
                    .map(|x| x.bids.len() as f32)
                    .collect();

                let bids_stats = Stats::new(&level_bids);
                let asks_stats = Stats::new(&level_asks);

                println!("bids_stats: {:?}, asks_stats {:?}", bids_stats, asks_stats);
            
            } 
            Err(e) => {
                eprintln!("At least one progression failed: {}", e);
            }
        }
    Ok(())
}

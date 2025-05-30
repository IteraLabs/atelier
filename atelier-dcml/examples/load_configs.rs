use atelier_core::templates;
use std::error::Error;

// ----------------------------------------------------------------------------------- //

fn main() -> Result<(), Box<dyn Error>> {
    let config_file = "atelier-dcm/Config_01.toml";

    // Load configuration
    let config = templates::Config::load_from_toml(config_file)
        .unwrap()
        .clone();
    let exp_id = &config.experiments[0].id;
    let mut files: Vec<_> = vec![];

    for i_e in 0..9 {
        // --- File Name Formation Orderbook -- //
        let mut folder_route = "atelier-dcm/datasets/".to_owned();
        let e_name = config.exchanges[i_e].clone().name.to_owned();
        let e_id = config.exchanges[i_e].clone().id.to_owned();

        folder_route.push_str(&exp_id);
        folder_route.push_str(&"_");
        folder_route.push_str(&e_id);
        folder_route.push_str(&"_");
        folder_route.push_str(&e_name);
        folder_route.push_str("_ob.json");
        files.push(folder_route);
    }

    println!("\nfolder/files/ routes: \n\n{:?}", files);

    Ok(())
}

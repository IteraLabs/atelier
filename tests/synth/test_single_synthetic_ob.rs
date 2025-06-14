#[cfg(test)]

// -- ----------------------------------------------------------------- TESTS UTILS -- //
// -- ----------------------------------------------------------------- ----------- -- //

mod tests {

    // -------------------------------------------------------- SINGLE SYNTHETIC OB -- //

    #[tokio::test]
    async fn test_single_synthetic_ob() {
        use atelier_data::templates;
        use atelier_synth::synthbooks::progressions;
        use std::{env, path::Path};

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

        println!("model: {:?}", template.models[0].clone());

        // --- Extract parameters from template
        let n_progres = template.experiments[0].n_progressions as usize;
        let template_orderbook = template.exchanges[0].orderbook.clone().unwrap();
        let template_model = template.models[0].clone();

        // --- Create progressions
        let _v_rand_ob =
            progressions(template_orderbook, template_model, n_progres).await;
    }
}

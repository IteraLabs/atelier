use atelier_data::orderbooks::Orderbook;
use std::{fs::File, io::BufReader};

// --------------------------------------------------------------------------- Asia Orderbook -- //
pub fn read_json(file_name: &str) -> Result<Vec<Orderbook>, Box<dyn std::error::Error>> {
    let file_route = "/Users/franciscome/git/iteralabs/atelier/atelier-dcm/examples/".to_owned();
    let folder_file = file_route + file_name;
    let file = File::open(folder_file)?;
    let reader = BufReader::new(file);
    let v_orderbook_2: Vec<Orderbook> = serde_json::from_reader(reader)?;

    Ok(v_orderbook_2)
}


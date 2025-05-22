use atelier_data::orderbooks::Orderbook;
use std::{fs::File, io::BufReader};

// ----------------------------------------------------------------- Asia Orderbook -- //

pub fn read_json(file_route: &str) -> Result<Vec<Orderbook>, Box<dyn std::error::Error>> {
    
    let file = File::open(file_route)?;
    let reader = BufReader::new(file);
    let v_orderbook_2: Vec<Orderbook> = serde_json::from_reader(reader)?;

    Ok(v_orderbook_2)
}

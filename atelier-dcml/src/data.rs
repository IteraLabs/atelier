use atelier_core::orderbooks::Orderbook;
use std::{fs::File, io::BufReader, error::Error};

// ----------------------------------------------------------------- Asia Orderbook -- //

pub fn read_json(file_route: &str) -> Result<Vec<Orderbook>, Box<dyn Error>> {
    let file = File::open(file_route)?;
    let reader = BufReader::new(file);
    let v_orderbook_2: Vec<Orderbook> = serde_json::from_reader(reader)?;

    Ok(v_orderbook_2)
}

#[derive(Debug)]
pub struct Dataset {
    pub features: Vec<f64>,
    pub labels: Vec<f64>,
}


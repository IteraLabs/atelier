
/// Market event generator module

use atelier::generators::hawkes::{self, HawkesProcess};
use atelier::data::market;
use atelier::messages::errors;
use std::time::{SystemTime, UNIX_EPOCH};


fn main() {

    // -------------------------------------------------------------- New Limit Order -- //
    // -------------------------------------------------------------- --------------- -- //

    // Definitive: parametric value
    let i_type = market::OrderType::Limit;
    
    // Currently: parametric variables
    let i_side = market::Side::Bids; 
    let i_price = 70_000.
    let i_amount = 10.50;

    // -- Define the Time-Window (nano seconds) : 1000
    let time_window = 1000;

    // -- Define the initial timestamp
    let initial_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as f64;

    // -- Specify parameters of the arrival-times generating model
    let h_mu: f64 = 0.85;
    let h_alpha: f64 = 0.80;
    let h_beta: f64 = 1.0;
    
    // -- Specify the model
    let model_lo = hawkes::HawkesProcess::new(h_mu, h_alpha, h_beta);
    // Number of events to generate by this instance of the generator
    let h_n: usize = 5;
    let parameters_lo = (h_n);
    
    let models_collection = vec![model_lo];
    let parameters_collection = vec![parameters_lo];

    pub enum Models {
        Hawkes(model_lo),
    }

    fn generate_new_order<T, P>(
        initial_ts: f64,
        models: Vec<Models>,
        params: Vec<usize>
    ) -> Result<(), errors::EventError> {

        // Currently: Static value, Definitive: independent parametric variable 
        let i_order_id = 1234;
        
        event_model = Models::Hawkes(HawkesProcess::new()
        )
   
        // -- Generate the values
        let synthetic_times = Models::Hawkes.generate_values(
            initial_ts,
            params[0]
        );
        
        println!("Synthetic Times: {:?}", synthetic_times);
    
        Ok(())
    }

}


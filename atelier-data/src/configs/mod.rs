use std::fs;
use std::process::exit;
use serde::Deserialize;
use toml:

#[derive(Debug, Clone)]
pub struct OrderbookConfigBuilder {
    ini_bid_price: Option<f64>,
    ini_bid_levels: Option<Vec<i32>>,
    ini_bid_orders: Option<Vec<i32>>,
    ini_ask_price: Option<f64>,
    ini_ask_levels: Option<Vec<i32>>,
    ini_ask_orders: Option<Vec<i32>>,
    ini_ticksize: Option<Vec<f64>>,
}

impl OrderbookConfigBuilder {
    pub fn new() -> Self {
        OrderbookConfigBuilder {
            ini_bid_price: None,
            ini_bid_levels: None,
            ini_bid_orders: None,
            ini_ask_price: None,
            ini_ask_levels: None,
            ini_ask_orders: None,
            ini_ticksize: None,
        }
    }

    pub fn ini_bid_price(mut self, ini_bid_price: f64) -> Self {
        self.ini_bid_price = Some(ini_bid_price);
        self
    }

    pub fn ini_bid_levels(mut self, ini_bid_levels: Vec<i32>) -> Self {
        self.ini_bid_levels = Some(ini_bid_levels);
        self
    }

    pub fn ini_bid_orders(mut self, ini_bid_orders: Vec<i32>) -> Self {
        self.ini_bid_orders = Some(ini_bid_orders);
        self
    }

    pub fn ini_ask_price(mut self, ini_ask_price: f64) -> Self {
        self.ini_ask_price = Some(ini_ask_price);
        self
    }
    pub fn ini_ask_levels(mut self, ini_ask_levels: Vec<i32>) -> Self {
        self.ini_ask_levels = Some(ini_ask_levels);
        self
    }

    pub fn ini_ask_orders(mut self, ini_ask_orders: Vec<i32>) -> Self {
        self.ini_ask_orders = Some(ini_ask_orders);
        self
    }
    
    pub fn ini_ticksize(mut self, ini_ticksize: Vec<f64>) -> Self {
        self.ini_ticksize = Some(ini_ticksize);
        self
    }

    pub fn build(self) -> Result<OrderbookConfig, &'static str> {
        let ini_bid_price = self.ini_bid_price.ok_or("Missing initial bid price")?;
        let ini_bid_levels = self.ini_bid_levels.ok_or("Missing initial bid levels")?;
        let ini_bid_orders = self.ini_bid_orders.ok_or("Missing initial bid orders")?;
        let ini_ask_price = self.ini_ask_price.ok_or("Missing initial ask price")?;
        let ini_ask_levels = self.ini_ask_levels.ok_or("Missing initial ask levels")?;
        let ini_ask_orders = self.ini_ask_orders.ok_or("Missing initial ask orders")?;
        let ini_ticksize = self.ini_ticksize.ok_or("Missing initial tick size")?;

        Ok(OrderbookConfig {
            ini_bid_price,
            ini_bid_levels,
            ini_bid_orders,
            ini_ask_price,
            ini_ask_levels,
            ini_ask_orders,
            ini_ticksize,
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrderbookConfig {
    // Orderbook generation
    ini_bid_price: f64,
    ini_bid_levels: Vec<i32>,
    ini_bid_orders: Vec<i32>,

    ini_ask_price: f64,
    ini_ask_levels: Vec<i32>,
    ini_ask_orders: Vec<i32>,

    ini_ticksize: Vec<f64>,

}

impl OrderbookConfig {
    
    pub fn builder() -> OrderbookConfigBuilder {
        OrderbookConfigBuilder::new()
    }

    pub fn loader(&self, filename) -> OrderbookConfig {

        let contents = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read the config file `{}`", filename);
                exit(1);
            }
        };

        let data: OrderbookConfig = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Unable to load data from `{}`", filename);
                exit(1);
            }
        };

        data
            
    }
}

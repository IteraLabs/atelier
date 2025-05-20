use serde::Deserialize;
use std::{fs, error::Error};
use toml;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub experiments: Vec<ExpConfig>,
    pub exchanges: Vec<ExchangeConfig>,
}

impl Config {
    pub fn load_from_toml(filename: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(filename)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExpConfig {
    pub id: String,
    pub n_progressions: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExchangeConfig {
    pub id: String,
    pub region: String,
    pub name: String,
    pub category: String,

    pub orderbook: Option<OrderbookConfig>,
    #[serde(default)]
    pub params: Vec<f64>
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrderbookConfig {
    
    pub bid_price: Option<f64>,
    pub bid_levels: Option<Vec<u32>>,
    pub bid_orders: Option<Vec<u32>>,
    pub ticksize: Option<Vec<f64>>,
    pub ask_price: Option<f64>,
    pub ask_levels: Option<Vec<u32>>,
    pub ask_orders: Option<Vec<u32>>,
    pub rands: Option<Vec<f64>>,

}

impl OrderbookConfig {

    pub fn builder() -> OrderbookConfigBuilder {
        OrderbookConfigBuilder::new()
    }

}

#[derive(Debug, Deserialize, Clone)]
pub struct OrderbookConfigBuilder {
    
    pub bid_price: Option<f64>,
    pub bid_levels: Option<Vec<u32>>,
    pub bid_orders: Option<Vec<u32>>,
    pub ticksize: Option<Vec<f64>>,
    pub ask_price: Option<f64>,
    pub ask_levels: Option<Vec<u32>>,
    pub ask_orders: Option<Vec<u32>>,
    pub rands: Option<Vec<f64>>,
    
}

impl OrderbookConfigBuilder {
    pub fn new() -> Self {
        OrderbookConfigBuilder {
            bid_price: None,
            bid_levels: None,
            bid_orders: None,
            ticksize: None,
            ask_price: None,
            ask_levels: None,
            ask_orders: None,
            rands: None,
        }
    }

    pub fn bid_price(mut self, bid_price: f64) -> Self {
        self.bid_price = Some(bid_price);
        self
    }

    pub fn bid_levels(mut self, bid_levels: Vec<u32> ) -> Self {
        self.bid_levels = Some(bid_levels);
        self
    }

    pub fn bid_orders(mut self, bid_orders: Vec<u32> ) -> Self {
        self.bid_orders = Some(bid_orders);
        self
    }

    pub fn ask_price(mut self, ask_price: f64) -> Self {
        self.ask_price = Some(ask_price);
        self
    }

    pub fn ask_levels(mut self, ask_levels: Vec<u32> ) -> Self {
        self.ask_levels = Some(ask_levels);
        self
    }

    pub fn ask_orders(mut self, ask_orders: Vec<u32> ) -> Self {
        self.ask_orders = Some(ask_orders);
        self
    }

    pub fn ticksize(mut self, ticksize: Vec<f64> ) -> Self {
        self.ticksize = Some(ticksize);
        self
    }

    pub fn build(self) -> Result<OrderbookConfig, &'static str> {
        let bid_price = self.bid_price.ok_or("Missing initial bid price")?;
        let bid_levels = self.bid_levels.ok_or("Missing initial bid levels")?;
        let bid_orders = self.bid_orders.ok_or("Missing initial bid orders")?;
        let ticksize = self.ticksize.ok_or("Missing initial tick size")?;
        let ask_price = self.ask_price.ok_or("Missing initial ask price")?;
        let ask_levels = self.ask_levels.ok_or("Missing initial ask levels")?;
        let ask_orders = self.ask_orders.ok_or("Missing initial ask orders")?;
        let rands = self.rands.ok_or("Missing initial random numbers")?;

        Ok(OrderbookConfig {
            bid_price: Some(bid_price),
            bid_levels: Some(bid_levels),
            bid_orders: Some(bid_orders),
            ticksize: Some(ticksize),
            ask_price: Some(ask_price),
            ask_levels: Some(ask_levels),
            ask_orders: Some(ask_orders),
            rands: Some(rands),
        })
    }
}



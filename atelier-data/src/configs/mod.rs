use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    ini_bid_price: Option<f64>,
    ini_bid_levels: Option<Vec<i32>>,
    ini_bid_orders: Option<Vec<i32>>,
    ini_ask_price: Option<f64>,
    ini_ask_levels: Option<Vec<i32>>,
    ini_ask_orders: Option<Vec<i32>>,
    ini_ticksize: Option<Vec<f64>>,
    uni_params: Option<Vec<f64>>,
    ber_params: Option<Vec<f64>>,
}

//  TODO: create InitParam, enum with variants like:
// Value: a single value (u32, or i32, or f32)
// Range: a range of (min(f32 / u32), max(f32 / u32)) values to later used as params for simulation,
// Generator: A type that has the iterator trait implemented, and with pre-loaded, or present loaded,
//            values to generate a secuence and iterate through.

impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder {
            ini_bid_price: None,
            ini_bid_levels: None,
            ini_bid_orders: None,
            ini_ask_price: None,
            ini_ask_levels: None,
            ini_ask_orders: None,
            ini_ticksize: None,
            uni_params: None,
            ber_params: None,
        }
    }

    pub fn ini_bid_price(mut self, ini_bid_price: f64) -> Self {
        self.ini_bid_price = Some(ini_bid_price);
        self
    }

    pub fn ini_ask_price(mut self, ini_ask_price: f64) -> Self {
        self.ini_ask_price = Some(ini_ask_price);
        self
    }

    pub fn ini_bid_levels(mut self, ini_bid_levels: Vec<i32>) -> Self {
        self.ini_bid_levels = Some(ini_bid_levels);
        self
    }

    pub fn ini_ask_levels(mut self, ini_ask_levels: Vec<i32>) -> Self {
        self.ini_ask_levels = Some(ini_ask_levels);
        self
    }

    pub fn ini_bid_orders(mut self, ini_bid_orders: Vec<i32>) -> Self {
        self.ini_bid_orders = Some(ini_bid_orders);
        self
    }

    pub fn ini_ask_orders(mut self, ini_ask_orders: Vec<i32>) -> Self {
        self.ini_ask_orders = Some(ini_ask_orders);
        self
    }

    pub fn build(self) -> Result<Config, &'static str> {
        let ini_bid_price = self.ini_bid_price.ok_or("Missing initial bid price")?;
        let ini_bid_levels = self.ini_bid_levels.ok_or("Missing initial bid levels")?;
        let ini_bid_orders = self.ini_bid_orders.ok_or("Missing initial bid orders")?;
        let ini_ask_price = self.ini_ask_price.ok_or("Missing initial ask price")?;
        let ini_ask_levels = self.ini_ask_levels.ok_or("Missing initial ask levels")?;
        let ini_ask_orders = self.ini_ask_orders.ok_or("Missing initial ask orders")?;
        let ini_ticksize = self.ini_ticksize.ok_or("Missing initial tick size")?;
        let uni_params = self
            .uni_params
            .ok_or("Missing params for random numbers: Uniform")?;
        let ber_params = self
            .ber_params
            .ok_or("Missing params for random numbers: Bernoulli")?;

        Ok(Config {
            ini_bid_price,
            ini_bid_levels,
            ini_bid_orders,

            ini_ask_price,
            ini_ask_levels,
            ini_ask_orders,
            ini_ticksize,
            uni_params,
            ber_params,
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    // Orderbook generation
    ini_bid_price: f64,
    ini_bid_levels: Vec<i32>,
    ini_bid_orders: Vec<i32>,

    ini_ask_price: f64,
    ini_ask_levels: Vec<i32>,
    ini_ask_orders: Vec<i32>,

    ini_ticksize: Vec<f64>,

    // Probabilistic Parameters
    uni_params: Vec<f64>,
    ber_params: Vec<f64>,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}

use atelier_core::orderbooks::Orderbook;

pub fn ob_price_direction(
    orderbooks: &Vec<Orderbook>,
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    // Compute midprices
    let mid_prices: Vec<f64> = orderbooks
        .iter()
        .map(|x| (x.asks[0].price + x.bids[0].price) / 2.0)
        .collect();

    // Compute up indicator: 1.0 if midprice increases, 0.0 otherwise
    let mut up_indicator = Vec::with_capacity(mid_prices.len());
    up_indicator.push(0.0); // First value, no previous to compare

    for i in 1..mid_prices.len() {
        if mid_prices[i] > mid_prices[i - 1] {
            up_indicator.push(1.0);
        } else {
            up_indicator.push(0.0);
        }
    }

    Ok(up_indicator)
}

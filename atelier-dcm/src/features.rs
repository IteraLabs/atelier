use atelier_core::orderbooks::Orderbook;

pub fn ob_vwap(
    orderbooks: &Vec<Orderbook>,
    depth: usize,
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let vwap: Vec<f64> = orderbooks
        .iter()
        .map(|ob| {
            let bid_levels = ob.bids.iter().take(depth);
            let ask_levels = ob.asks.iter().take(depth);
            let all_levels = bid_levels.chain(ask_levels);

            let (sum_px_vol, sum_vol) =
                all_levels.fold((0.0, 0.0), |(acc_px_vol, acc_vol), level| {
                    (
                        acc_px_vol + level.price * level.volume,
                        acc_vol + level.volume,
                    )
                });

            if sum_vol > 0.0 {
                sum_px_vol / sum_vol
            } else {
                0.0
            }
        })
        .collect();

    Ok(vwap)
}

pub fn ob_wmidprice(
    orderbooks: &Vec<Orderbook>,
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let vwap: Vec<f64> = orderbooks
        .iter()
        .map(|x| {
            ((x.bids[0].price * x.bids[0].volume) + (x.asks[0].price * x.asks[0].volume))
                / (x.asks[0].volume + x.bids[0].volume)
        })
        .collect();

    Ok(vwap)
}

pub fn ob_midprice(
    orderbooks: &Vec<Orderbook>,
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mid_price: Vec<f64> = orderbooks
        .iter()
        .map(|x| (x.asks[0].price + x.bids[0].price) / 2.0)
        .collect();

    Ok(mid_price)
}

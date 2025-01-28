
// Tutorial for Limit Order Book simple creation and manipulation

use atelier::data::market::Orderbook;

fn main() {

    let r_ob = Orderbook::random();

    // number of levels per side
    let n_bids = r_ob.bids.len();
    let n_asks = r_ob.asks.len();
    
    // mid price and total volume calculations
    let mid_price = (r_ob.asks[0].price + r_ob.bids[0].price) / 2.0;
    let volume_bids: f64 = r_ob.bids.clone().into_iter().map(|x| x.volume).sum();
    let volume_asks: f64 = r_ob.asks.clone().into_iter().map(|x| x.volume).sum();

    // random orders created at particular levels
    let n_orders_b0 = r_ob.bids[0].orders.len();
    let n_orders_a2 = r_ob.asks[2].orders.len();

    println!("\n-- Random Prices/Amounts generated --\n");
    println!("- Midprice: {:?}", mid_price);
    println!("- total volume in bids: {:?}", volume_bids);
    println!("- total volume in asks: {:?}", volume_asks);

    println!("\n-- Random Levels generated --\n");
    println!("- No. of levels in bids: {:?}", n_bids);
    println!("- No. of levels in asks: {:?}", n_asks);

    println!("\n-- Random Orders generated --\n");
    println!("- No. of Orders in the First level, at the Bids side: {:?}", n_orders_b0);
    println!("- No. of Orders in the Second level, at the Asks side: {:?}", n_orders_a2);
}


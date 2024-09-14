use atelier::data::market::Orderbook;

fn main() {
    let bid_price = 50_000.00;
    let ask_price = 50_100.00;
    let tick_size = 100.0;
    let n_levels = 200;
    let n_orders = 300;

    let i_ob = Orderbook::synthetize(bid_price, ask_price, tick_size, n_levels, n_orders);

    println!("\nlevel_id {:?}", i_ob.bids[199].level_id);
    println!("side {:?}", i_ob.bids[199].side);
    println!("price {:?}", i_ob.bids[199].price);
    println!("orders[0]{:?}", i_ob.bids[199].orders[0]);
    println!("orders[1]{:?}", i_ob.bids[199].orders[1]);
    println!(" ... ");

    println!("\nlevel_id {:?}", i_ob.bids[0].level_id);
    println!("side {:?}", i_ob.bids[0].side);
    println!("price {:?}", i_ob.bids[0].price);
    println!("orders[0]{:?}", i_ob.bids[0].orders[0]);
    println!("orders[1]{:?}", i_ob.bids[0].orders[1]);
    println!(" ... ");

    let mid_price = (i_ob.bids[0].price + i_ob.asks[0].price) / 2.0;
    println!("Midprice: {}", mid_price);

    println!("\nlevel_id {:?}", i_ob.asks[0].level_id);
    println!("side {:?}", i_ob.asks[0].side);
    println!("price {:?}", i_ob.asks[0].price);
    println!("orders[0]{:?}", i_ob.asks[0].orders[0]);
    println!("orders[1]{:?}", i_ob.asks[0].orders[1]);
    println!(" ... ");

    println!("\nlevel_id {:?}", i_ob.asks[199].level_id);
    println!("side {:?}", i_ob.asks[199].side);
    println!("price {:?}", i_ob.asks[199].price);
    println!("orders[0]{:?}", i_ob.asks[199].orders[0]);
    println!("orders[1]{:?}", i_ob.asks[199].orders[1]);
    println!(" ... ");
}

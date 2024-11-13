use plotly::{Bar, Layout, Plot};

fn main() {
    // Sample data for the Limit Order Book
    let bids = vec![(9900, 100), (9950, 200), (10000, 300)];
    let asks = vec![(10050, 150), (10100, 250), (10150, 100)];

    // Prepare data for plotting
    let (bid_prices, bid_volumes): (Vec<f64>, Vec<f64>) =
        bids.into_iter().map(|(p, v)| (p as f64, v as f64)).unzip();

    let (ask_prices, ask_volumes): (Vec<f64>, Vec<f64>) =
        asks.into_iter().map(|(p, v)| (p as f64, v as f64)).unzip();

    // Create bid trace
    let bid_trace = Bar::new(bid_prices, bid_volumes)
        .name("Bids")
        .marker(plotly::common::Marker::new().color("green"));

    // Create ask trace
    let ask_trace = Bar::new(ask_prices, ask_volumes)
        .name("Asks")
        .marker(plotly::common::Marker::new().color("red"));

    // Create the layout
    let layout = Layout::new()
        .title("Limit Order Book".into())
        .x_axis(plotly::layout::Axis::new().title("Price".into()))
        .y_axis(plotly::layout::Axis::new().title("Volume".into()))
        .bar_mode(plotly::layout::BarMode::Group);

    // Create the plot
    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(bid_trace);
    plot.add_trace(ask_trace);

    // Save the plot as an HTML file
    plot.write_html("limit_order_book.html");
    println!("Limit Order Book plot has been saved to limit_order_book.html");
}

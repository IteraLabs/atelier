use std::{ops::Deref, time::Duration};

use atelier::net::OrderBook as SyncBook;
use atelier::net::OrderBook;
use tokio::{sync::mpsc::unbounded_channel, time::interval};
use tracing_subscriber::EnvFilter;
use trolly::{
    monitor::Depth,
    net::MultiSymbolStream,
    providers::{self},
};

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .init();

    let (tx, mut rx) = unbounded_channel();

    let local = tokio::task::LocalSet::new();

    tokio::spawn(async move {
        // BTCUSDT book
        let btc_book: OrderBook = rx.recv().await.unwrap();
        // ETHUSDT book
        let eth_book = rx.recv().await.unwrap();

        let mut every = interval(Duration::from_millis(5_000));
        every.tick().await;

        loop {
            every.tick().await;
            let btc_snapshot = btc_book.0.read().unwrap();
            println!("{:?}", &*btc_snapshot.deref());
            let eth_snapshot = eth_book.0.read().unwrap();
            println!("{:?}", &*eth_snapshot.deref());
        }
    });

    local
        .run_until(async move {
            let symbols = &["BTCUSDT".into(), "ETHUSDT".into()];
            MultiSymbolStream::stream::<Depth, SyncBook, _, _>(providers::Binance, tx, symbols)
                .await
        })
        .await;
}

use std::sync::{Arc, RwLock};

use tokio::sync::mpsc::UnboundedSender;
use trolly::monitor::DepthHandler;

use crate::data::market::{Level, Orderbook as L2Book};

// Used to access the book updated by the stream.
#[derive(Debug, Clone)]
pub struct OrderBook(pub Arc<std::sync::RwLock<L2Book>>);

impl DepthHandler for OrderBook {
    type Error = anyhow::Error;
    type Context = UnboundedSender<Self>;

    fn handle_update(&mut self, update: trolly::lob::DepthUpdate) -> Result<(), Self::Error> {
        let mut book = self.0.write().expect("not poisoned");

        if update.skip_update(book.orderbook_id) {
            return Ok(());
        }
        for bid in update.bids.iter() {
            book.insert_bid(Level::new(0, bid.0, bid.1, vec![]));
        }

        for ask in update.asks.iter() {
            book.insert_ask(Level::new(0, ask.0, ask.1, vec![]));
        }

        book.orderbook_id = update.last_update_id;

        Ok(())
    }

    async fn build<En>(
        provider: En,
        symbols: &[String],
        sender: Self::Context,
    ) -> Result<Self, Self::Error>
    where
        En: trolly::providers::Endpoints<trolly::monitor::Depth>,
        Self: Sized,
    {
        //query book snapshot

        // expect a single simble here.
        let symbol = &symbols[0];
        //query
        let response: reqwest::Response = {
            let url = provider.rest_api_url(symbol);
            reqwest::get(url).await
        }?;

        response.error_for_status_ref()?;

        let mut snapshot: L2Book = response.json().await?;
        snapshot.symbol = symbol.clone();
        let lob = OrderBook(Arc::new(RwLock::new(snapshot)));

        sender.send(lob.clone()).unwrap();

        Ok(lob)
    }
}

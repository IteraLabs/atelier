/// Channels
use crate::events::message::MarketEventType;
use crossbeam::channel::unbounded;

pub fn create_channel() -> (
    crossbeam::channel::Sender<MarketEventType>,
    crossbeam::channel::Receiver<MarketEventType>,
) {
    let (channel_sender, channel_receiver) = unbounded::<MarketEventType>();
    (channel_sender, channel_receiver)
}

fn main() {
    let (cancel_lo_s, cancel_lo_r) = create_channel();
    let (new_mo_s, new_mo_r) = create_channel();
    let (modify_lo_s, modify_lo_r) = create_channel();
    let (new_mo_s, new_mo_r) = create_channel();
}

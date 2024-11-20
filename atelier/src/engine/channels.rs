use crossbeam::channel::unbounded;
use crate::events::templates::MarketEventType;

pub fn create_channel(

) -> (
    crossbeam::channel::Sender<MarketEventType>,
    crossbeam::channel::Receiver<MarketEventType>,
) {
    let (channel_sender, channel_receiver) = unbounded::<MarketEventType>();
    (channel_sender, channel_receiver)
}

fn main(){
    let (cancel_lo_queue_s, cancel_lo_queue_r) = create_channel();
    let (new_mo_queue_s, cancel_mo_queue_r) = create_channel();

}


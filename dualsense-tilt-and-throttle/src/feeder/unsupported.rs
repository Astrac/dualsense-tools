use crate::{
    emulated::EmulatedGamepad,
    feeder::{EmulatedStateFeeder, FeederId},
};

#[derive(Default, Debug)]
pub struct Unsupported;

impl EmulatedStateFeeder for Unsupported {
    fn feed_state(&mut self, _state: &EmulatedGamepad) -> Result<(), super::error::Error> {
        Ok(())
    }

    fn id(&self) -> FeederId {
        FeederId::Unsupported
    }
}

use crate::{emulated::EmulatedGamepad, feeder::EmulatedStateFeeder};

pub struct Dummy;

impl EmulatedStateFeeder for Dummy {
    fn feed_state(&mut self, _state: &EmulatedGamepad) -> Result<(), super::error::Error> {
        Ok(())
    }
}

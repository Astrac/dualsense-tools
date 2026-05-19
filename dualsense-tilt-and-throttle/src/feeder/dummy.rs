use crate::{emulated::EmulatedGamepad, feeder::Feeder};

pub struct Dummy;

impl Feeder for Dummy {
    fn feed_state(&mut self, _state: &EmulatedGamepad) -> Result<(), super::Error> {
        Ok(())
    }
}

use crossbeam_channel::Receiver;

use crate::{
    feeder::{ConfiguredFeeder, Feeder as _, Feeders, backend},
    threads::PollingEvent,
};

pub struct Feeder {
    polling_events: Receiver<PollingEvent>,
}

impl Feeder {
    pub fn new(polling_events: Receiver<PollingEvent>) -> Feeder {
        Feeder { polling_events }
    }

    pub fn run(&self) -> color_eyre::Result<()> {
        let backend = backend::auto();
        let mut feeders = Feeders::new().unwrap();
        let feeder_config = feeders.next();
        let mut feeder = ConfiguredFeeder::new(backend, &feeder_config);

        for event in self.polling_events.iter() {
            if let PollingEvent::StateAvailable(state) = event {
                feeder.feed(&state).unwrap();
            }
        }

        Ok(())
    }
}

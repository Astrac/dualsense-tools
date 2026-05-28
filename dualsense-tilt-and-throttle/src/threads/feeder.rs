use tokio::sync::broadcast::Receiver;

use crate::{
    feeder::{ConfiguredFeeder, Feeder as _, Feeders, backend},
    threads::{Command, PollingEvent},
};

pub struct Feeder {
    polling_events: Receiver<PollingEvent>,
    commands: Receiver<Command>,
}

impl Feeder {
    pub fn new(polling_events: Receiver<PollingEvent>, commands: Receiver<Command>) -> Feeder {
        Feeder {
            polling_events,
            commands,
        }
    }

    pub async fn run(&mut self) -> color_eyre::Result<()> {
        let mut backend = backend::auto();
        let mut feeders = Feeders::new().unwrap();
        let feeder_config = feeders.next();
        let mut feeder = ConfiguredFeeder::new(&mut backend, &feeder_config);

        loop {
            tokio::select! {
                event = self.polling_events.recv() =>
                    // TODO: Handle backend errors and raise events to update UI
                    if let PollingEvent::StateAvailable(state) = event? {
                        feeder.feed(&state).unwrap();
                    },
                event = self.commands.recv() =>
                    match event? {
                        Command::NextFeeder => {
                            feeder = ConfiguredFeeder::new(&mut backend, &feeders.next());
                            log::info!("Feeder selected: {}", feeder.config.description)
                        }
                        Command::Quit => {
                            break
                        }
                    },
            }
        }

        log::info!("Feeder quitting");
        Ok(())
    }
}

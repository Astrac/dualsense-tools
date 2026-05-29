use std::sync::{Arc, Mutex};

use tokio::sync::broadcast::{Receiver, Sender, error::SendError};

use crate::{
    feeder::{
        Feeder, Feeders,
        backend::{Backend, BackendError, BackendId},
    },
    threads::{Command, PollingEvent},
};

#[derive(Clone, Debug)]
pub enum FeederEvent {
    Selected { backend: BackendId, feeder: String },
    BackendError(String),
}

pub struct Feeding<B: Backend> {
    backend: Arc<Mutex<B>>,
    backend_id: BackendId,
    feeder_events: Sender<FeederEvent>,
    polling_events: Receiver<PollingEvent>,
    commands: Receiver<Command>,
    feeder_configs: Feeders,
    running_feeder: Option<Feeder<B>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ShouldQuit(bool);

impl<B: Backend> Feeding<B> {
    pub fn new(
        backend: B,
        feeder_events: Sender<FeederEvent>,
        polling_events: Receiver<PollingEvent>,
        commands: Receiver<Command>,
    ) -> Feeding<B> {
        let backend_id = backend.name();
        Feeding {
            backend: Arc::new(Mutex::new(backend)),
            feeder_configs: Feeders::new().unwrap(),
            backend_id,
            feeder_events,
            polling_events,
            commands,
            running_feeder: None,
        }
    }

    pub async fn run(&mut self) -> color_eyre::Result<()> {
        self.next_feeder()?;
        loop {
            let result = self.handle_inputs().await?;

            if result == ShouldQuit(true) {
                break;
            }
        }

        Ok(())
    }

    async fn handle_inputs(&mut self) -> color_eyre::Result<ShouldQuit> {
        let should_quit = tokio::select! {
            event = self.polling_events.recv() => {
                self.handle_event(event?)?;
                ShouldQuit(false)
            },
            command = self.commands.recv() => self.handle_command(command?)?
        };

        Ok(should_quit)
    }

    fn handle_event(&mut self, event: PollingEvent) -> Result<(), SendError<FeederEvent>> {
        if let PollingEvent::StateAvailable(state) = event
            && let Some(feeder) = &mut self.running_feeder
        {
            let result = feeder.feed(&state);
            self.handle_backend_errors(&result)?;
        }

        Ok(())
    }

    fn handle_command(&mut self, command: Command) -> Result<ShouldQuit, SendError<FeederEvent>> {
        match command {
            Command::NextFeeder => self.next_feeder()?,
            Command::Quit => {
                return Ok(ShouldQuit(true));
            }
        }

        Ok(ShouldQuit(false))
    }

    fn next_feeder(&mut self) -> Result<(), SendError<FeederEvent>> {
        let new_feeder = Feeder::new(self.backend.clone(), self.feeder_configs.next().clone());

        self.handle_backend_errors(&new_feeder)?;
        self.running_feeder = new_feeder.ok();

        if let Some(feeder) = &self.running_feeder {
            self.feeder_events.send(FeederEvent::Selected {
                feeder: feeder.config.description.clone(),
                backend: self.backend_id,
            })?;

            log::info!("Feeder selected: {}", feeder.config.description)
        }

        Ok(())
    }

    fn handle_backend_errors<A>(
        &mut self,
        result: &Result<A, BackendError>,
    ) -> Result<(), SendError<FeederEvent>> {
        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("Backend error: {:?}", err);
                self.feeder_events
                    .send(FeederEvent::BackendError(err.description.clone()))?;
                self.running_feeder = None;
                Ok(())
            }
        }
    }
}

use std::sync::{Arc, Mutex};

use tokio::sync::broadcast::Receiver;

use crate::{
    term_ui::{DualsenseStatus, FeederStatus, UiState},
    threads::{Command, FeederEvent, PollingEvent},
    virtual_controller::VirtualControllerState,
};

pub struct UpdatingUI {
    feeder_events: Receiver<FeederEvent>,
    polling_events: Receiver<PollingEvent>,
    commands: Receiver<Command>,
    ui_state: Arc<Mutex<UiState>>,
}

impl UpdatingUI {
    pub fn new(
        feeder_events: Receiver<FeederEvent>,
        polling_events: Receiver<PollingEvent>,
        commands: Receiver<Command>,
        ui_state: Arc<Mutex<UiState>>,
    ) -> UpdatingUI {
        UpdatingUI {
            feeder_events,
            polling_events,
            commands,
            ui_state,
        }
    }

    pub async fn run(&mut self) -> color_eyre::Result<()> {
        loop {
            tokio::select! {
                event = self.polling_events.recv() => self.handle_polling_event(event?),
                event = self.feeder_events.recv() => self.handle_feeder_event(event?),
                command = self.commands.recv() => {
                    match command? {
                        Command::Quit => {
                            break
                        }
                        _ => ()
                    }
                },
            }
        }

        log::info!("Quitting UI state updater");

        Ok(())
    }

    fn handle_feeder_event(&self, event: FeederEvent) {
        let mut ui_state = self.ui_state.lock().unwrap();
        match event {
            FeederEvent::Selected { backend, feeder } => {
                ui_state.feeder.backend = backend;
                ui_state.feeder.name = feeder;
                ui_state.feeder.status = FeederStatus::Running;
            }
            FeederEvent::BackendError(err) => ui_state.feeder.status = FeederStatus::Error(err),
        }
    }

    fn handle_polling_event(&self, event: PollingEvent) {
        match event {
            PollingEvent::StateAvailable(state) => self.update_state(state),
            PollingEvent::Connected => self.connect_ds(),
            PollingEvent::Disconnected => self.disconnect_ds(),
        }
    }

    fn disconnect_ds(&self) {
        let mut ui_state = self.ui_state.lock().unwrap();
        ui_state.dualsense = DualsenseStatus::Disconnected;
        ui_state.virtual_controller = Default::default();
    }

    fn connect_ds(&self) {
        self.ui_state.lock().unwrap().dualsense = DualsenseStatus::Connected
    }

    fn update_state(&self, state: VirtualControllerState) {
        self.ui_state.lock().unwrap().virtual_controller = state
    }
}

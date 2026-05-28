use std::sync::{Arc, Mutex};

use tokio::sync::broadcast::Receiver;

use crate::{
    term_ui::{DualsenseStatus, RenderState},
    threads::{Commands, PollingEvent},
    virtual_controller::VirtualControllerState,
};

pub struct UIUpdater {
    polling_events: Receiver<PollingEvent>,
    commands: Receiver<Commands>,
    ui_state: Arc<Mutex<RenderState>>,
}

impl UIUpdater {
    pub fn new(
        polling_events: Receiver<PollingEvent>,
        commands: Receiver<Commands>,
        ui_state: Arc<Mutex<RenderState>>,
    ) -> UIUpdater {
        UIUpdater {
            polling_events,
            commands,
            ui_state,
        }
    }

    pub async fn run(&mut self) -> color_eyre::Result<()> {
        loop {
            tokio::select! {
                event = self.polling_events.recv() => self.handle_polling_event(event?),
                command = self.commands.recv() => {
                    match command? {
                        Commands::Quit => {
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

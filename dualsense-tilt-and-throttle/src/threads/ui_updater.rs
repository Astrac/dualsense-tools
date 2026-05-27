use std::sync::{Arc, Mutex};

use crossbeam_channel::Receiver;

use crate::{
    term_ui::{DualsenseStatus, RenderState},
    threads::PollingEvent,
};

pub struct UIUpdater {
    polling_events: Receiver<PollingEvent>,
    ui_state: Arc<Mutex<RenderState>>,
}

impl UIUpdater {
    pub fn new(
        polling_events: Receiver<PollingEvent>,
        ui_state: Arc<Mutex<RenderState>>,
    ) -> UIUpdater {
        UIUpdater {
            polling_events,
            ui_state,
        }
    }

    pub fn run(&self) -> color_eyre::Result<()> {
        for event in self.polling_events.iter() {
            let mut render_state = self.ui_state.lock().unwrap();

            match event {
                PollingEvent::StateAvailable(state) => render_state.virtual_controller = state,
                PollingEvent::Connected => render_state.dualsense = DualsenseStatus::Connected,
                PollingEvent::Disconnected => {
                    render_state.dualsense = DualsenseStatus::Disconnected;
                    render_state.virtual_controller = Default::default();
                }
            }
        }

        Ok(())
    }
}

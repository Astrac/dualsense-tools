use std::time::Duration;

use color_eyre::eyre::Result;
use dualsense_tools::Dualsense;
use hidapi::HidApi;
use tokio::sync::broadcast::{Receiver, Sender};

use crate::{
    threads::Commands,
    virtual_controller::{VirtualController, VirtualControllerState},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PollingEvent {
    Connected,
    Disconnected,
    StateAvailable(VirtualControllerState),
}

pub struct Poller<const N: usize> {
    hid_api: HidApi,
    controller: VirtualController<N>,
    commands: Receiver<Commands>,
    polling_events: Sender<PollingEvent>,
    poll_period: Duration,
    device: Option<Dualsense>,
}

impl<const N: usize> Poller<N> {
    pub fn new(
        hid_api: HidApi,
        controller: VirtualController<N>,
        commands: Receiver<Commands>,
        polling_events: Sender<PollingEvent>,
        poll_period: Duration,
    ) -> Poller<N> {
        Poller {
            commands,
            controller,
            hid_api,
            poll_period,
            polling_events,
            device: None,
        }
    }
}

impl<const N: usize> Poller<N> {
    pub fn run(&mut self) -> Result<()> {
        loop {
            if self.commands.try_recv() == Ok(Commands::Quit) {
                break;
            }

            let event = match self.device {
                Some(ref mut d) => match d.read() {
                    Ok(ds_state) => {
                        let state = self.controller.handle_dualsense(ds_state);
                        Some(PollingEvent::StateAvailable(state))
                    }
                    Err(err) => {
                        log::info!("Cannot read state of controller - error: {}", err);
                        self.device = None;
                        Some(PollingEvent::Disconnected)
                    }
                },
                None => {
                    if let Ok(ds) = Dualsense::new(&mut self.hid_api) {
                        log::info!("Connected new dualsense device");
                        self.device = Some(ds);
                        Some(PollingEvent::Connected)
                    } else {
                        log::debug!("No device found");
                        None
                    }
                }
            };

            if let Some(ev) = event {
                self.polling_events.send(ev)?;
            }

            std::thread::sleep(self.poll_period);
        }

        log::info!("Poller quitting");
        Ok(())
    }
}

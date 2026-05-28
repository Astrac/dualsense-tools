use std::time::Duration;

use color_eyre::eyre::Result;
use dualsense_tools::{Dualsense, TiltEstimator, TiltEstimatorConfig};
use tokio::sync::broadcast::{Receiver, Sender};

use crate::{
    threads::Command,
    virtual_controller::{VirtualController, VirtualControllerState},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PollingEvent {
    Connected,
    Disconnected,
    StateAvailable(VirtualControllerState),
}

pub struct Poller<const N: usize> {
    tilt_estimator_config: TiltEstimatorConfig<N>,
    commands: Receiver<Command>,
    polling_events: Sender<PollingEvent>,
    poll_period: Duration,
    device: Option<Dualsense>,
}

impl<const N: usize> Poller<N> {
    pub fn new(
        tilt_estimator_config: TiltEstimatorConfig<N>,
        commands: Receiver<Command>,
        polling_events: Sender<PollingEvent>,
        poll_period: Duration,
    ) -> Poller<N> {
        Poller {
            tilt_estimator_config,
            commands,
            poll_period,
            polling_events,
            device: None,
        }
    }
}

impl<const N: usize> Poller<N> {
    pub fn run(&mut self) -> Result<()> {
        let mut hid_api = hidapi::HidApi::new()?;
        log::info!("Hid API initialized");

        let mut controller = VirtualController::new(TiltEstimator::new(self.tilt_estimator_config));

        loop {
            if self.commands.try_recv() == Ok(Command::Quit) {
                break;
            }

            let event = match self.device {
                Some(ref mut d) => match d.read() {
                    Ok(ds_state) => {
                        let state = controller.handle_dualsense(ds_state);
                        Some(PollingEvent::StateAvailable(state))
                    }
                    Err(err) => {
                        log::info!("Cannot read state of controller - error: {}", err);
                        self.device = None;
                        Some(PollingEvent::Disconnected)
                    }
                },
                None => {
                    if let Ok(ds) = Dualsense::new(&mut hid_api) {
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

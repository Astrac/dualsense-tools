mod app;
mod feeder;
mod term_ui;
mod virtual_controller;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use dualsense_tools::{Dualsense, TiltEstimator, TiltEstimatorConfig};

use crate::feeder::Feeder;
use crate::{
    feeder::{ConfiguredFeeder, Feeders},
    term_ui::{DualsenseStatus, RenderState},
    virtual_controller::{VirtualController, VirtualControllerState},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Commands {
    Quit,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PollingEvents {
    Connected,
    Disconnected,
    StateAvailable(VirtualControllerState),
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut api = hidapi::HidApi::new().expect("Cannot initialize HID API");

    let tilt_estimator = TiltEstimator::<20>::new(TiltEstimatorConfig::default());
    let mut controller = VirtualController::new(tilt_estimator);
    let frame_duration = Duration::from_millis(15);

    let (polling_tx, polling_rx) = crossbeam_channel::bounded(20);
    let (command_tx, command_rx) = crossbeam_channel::bounded(20);

    let polling_sender = polling_tx.clone();
    let polling = thread::spawn(move || {
        let mut device: Option<Dualsense> = None;

        loop {
            let event = match device {
                Some(ref mut d) => match d.read() {
                    Ok(ds_state) => {
                        let state = controller.handle_dualsense(ds_state);
                        Some(PollingEvents::StateAvailable(state))
                    }
                    Err(_) => {
                        device = None;
                        Some(PollingEvents::Disconnected)
                    }
                },
                None => {
                    if let Ok(ds) = Dualsense::new(&mut api) {
                        device = Some(ds);
                        Some(PollingEvents::Connected)
                    } else {
                        None
                    }
                }
            };

            if let Some(ev) = event {
                polling_sender.send(ev).unwrap_or_else(|e| {
                    panic!("Cannot send polling event: {event:?} - Error: {e}")
                });
            }

            if command_rx.try_recv() == Ok(Commands::Quit) {
                break;
            }

            thread::sleep(frame_duration);
        }
    });

    let feeding_rx = polling_rx.clone();
    let feeding = thread::spawn(move || {
        let backend = feeder::backend::auto();
        let mut feeders = Feeders::new().unwrap();
        let feeder_config = feeders.next();
        let mut feeder = ConfiguredFeeder::new(backend, &feeder_config);

        for event in feeding_rx {
            if let PollingEvents::StateAvailable(state) = event {
                feeder.feed(&state).unwrap();
            }
        }
    });

    let render_state_shared = Arc::new(Mutex::new(RenderState::new("TODO")));
    let render_state_updater_rx = polling_rx.clone();
    let render_state_updater_state = render_state_shared.clone();
    let render_state_updater = thread::spawn(move || {
        for event in render_state_updater_rx {
            let mut render_state = render_state_updater_state.lock().unwrap();
            match event {
                PollingEvents::StateAvailable(state) => render_state.virtual_controller = state,
                PollingEvents::Connected => render_state.dualsense = DualsenseStatus::Connected,
                PollingEvents::Disconnected => {
                    render_state.dualsense = DualsenseStatus::Disconnected;
                    render_state.virtual_controller = Default::default();
                }
            }
        }
    });

    app::start(render_state_shared, frame_duration)?;

    command_tx.send(Commands::Quit).unwrap();

    polling.join().unwrap();
    feeding.join().unwrap();
    render_state_updater.join().unwrap();

    Ok(())
}

mod app;
mod emulated;
mod emulated_axis_value;
mod emulator;
mod feeder;
mod term_ui;

use crate::{
    emulated::EmulatedGamepad,
    emulator::Emulator,
    feeder::EmulatedStateFeeder,
    term_ui::{DualsenseStatus, RenderState},
};
use dualsense_tools::{Dualsense, TiltEstimator, TiltEstimatorConfig};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Commands {
    Quit,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PollingEvents {
    Connected,
    Disconnected,
    StateAvailable(EmulatedGamepad),
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut api = hidapi::HidApi::new().expect("Cannot initialize HID API");

    let tilt_estimator = TiltEstimator::<20>::new(TiltEstimatorConfig::default());
    let mut emulator = Emulator::new(tilt_estimator);
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
                        let state = emulator.handle_dualsense_state(ds_state);
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

    let mut feeder = feeder::Feeder::auto()?;
    let feeder_id = feeder.id();
    let feeding_rx = polling_rx.clone();
    let feeding = thread::spawn(move || {
        for event in feeding_rx {
            if let PollingEvents::StateAvailable(state) = event {
                feeder.feed_state(&state).unwrap();
            }
        }
    });

    let render_state_shared = Arc::new(Mutex::new(RenderState::new(feeder_id)));
    let render_state_updater_rx = polling_rx.clone();
    let render_state_updater_state = render_state_shared.clone();
    let render_state_updater = thread::spawn(move || {
        for event in render_state_updater_rx {
            let mut render_state = render_state_updater_state.lock().unwrap();
            match event {
                PollingEvents::StateAvailable(state) => render_state.emulation = state,
                PollingEvents::Connected => render_state.dualsense = DualsenseStatus::Connected,
                PollingEvents::Disconnected => {
                    render_state.dualsense = DualsenseStatus::Disconnected;
                    render_state.emulation = EmulatedGamepad::default();
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

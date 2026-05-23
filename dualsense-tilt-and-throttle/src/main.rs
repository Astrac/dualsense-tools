mod app;
mod emulated;
mod emulated_axis_value;
mod emulator;
mod feeder;
mod term_ui;

use crate::{emulator::Emulator, feeder::EmulatedStateFeeder};
use dualsense_tools::{Dualsense, TiltEstimator, TiltEstimatorConfig};
use std::{thread, time::Duration};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Commands {
    Quit,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut api = hidapi::HidApi::new()?;
    let device = Dualsense::new(&mut api)?;
    let tilt_estimator = TiltEstimator::<20>::new(TiltEstimatorConfig::default());
    let emulator = Emulator::new(device, tilt_estimator);
    let frame_duration = Duration::from_millis(15);

    // From polling to others
    let (state_tx, state_rx) = crossbeam_channel::bounded(20);
    // From UI to polling
    let (command_tx, command_rx) = crossbeam_channel::bounded(20);

    let polling = thread::spawn(move || {
        for state in emulator {
            if command_rx.try_recv() == Ok(Commands::Quit) {
                break;
            }
            state_tx.send(state).unwrap();
            thread::sleep(frame_duration);
        }
    });

    let mut feeder = feeder::Feeder::auto()?;
    let feeder_description = feeder.description();
    let feeding_rx = state_rx.clone();
    let feeding = thread::spawn(move || {
        for state in feeding_rx {
            feeder.feed_state(&state).unwrap();
        }
    });

    let displaying_rx = state_rx.clone();
    app::start(displaying_rx, feeder_description, frame_duration)?;

    command_tx.send(Commands::Quit).unwrap();

    polling.join().unwrap();
    feeding.join().unwrap();

    Ok(())
}

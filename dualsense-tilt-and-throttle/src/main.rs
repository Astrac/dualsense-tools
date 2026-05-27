mod feeder;
mod term_ui;
mod threads;
mod virtual_controller;
mod window;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use dualsense_tools::{TiltEstimator, TiltEstimatorConfig};

use crate::{term_ui::RenderState, virtual_controller::VirtualController};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Commands {
    Quit,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let api = hidapi::HidApi::new().expect("Cannot initialize HID API");

    let tilt_estimator = TiltEstimator::<20>::new(TiltEstimatorConfig::default());
    let controller = VirtualController::new(tilt_estimator);
    let frame_duration = Duration::from_millis(15);

    let (polling_tx, polling_rx) = crossbeam_channel::bounded(20);
    let (command_tx, command_rx) = crossbeam_channel::bounded(20);

    let render_state_shared = Arc::new(Mutex::new(RenderState::new("TODO")));

    let mut poller = threads::Poller::new(
        api,
        controller,
        command_rx.clone(),
        polling_tx.clone(),
        frame_duration,
    );

    let feeder = threads::Feeder::new(polling_rx.clone());
    let ui_updater = threads::UIUpdater::new(polling_rx.clone(), render_state_shared.clone());

    let polling = thread::spawn(move || poller.run().unwrap());
    let feeding = thread::spawn(move || feeder.run().unwrap());
    let ui_updating = thread::spawn(move || ui_updater.run().unwrap());

    window::init(render_state_shared, frame_duration)?;

    command_tx.send(Commands::Quit).unwrap();

    polling.join().unwrap();
    feeding.join().unwrap();
    ui_updating.join().unwrap();

    Ok(())
}

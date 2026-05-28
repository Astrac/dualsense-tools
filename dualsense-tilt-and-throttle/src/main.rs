mod feeder;
mod term_ui;
mod threads;
mod virtual_controller;
mod window;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use dualsense_tools::{TiltEstimator, TiltEstimatorConfig};
use spdlog::formatter::FullFormatter;

use crate::{term_ui::RenderState, virtual_controller::VirtualController};

/// In Hertz
const POLL_FREQUENCY: u8 = 120;
const POLL_PERIOD: Duration = Duration::from_nanos(1000000 / POLL_FREQUENCY as u64);

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_log()?;

    log::set_max_level(log::LevelFilter::Info);

    log::info!("Initializing...");

    let api = hidapi::HidApi::new().expect("Cannot initialize HID API");
    log::info!("Hid API initialized");

    let tilt_estimator = TiltEstimator::<20>::new(TiltEstimatorConfig::default());
    let controller = VirtualController::new(tilt_estimator);

    let (polling_tx, polling_rx) = tokio::sync::broadcast::channel(100);
    let (command_tx, command_rx) = tokio::sync::broadcast::channel(100);

    let render_state_shared = Arc::new(Mutex::new(RenderState::new("TODO")));

    let mut poller = threads::Poller::new(
        api,
        controller,
        command_tx.subscribe(),
        polling_tx.clone(),
        POLL_PERIOD,
    );

    let mut feeder = threads::Feeder::new(polling_tx.subscribe(), command_tx.subscribe());

    let mut ui_updater =
        threads::UIUpdater::new(polling_rx, command_rx, render_state_shared.clone());

    log::info!("Starting background threads...");

    let polling = tokio::spawn(async move { poller.run().unwrap() });
    let feeding = tokio::spawn(async move { feeder.run().await.unwrap() });
    let ui_updating = tokio::spawn(async move { ui_updater.run().await.unwrap() });

    log::info!("Background threads started");

    window::init(render_state_shared, command_tx.clone(), POLL_PERIOD)?;

    log::info!("Main window closed - shutting down...");

    log::info!("Waiting poller shutdown... ");
    polling.await?;
    log::info!("Waiting UI state shutdown...");
    ui_updating.await?;
    log::info!("Waiting feeder shutdown...");
    feeding.await?;

    log::info!("Shutdown successful");
    Ok(())
}

fn init_log() -> color_eyre::Result<()> {
    spdlog::init_log_crate_proxy()?;
    let formatter = Box::new(FullFormatter::builder().source_location(false).build());

    // Setting the new formatter for each sink of the default logger.
    for sink in spdlog::default_logger().sinks() {
        sink.set_formatter(formatter.clone());
    }

    Ok(())
}

mod channels;
mod feeder;
mod term_ui;
mod threads;
mod virtual_controller;
mod window;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use dualsense_tools::TiltEstimatorConfig;
use spdlog::formatter::FullFormatter;

use crate::term_ui::UiState;

/// In Hertz
const POLL_FREQUENCY: u8 = 120;
const POLL_PERIOD: Duration = Duration::from_nanos(1000000 / POLL_FREQUENCY as u64);

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_log()?;

    log::set_max_level(log::LevelFilter::Info);

    log::info!("Initializing...");

    let channels = channels::Channels::new();

    let mut poller = threads::Poller::new(
        TiltEstimatorConfig::<20>::default(),
        channels.commands.subscribe(),
        channels.polling.dispatch(),
        POLL_PERIOD,
    );

    let mut feeder =
        threads::Feeder::new(channels.polling.subscribe(), channels.commands.subscribe());

    let ui_state = Arc::new(Mutex::new(UiState::new()));
    let mut ui_updater = threads::UIUpdater::new(
        channels.polling.subscribe(),
        channels.commands.subscribe(),
        ui_state.clone(),
    );

    log::info!("Starting background threads...");

    let polling = tokio::spawn(async move { poller.run().unwrap() });
    let feeding = tokio::spawn(async move { feeder.run().await.unwrap() });
    let ui_updating = tokio::spawn(async move { ui_updater.run().await.unwrap() });

    log::info!("Background threads started, main UI is starting");

    window::init(ui_state, channels.commands.dispatch(), POLL_PERIOD)?;

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

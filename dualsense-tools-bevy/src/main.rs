mod spatial_visualizer;
mod term_ui;

use dualsense_tools::{Dualsense, TiltEstimator, TiltEstimatorConfig};
use hidapi::HidApi;
use std::sync::{Arc, Mutex};

fn main() -> color_eyre::Result<()> {
    color_eyre::install();
    let mut hid_api = HidApi::new()?;
    let ds = Arc::new(Mutex::new(Dualsense::new(&mut hid_api)?));
    let estimator = TiltEstimator::<5>::new(TiltEstimatorConfig::default());
    spatial_visualizer::scene(ds, estimator);
    Ok(())
}

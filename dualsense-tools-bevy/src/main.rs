mod plugin;
mod scene;

use dualsense_tools::{Dualsense};
use hidapi::HidApi;
use std::sync::{Arc, Mutex};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut hid_api = HidApi::new()?;
    let ds = Arc::new(Mutex::new(Dualsense::new(&mut hid_api)?));
    scene::scene::<10>(ds);
    Ok(())
}

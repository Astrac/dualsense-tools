use std::collections::HashSet;

use crate::virtual_controller::{AxisId, ButtonId};
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct FeederConfig {
    pub description: String,
    pub buttons: Vec<ButtonId>,
    pub axes: Vec<AxisId>,
    pub hat: bool,
    pub tilt_switch_trigger: HashSet<ButtonId>,
}

#[test]
fn test_deserialize_full_6_axes_and_throttle() -> color_eyre::Result<()> {
    let config_toml = include_bytes!("../../feeders/full_6_axes_and_throttle.toml");
    let config: FeederConfig = toml::from_slice(config_toml)?;

    assert_eq!(config.axes.len(), 7);
    assert_eq!(config.buttons.len(), 13);
    assert_eq!(
        config.tilt_switch_trigger.iter().collect::<Vec<_>>(),
        vec![&ButtonId::Mic]
    );
    assert_eq!(config.description, "Full emulation, 6-axis and throttle");

    Ok(())
}

#[test]
fn test_deserialize_builtin_ok() -> color_eyre::Result<()> {
    let config_files = std::fs::read_dir("../dualsense-tilt-and-throttle/feeders")?;
    for file in config_files {
        let file = file?;
        let content = std::fs::read(file.path())?;
        let _: FeederConfig = toml::from_slice(content.as_slice())?;
    }

    Ok(())
}

use crate::feeder::config::FeederConfig;

pub struct Feeders {
    feeders: Vec<FeederConfig>,
    selected: usize,
}

impl Feeders {
    pub fn new() -> Result<Feeders, toml::de::Error> {
        let feeders: Vec<FeederConfig> = vec![
            toml::from_slice(include_bytes!(
                "../../feeders/full_6_axes_and_throttle.toml"
            ))?,
            toml::from_slice(include_bytes!("../../feeders/full_8_axes.toml"))?,
            toml::from_slice(include_bytes!("../../feeders/roll_pitch.toml"))?,
            toml::from_slice(include_bytes!("../../feeders/roll_pitch_throttle.toml"))?,
        ];

        Ok(Feeders {
            feeders,
            selected: 0,
        })
    }

    pub fn next(&mut self) -> &FeederConfig {
        self.selected = (self.selected + 1) % self.feeders.len();
        &self.feeders[self.selected]
    }
}

use crate::{feeder::config::FeederConfig, virtual_controller::VirtualControllerState};

pub enum FeederId {
    Full6AxesAndThrottle,
    Full8Axes,
    RollAndPitch,
    RollPitchAndThrottle,
}

impl FeederId {
    fn get_config(&self) -> Result<FeederConfig, impl serde::de::Error> {
        match self {
            FeederId::Full6AxesAndThrottle => toml::from_slice(include_bytes!(
                "../../feeders/full_6_axes_and_throttle.toml"
            )),
            FeederId::Full8Axes => {
                toml::from_slice(include_bytes!("../../feeders/full_8_axes.toml"))
            }
            FeederId::RollAndPitch => {
                toml::from_slice(include_bytes!("../../feeders/roll_pitch.toml"))
            }
            FeederId::RollPitchAndThrottle => {
                toml::from_slice(include_bytes!("../../feeders/roll_pitch_throttle.toml"))
            }
        }
    }
}

pub const FEEDERS: [FeederId; 4] = [
    FeederId::Full6AxesAndThrottle,
    FeederId::Full8Axes,
    FeederId::RollAndPitch,
    FeederId::RollPitchAndThrottle,
];

struct FeederError;

trait Feeder {
    fn feed(&self, state: VirtualControllerState) -> Result<(), FeederError>;
}

struct ConfiguredFeeder {
    config: FeederConfig,
}

// impl Feeder for ConfiguredFeeder {
//     fn feed(&self, state: Controller) -> Result<(), FeederError> {
//         self.config
//         todo!()
//     }
// }

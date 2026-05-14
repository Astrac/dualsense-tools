mod config;
mod estimator;

pub use config::TiltEstimatorConfig;
pub use estimator::TiltEstimator;

use crate::Tilt;

/// Represents current estimates on the tilt status of a controller
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct TiltEstimates {
    /// Estimated only using the accelerometer average
    pub accel: Tilt,
    /// Instantaneus gyro orientation
    pub gyro: Tilt,
    /// Predicted tilt as determined by gyro integration on previous estimate
    pub integrated_gyro: Tilt,
    /// Predicted tilt using gyro+accelerometer fusion
    pub fused: Tilt,
}

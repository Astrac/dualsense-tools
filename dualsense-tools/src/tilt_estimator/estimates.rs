use crate::Tilt;

/// Represents current estimates on the tilt status of a controller
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct TiltEstimates {
    /// Estimated only using the accelerometer average
    pub accel_avg: Tilt,
    /// Estimated only using last accelerometer reading
    pub accel_instant: Tilt,
    /// Estimated only using last gyro reading
    pub gyro_instant: Tilt,
    /// Estimated tilt using gyro+accelerometer fusion
    pub accel_corrected_gyro: Tilt,
}

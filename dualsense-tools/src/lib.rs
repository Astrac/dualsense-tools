mod dualsense;
mod hid_report;
mod tilt;
mod tilt_estimator;

pub mod control_ids;
pub mod state;

pub use dualsense::Dualsense;
pub use tilt::{Radians, Tilt};
pub use tilt_estimator::{TiltEstimates, TiltEstimator, TiltEstimatorConfig};

/// Dualsense HID device vendor identifier
pub const VENDOR_ID: u16 = 1356;
/// Dualsense HID device product identifier
pub const PRODUCT_ID: u16 = 3302;
/// Minimum reading returned by accelerometer and gyro
pub const SENSORS_MIN: i16 = -8192;
/// Maximum reading returned by accelerometer and gyro
pub const SENSORS_MAX: i16 = 8191;
/// Range of possible reading returned by accelerometer and gyro
pub const SENSORS_RANGE: i16 = SENSORS_MAX - SENSORS_MIN;

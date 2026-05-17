/// Configuration for a [crate::TiltEstimator]; the `SAMPLES` constant will determine
/// how big a buffer of accelerometer readings to keep in order to compute
/// the accelerometer's average over time.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TiltEstimatorConfig<const SAMPLES: usize> {
    /// Value in [0..1] where 0 is purely accelerometer and 1 purely integrated gyro
    pub correction_alpha: f32,
    /// Used to scale the gyro force strenght when integrating gyro values
    pub integration_alpha: f32,
    /// Whether to integrate gyro values or only use accelerometer averages
    pub use_gyro_integration: bool,
}

impl<const SAMPLES: usize> TiltEstimatorConfig<SAMPLES> {
    pub fn new() -> TiltEstimatorConfig<SAMPLES> {
        TiltEstimatorConfig::<SAMPLES>::default()
    }

    pub fn with_correction_alpha(mut self, alpha: f32) -> TiltEstimatorConfig<SAMPLES> {
        self.correction_alpha = alpha;
        self
    }

    pub fn with_integration_dampening(mut self, dampening: f32) -> TiltEstimatorConfig<SAMPLES> {
        self.integration_alpha = dampening;
        self
    }

    pub fn with_use_gyro_integration(mut self, flag: bool) -> TiltEstimatorConfig<SAMPLES> {
        self.use_gyro_integration = flag;
        self
    }
}

impl<const N: usize> Default for TiltEstimatorConfig<N> {
    fn default() -> Self {
        Self {
            correction_alpha: 0.25,
            integration_alpha: 0.7,
            use_gyro_integration: true,
        }
    }
}

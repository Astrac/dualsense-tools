use crate::TiltEstimator;

#[derive(Clone, Debug, Default)]
pub struct DeterminatorConfig<const TILT_SAMPLES: usize> {
    pub tilt_estimator: Option<TiltEstimator<TILT_SAMPLES>>,
    pub gyro_events_enabled: bool,
    pub accel_events_enabled: bool,
}

impl<const TILT_SAMPLES: usize> DeterminatorConfig<TILT_SAMPLES> {
    pub fn new() -> DeterminatorConfig<TILT_SAMPLES> {
        DeterminatorConfig::<TILT_SAMPLES>::default()
    }
}

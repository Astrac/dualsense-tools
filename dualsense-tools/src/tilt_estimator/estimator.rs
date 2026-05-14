use std::time::Duration;

use circular_buffer::CircularBuffer;
use glam::{Quat, Vec3};

use crate::{
    AsTilt, Tilt, TiltEstimates,
    state::{Accel, DualsenseSensorValue, Gyro},
    tilt_estimator::TiltEstimatorConfig,
};

/// Estimates Tilt given sample readings; it keeps state pertaining the
/// previous readings and estimates to implement an algorighm similar
/// to the one described [here](https://stanford.edu/class/ee267/notes/ee267_notes_imu.pdf)
#[derive(Clone, Debug)]
pub struct TiltEstimator<const N: usize> {
    accel_samples: CircularBuffer<N, Vec3>,
    accel_samples_sum: Vec3,
    current: TiltEstimates,
    config: TiltEstimatorConfig<N>,
}

impl<const N: usize> TiltEstimator<N> {
    /// Creates a new estimator - see [crate::TiltEstimatorConfig]
    pub fn new(config: TiltEstimatorConfig<N>) -> TiltEstimator<N> {
        TiltEstimator {
            accel_samples: CircularBuffer::new(),
            accel_samples_sum: Vec3::ZERO,
            current: TiltEstimates::default(),
            config,
        }
    }

    /// Returns the lastly computed estimates
    pub fn current(&self) -> TiltEstimates {
        self.current
    }

    /// Computes the next estimates and updates the internal state
    pub fn next_estimate(
        &mut self,
        accel: &Accel<DualsenseSensorValue>,
        gyro: &Gyro<DualsenseSensorValue>,
        delta_t: &Duration,
    ) -> TiltEstimates {
        let accel_vec = accel.raw_vec();
        self.accel_samples_sum += accel_vec;

        if let Some(outdated) = self.accel_samples.push_back(accel_vec) { self.accel_samples_sum -= outdated }

        let accel_avg =
            Accel::<f32>::from_vec(self.accel_samples_sum / (self.accel_samples.len() as f32));

        let accel_tilt = accel_avg.as_tilt();

        let gyro_norm: Tilt;
        let gyro_quat: Quat;
        let fused_tilt: Tilt;

        if self.config.use_gyro_integration {
            let gyro_length = gyro.raw_vec().length();
            gyro_norm = gyro.as_tilt();

            let gyro_dquat = Quat::from_axis_angle(
                gyro_norm.quat.to_scaled_axis().normalize_or_zero(),
                f32::to_radians(gyro_length)
                    * delta_t.as_secs_f32()
                    * self.config.integration_dampening,
            );

            gyro_quat = self.current.fused.quat.mul_quat(gyro_dquat);

            fused_tilt = Tilt::new(
                accel_tilt
                    .quat
                    .slerp(gyro_quat, self.config.correction_alpha),
            );
        } else {
            gyro_norm = Tilt::default();
            gyro_quat = Quat::from_xyzw(1., 0., 0., 0.);
            fused_tilt = accel_tilt;
        }

        self.current = TiltEstimates {
            accel: accel_tilt,
            gyro: gyro_norm,
            integrated_gyro: Tilt::new(gyro_quat),
            fused: fused_tilt,
        };

        self.current
    }
}

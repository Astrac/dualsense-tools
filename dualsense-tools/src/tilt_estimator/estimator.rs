use std::f32::consts::PI;

use glam::{Quat, Vec3};

use crate::{
    StateEvent, Tilt, TiltEstimates,
    state::{Accel, Gyro},
    tilt_estimator::TiltEstimatorConfig,
};

/// Estimates Tilt given sample readings; it keeps state pertaining the
/// previous readings and estimates to implement an algorighm similar
/// to the one described [here](https://stanford.edu/class/ee267/notes/ee267_notes_imu.pdf)
///
/// The frame of reference of the rotations returned should be such that at rest the
/// y axis points up, the x axis to the right and the z axis follows the
/// right-hand rule (i.e. points towards an observer looking at the xy plane
/// from above).
#[derive(Clone, Debug)]
pub struct TiltEstimator<const N: usize> {
    accel_samples_sum: Vec3,
    current: TiltEstimates,
    config: TiltEstimatorConfig<N>,
}

impl<const N: usize> TiltEstimator<N> {
    /// Creates a new estimator - see [crate::TiltEstimatorConfig]
    pub fn new(config: TiltEstimatorConfig<N>) -> TiltEstimator<N> {
        TiltEstimator {
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
    pub fn next_estimate(&mut self, event: StateEvent) -> TiltEstimates {
        let accel = event.current.value.accel;
        let gyro = event.current.value.gyro;

        let accel_vec = accel.as_vec3();
        self.accel_samples_sum += accel_vec;
        self.accel_samples_sum -= event.evicted.value.accel.as_vec3();

        let accel_avg = Accel::<f32>::from_vec(self.accel_samples_sum / (N as f32));

        let accel_tilt_quat = normalized_accel_quat(&accel_avg);

        let fused_tilt: Tilt;
        let gyro_norm_quat: Quat;

        if self.config.use_gyro_integration {
            let gyro_length = gyro.as_vec3().length();
            gyro_norm_quat = normalized_gyro_quat(&gyro);

            let gyro_dquat = Quat::from_axis_angle(
                gyro_norm_quat.to_scaled_axis().normalize_or_zero(),
                f32::to_radians(gyro_length) * event.elapsed_time().as_secs_f32(),
            );

            let gyro_quat = self.current.accel_corrected_gyro.quat.mul_quat(gyro_dquat);

            fused_tilt = Tilt::new(accel_tilt_quat.slerp(gyro_quat, self.config.correction_alpha));
        } else {
            fused_tilt = Tilt::new(accel_tilt_quat);
            gyro_norm_quat = Quat::default();
        }

        self.current = TiltEstimates {
            accel_avg: Tilt::new(accel_tilt_quat),
            accel_instant: Tilt::new(normalized_accel_quat(&accel)),
            gyro_instant: Tilt::new(gyro_norm_quat),
            accel_corrected_gyro: fused_tilt,
        };

        self.current
    }
}

const GRAVITY: Vec3 = Vec3::new(0., -1., 0.);

/// Returns the quaternion representing the rotation of the gyroscope unit-vector.
fn normalized_gyro_quat<T: Into<f32> + Copy>(gyro: &Gyro<T>) -> Quat {
    Quat::from_rotation_arc(
        GRAVITY,
        Vec3::new(gyro.z.into(), gyro.y.into(), -gyro.x.into()).normalize_or_zero(),
    )
}

/// Returns the quaternion representing the rotation of the accelerometer unit-vector.
fn normalized_accel_quat<T: Into<f32> + Copy>(accel: &Accel<T>) -> Quat {
    Quat::from_rotation_arc(
        GRAVITY,
        Vec3::new(-accel.x.into(), accel.y.into(), accel.z.into())
            .normalize_or_zero()
            .rotate_z(PI),
    )
}

#[test]
fn test_normalized_gyro_quat() {
    let examples: [(Gyro<f32>, Quat); 4] = [
        (Gyro::new(0., 0., 0.), Quat::default()),
        (
            Gyro::new(10., 0., 0.),
            Quat::from_axis_angle(Vec3::X, PI / 2.),
        ),
        (Gyro::new(0., 10., 0.), Quat::from_axis_angle(Vec3::Z, PI)),
        (
            Gyro::new(0., 0., 10.),
            Quat::from_axis_angle(Vec3::Z, PI / 2.),
        ),
    ];

    for (gyro, expectation) in examples {
        assert_eq!(
            normalized_gyro_quat(&gyro).to_axis_angle(),
            expectation.to_axis_angle(),
            "Failed with: {gyro:?}"
        )
    }
}

#[test]
fn test_normalized_accel_quat() {
    let examples: [(Accel<f32>, Quat); 4] = [
        (Accel::new(0., 0., 0.), Quat::default()),
        (
            Accel::new(10., 0., 0.),
            Quat::from_axis_angle(Vec3::Z, PI / 2.),
        ),
        (Accel::new(0., 10., 0.), Quat::from_axis_angle(Vec3::X, 0.)),
        (
            Accel::new(0., 0., 10.),
            Quat::from_axis_angle(-Vec3::X, PI / 2.),
        ),
    ];

    for (accel, expectation) in examples {
        assert_eq!(
            normalized_accel_quat(&accel).to_axis_angle(),
            expectation.to_axis_angle(),
            "Failed with: {accel:?}"
        )
    }
}

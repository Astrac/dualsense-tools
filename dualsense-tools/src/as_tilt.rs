use crate::{
    Tilt,
    state::{Accel, Gyro},
};
use glam::{Quat, Vec3};
use std::f32::consts::PI;

const GRAVITY: Vec3 = Vec3::new(0., -1., 0.);

/// The frame of reference of the quaternion returned should be such that the
/// y axis points up, the x axis to the right and the z axis follows the
/// right-hand rule (i.e. points towards an observer looking at the xy plane
/// from above).
pub trait AsTilt {
    fn as_tilt(&self) -> Tilt;
}

impl<T> AsTilt for Accel<T>
where
    T: Into<f32> + Copy,
{
    fn as_tilt(&self) -> Tilt {
        Tilt::new(Quat::from_rotation_arc(
            GRAVITY,
            Vec3::new(-self.x.into(), self.y.into(), self.z.into())
                .normalize_or_zero()
                .rotate_z(PI),
        ))
    }
}

impl<T> AsTilt for Gyro<T>
where
    T: Into<f32> + Copy,
{
    fn as_tilt(&self) -> Tilt {
        Tilt::new(Quat::from_rotation_arc(
            GRAVITY,
            Vec3::new(self.z.into(), self.y.into(), -self.x.into()).normalize_or_zero(),
        ))
    }
}

use std::ops::{Deref, DerefMut};

use glam::Vec3;

use crate::state::SpatialSensor;

/// Represents readings from an accelerometer
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct Accel<V>(SpatialSensor<V>);

impl<T> Accel<T> {
    /// Instantiate a new Accel from individual components
    pub fn new(x: T, y: T, z: T) -> Accel<T> {
        Accel(SpatialSensor { x, y, z })
    }

    /// Instantiate a new Accel from a glam Vec3
    pub fn from_vec(v: Vec3) -> Accel<T>
    where
        T: From<f32>,
    {
        Accel(SpatialSensor {
            x: v.x.into(),
            y: v.y.into(),
            z: v.z.into(),
        })
    }
}

impl<V> Deref for Accel<V> {
    type Target = SpatialSensor<V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V> DerefMut for Accel<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

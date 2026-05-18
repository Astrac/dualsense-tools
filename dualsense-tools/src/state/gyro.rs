use crate::state::SpatialSensor;
use std::ops::{Deref, DerefMut};

/// Represents readings from a gyroscope
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct Gyro<V>(SpatialSensor<V>);

impl<V> Deref for Gyro<V> {
    type Target = SpatialSensor<V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V> DerefMut for Gyro<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

use glam::Vec3;

/// Models the readings from a 3d spatial sensor (accelerometer, gyro).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct SpatialSensor<V> {
    pub x: V,
    pub y: V,
    pub z: V,
}

impl<V> SpatialSensor<V>
where
    V: Into<f32> + Copy,
{
    /// Reads the values as a glam Vec3
    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x.into(), self.y.into(), self.z.into())
    }
}

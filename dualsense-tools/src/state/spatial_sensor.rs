use glam::Vec3;

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
    pub fn raw_vec(&self) -> Vec3 {
        Vec3::new(self.x.into(), self.y.into(), self.z.into())
    }
}

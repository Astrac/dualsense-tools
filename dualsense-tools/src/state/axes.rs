#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct DualsenseAxes<V> {
    pub x: V,
    pub y: V,
    pub z: V,
    pub rx: V,
    pub ry: V,
    pub rz: V,
}

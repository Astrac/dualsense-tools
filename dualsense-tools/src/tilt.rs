use glam::{Quat, Vec3};

/// Describe the tilt of a controller, i.e. its orientation in terms
/// of roll (rotation around the Z axis) and pitch (rotation around the X axis).
/// See [crate::AsTilt] for more details.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Tilt {
    pub quat: Quat,
}

impl Tilt {
    pub const fn new(quat: Quat) -> Tilt {
        Tilt { quat }
    }

    pub fn get_roll_radians(&self) -> Radians {
        Radians(-self.quat.to_scaled_axis().dot(Vec3::Z))
    }

    pub fn get_pitch_radians(&self) -> Radians {
        Radians(self.quat.to_scaled_axis().dot(Vec3::X))
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Radians(f32);

impl Radians {
    pub fn get_angle(&self) -> f32 {
        self.0
    }
}

impl From<f32> for Radians {
    fn from(val: f32) -> Self {
        Radians(val)
    }
}

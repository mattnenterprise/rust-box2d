use super::Vec2;

/// Rotation
#[derive(Copy, Clone)]
pub struct Rot {
    pub s: f32,
    pub c: f32
}

impl Rot {
    pub fn new() -> Rot {
        Rot {
            s: 0.0,
            c: 1.0
        }
    }

    /// Initialize from an angle in radians
    pub fn new_angle(angle: f32) -> Rot {
        Rot {
            s: angle.sin(),
            c: angle.cos()
        }
    }

    pub fn set(&mut self, angle: f32) {
        self.s = angle.sin();
        self.c = angle.cos();
    }

    /// Set to the identity rotation
    pub fn set_identity(&mut self) {
        self.s = 0.0;
        self.c = 1.0;
    }

    /// Get the angle in radians
    pub fn get_angle(&mut self) -> f32 {
        self.s.atan2(self.c)
    }

    /// Get the x-axis
    pub fn get_x_axis(&mut self) -> Vec2 {
        Vec2::new(self.c, self.s)
    }

    /// Get the u axis
    pub fn get_y_axis(&mut self) -> Vec2 {
        Vec2::new(-self.s, self.c)
    }
}

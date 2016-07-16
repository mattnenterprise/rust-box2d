use super::Vec2;
use super::Vec3;

/// A 3-by-3 matrix. Stored in column-major order.
pub struct Mat33 {
    pub ex: Vec2,
	pub ey: Vec2,
    pub ez: Vec2
}

impl Mat33 {
    pub fn new_columns(c1: Vec2, c2: Vec2, c3: Vec2) -> Mat33 {
        Mat33 {
            ex: c1,
            ey: c2,
            ez: c3
        }
    }

    pub fn set_zero(&mut self) {
        self.ex.set_zero();
        self.ey.set_zero();
        self.ez.set_zero();
    }
}

use super::Vec2;
use super::Rot;

/// A transform contains translation and rotation. It is used to represent
/// the position and orientation of rigid frames.
#[derive(Copy, Clone)]
pub struct Transform {
    pub p: Vec2,
    pub q: Rot
}

impl Transform {
    pub fn new(position: Vec2, rotation: Rot) -> Transform {
        return Transform {
            p: position,
            q: rotation
        }
    }

    /// Set this to the identity transform.
    pub fn set_identity(&mut self) {
        self.p.set_zero();
        self.q.set_identity();
    }

    /// Set this based on the position and angle.
    pub fn set(&mut self, position: Vec2, angle: f32) {
        self.p = position;
        self.q.set(angle);
    }
}

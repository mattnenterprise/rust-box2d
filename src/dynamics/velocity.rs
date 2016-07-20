use super::super::common::Vec2;

pub struct Velocity {
    pub v: Vec2,
    pub w: f32
}

impl Velocity {
    pub fn new() -> Velocity {
        Velocity {
            v: Vec2::zero(),
            w: 0.0
        }
    }
}

use super::super::common::Vec2;

pub struct RayCastInput {
    pub p1: Vec2,
    pub p2: Vec2,
    pub max_fraction: f32
}

impl RayCastInput {
    pub fn new() -> RayCastInput {
        RayCastInput {
            p1: Vec2::new(0.0, 0.0),
            p2: Vec2::new(0.0, 0.0),
            max_fraction: 0.0
        }
    }
}

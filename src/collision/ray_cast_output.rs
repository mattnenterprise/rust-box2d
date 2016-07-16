use super::super::common::Vec2;

pub struct RayCastOutput {
    pub normal: Vec2,
    pub fraction: f32
}

impl RayCastOutput {
    pub fn new() -> RayCastOutput {
        RayCastOutput {
            normal: Vec2::new(0.0, 0.0),
            fraction: 0.0
        }
    }
}

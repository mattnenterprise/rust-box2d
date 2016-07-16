use super::super::common::Vec2;

/// Ray-cast output data. The ray hits at p1 + fraction * (p2 - p1), where p1 and p2
/// come from b2RayCastInput.
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

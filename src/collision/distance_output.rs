use super::super::common::Vec2;

/// Output for Distance
pub struct DistanceOutput {
    pub point_a: Vec2,
    pub point_b: Vec2,
    pub distance: f32,
    pub iterations: i32
}

impl DistanceOutput {
    pub fn new() -> DistanceOutput {
        DistanceOutput {
            point_a: Vec2::zero(),
            point_b: Vec2::zero(),
            distance: 0.0,
            iterations: 0
        }
    }
}

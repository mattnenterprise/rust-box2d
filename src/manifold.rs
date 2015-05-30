use super::math::Vec2;

pub struct Manifold {
    pub normal: Vec2,
    pub penetration: f32
}

impl Manifold {
    pub fn new(normal: Vec2, penetration: f32) -> Manifold {
        return Manifold{ normal: normal, penetration: penetration };
    }
}

use super::body::Body;
use super::common::math::Vec2;

#[derive(Clone)]
pub struct Manifold {
    pub body_a: Body,
    pub body_b: Body,
    pub normal: Vec2,
    pub penetration: f32
}

impl Manifold {
    pub fn new(body_a: Body, body_b: Body, normal: Vec2, penetration: f32) -> Manifold {
        return Manifold{ body_a: body_a, body_b: body_b, normal: normal, penetration: penetration };
    }
}

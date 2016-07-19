use super::super::common::Vec2;

pub struct SimplexVertex {
    /// Support point for proxy_a
    pub w_a: Vec2,
    /// Support point for proxy_b
    pub w_b: Vec2,
    /// w_b - w_a
    pub w: Vec2,
    /// barycentric coordinate for closest point
    pub a: f32,
    /// w_a index
    pub index_a: i32,
    /// w_b index
    pub index_b: i32
}

impl SimplexVertex {
    pub fn new() -> SimplexVertex {
        SimplexVertex {
            w_a: Vec2::zero(),
            w_b: Vec2::zero(),
            w: Vec2::zero(),
            a: 0.0,
            index_a: 0,
            index_b: 0
        }
    }
}

use super::body::Body;

pub struct Manifold {
    pub body_a: Body,
    pub body_b: Body
}

impl Manifold {
    pub fn new(body_a: Body, body_b: Body) -> Manifold {
        return Manifold{ body_a: body_a, body_b: body_b};
    }
}

use super::body::Body;

#[derive(Clone)]
pub struct BodyPair {
    pub body_a: Body,
    pub body_b: Body
}

impl BodyPair {
    fn new(body_a: Body, body_b: Body) -> BodyPair {
        return BodyPair{ body_a: body_a, body_b: body_b }
    }
}

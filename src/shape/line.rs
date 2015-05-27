use super::super::math::Vec2;
use super::shape::Shape;

pub struct LineShape {
    p1: Vec2,
    p2: Vec2
}

impl Shape for LineShape {
    fn getType() -> &'static str {
        return "LineShape";
    }
}

impl LineShape {
    pub fn new(p1: Vec2, p2: Vec2) -> LineShape {
        return LineShape{ p1: p1, p2: p2 }
    }
}

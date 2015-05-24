use super::super::math::Vec2;
use super::shape::Shape;

pub struct CircleShape {
    center: Vec2,
    radius: f32
}

impl Shape for CircleShape {
    fn getType() -> &'static str {
        return "CircleShape";
    }
}
